//! MCP JSON-RPC 2.0 client over stdio.
//!
//! Spawns an MCP-compliant server process and communicates via
//! newline-delimited JSON-RPC 2.0 on stdin/stdout.
//!
//! ## Protocol flow
//! 1. `initialize`  → server replies with capabilities + server_info
//! 2. `initialized` → client sends notification (no response)
//! 3. `tools/list`  → discover available tools
//! 4. `tools/call`  → invoke a tool by name

use std::fmt;
use std::process::Stdio;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::time::timeout;

// ── Default timeout for RPC calls ──────────────────────────────────
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

// ── JSON-RPC primitives ────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    #[allow(dead_code)]
    jsonrpc: String,
    #[serde(default)]
    id: Option<u64>,
    #[serde(default)]
    result: Option<Value>,
    #[serde(default)]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i64,
    message: String,
    #[allow(dead_code)]
    #[serde(default)]
    data: Option<Value>,
}

// ── MCP domain types ───────────────────────────────────────────────

/// Result of the `initialize` handshake.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub server_info: ServerInfo,
}

/// Server capabilities advertised during initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(default)]
    pub tools: Option<ToolsCapability>,
    #[serde(default)]
    pub resources: Option<ResourcesCapability>,
    #[serde(default)]
    pub prompts: Option<PromptsCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    #[serde(default)]
    pub list_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesCapability {
    #[serde(default)]
    pub subscribe: bool,
    #[serde(default)]
    pub list_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsCapability {
    #[serde(default)]
    pub list_changed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// A tool as returned by `tools/list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub input_schema: Value,
}

/// Result of a `tools/call` invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: Vec<Content>,
    #[serde(default)]
    pub is_error: bool,
}

/// A piece of content within a tool result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: ContentType,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    Image,
    Resource,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Text => write!(f, "text"),
            ContentType::Image => write!(f, "image"),
            ContentType::Resource => write!(f, "resource"),
        }
    }
}

// ── McpClient ──────────────────────────────────────────────────────

/// An MCP client connected to a child process via stdio JSON-RPC 2.0.
pub struct McpClient {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
    next_id: u64,
    initialized: bool,
    /// Human-readable label for logging / error messages.
    label: String,
}

impl McpClient {
    /// Spawn `command` with `args` and return a ready-to-initialize client.
    ///
    /// The process **must** speak MCP JSON-RPC 2.0 over stdin/stdout.
    pub fn new<S: AsRef<str>>(command: S, args: &[S]) -> Result<Self> {
        let cmd_str = command.as_ref().to_string();
        let arg_strs: Vec<String> = args.iter().map(|a| a.as_ref().to_string()).collect();

        let mut child = Command::new(&cmd_str)
            .args(&arg_strs)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .with_context(|| {
                format!(
                    "failed to spawn MCP server process: `{} {}`",
                    cmd_str,
                    arg_strs.join(" ")
                )
            })?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("child stdin is None"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("child stdout is None"))?;

        Ok(Self {
            label: format!("{} {}", cmd_str, arg_strs.join(" ")),
            child,
            stdin,
            reader: BufReader::new(stdout),
            next_id: 1,
            initialized: false,
        })
    }

    /// Perform the MCP `initialize` → `initialized` handshake.
    ///
    /// After this call the client is ready to list and call tools.
    pub async fn initialize(&mut self) -> Result<InitializeResult> {
        let params = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "clientInfo": {
                "name": "hermes-rust-operit",
                "version": env!("CARGO_PKG_VERSION")
            }
        });

        let raw = self.rpc_call("initialize", Some(params)).await?;
        let result: InitializeResult =
            serde_json::from_value(raw).context("malformed initialize response")?;

        // Send `initialized` notification (no id, no response expected).
        self.send_notification("notifications/initialized", None)
            .await?;

        self.initialized = true;
        Ok(result)
    }

    /// Discover tools exposed by the server.
    pub async fn list_tools(&mut self) -> Result<Vec<ToolDef>> {
        self.ensure_initialized()?;

        let raw = self.rpc_call("tools/list", None).await?;
        let tools: Vec<ToolDef> = raw
            .get("tools")
            .cloned()
            .unwrap_or_default()
            .as_array()
            .cloned()
            .map(|arr| serde_json::from_value(Value::Array(arr)).unwrap_or_default())
            .unwrap_or_default();

        Ok(tools)
    }

    /// Call a tool by name with the given arguments (serde_json::Value).
    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String> {
        self.ensure_initialized()?;

        let params = serde_json::json!({
            "name": name,
            "arguments": arguments,
        });

        let raw = self.rpc_call("tools/call", Some(params)).await?;

        // Try to extract text content for convenience; fall back to raw JSON.
        if let Some(content) = raw.get("content").and_then(|c| c.as_array()) {
            let texts: Vec<String> = content
                .iter()
                .filter_map(|c| c.get("text").and_then(|t| t.as_str()).map(String::from))
                .collect();
            Ok(texts.join("\n"))
        } else {
            Ok(raw.to_string())
        }
    }

    /// Gracefully shut down the server process.
    pub async fn shutdown(&mut self) -> Result<()> {
        // Best-effort notification; don't fail if the process is already gone.
        let _ = self.send_notification("shutdown", None).await;

        // Give the process a moment, then kill.
        let _ = tokio::time::sleep(Duration::from_millis(200)).await;
        let _ = self.child.start_kill();
        let _ = self.child.wait().await;
        Ok(())
    }

    // ── private helpers ──────────────────────────────────────────

    fn ensure_initialized(&self) -> Result<()> {
        if !self.initialized {
            bail!("MCP client not initialized — call initialize() first");
        }
        Ok(())
    }

    /// Send a JSON-RPC request and wait for the matching response.
    async fn rpc_call(&mut self, method: &str, params: Option<Value>) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;

        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params,
        };

        let mut body = serde_json::to_vec(&req)?;
        body.push(b'\n');
        self.stdin.write_all(&body).await?;
        self.stdin.flush().await?;

        let resp = timeout(DEFAULT_TIMEOUT, self.read_response(id))
            .await
            .map_err(|_| {
                anyhow!(
                    "MCP call '{}' timed out after {:?}",
                    method,
                    DEFAULT_TIMEOUT
                )
            })??;

        if let Some(err) = resp.error {
            bail!(
                "MCP error '{}': {} (code: {})",
                method,
                err.message,
                err.code
            );
        }

        resp.result
            .ok_or_else(|| anyhow!("MCP call '{}' returned null result", method))
    }

    /// Send a JSON-RPC notification (no `id`, no response expected).
    async fn send_notification(&mut self, method: &str, params: Option<Value>) -> Result<()> {
        #[derive(Serialize)]
        struct Notification {
            jsonrpc: String,
            method: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            params: Option<Value>,
        }

        let notif = Notification {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params,
        };

        let mut body = serde_json::to_vec(&notif)?;
        body.push(b'\n');
        self.stdin.write_all(&body).await?;
        self.stdin.flush().await?;
        Ok(())
    }

    /// Read lines until we get a JSON-RPC response whose `id` matches.
    async fn read_response(&mut self, expected_id: u64) -> Result<JsonRpcResponse> {
        let mut line = String::new();
        loop {
            line.clear();
            let n = self
                .reader
                .read_line(&mut line)
                .await
                .context("reading from MCP server stdout")?;
            if n == 0 {
                bail!("MCP server process closed stdout unexpectedly");
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            match serde_json::from_str::<JsonRpcResponse>(trimmed) {
                Ok(resp) => {
                    // Notifications have no id — skip them.
                    if resp.id == Some(expected_id) {
                        return Ok(resp);
                    }
                    // Non-matching id: keep reading.
                }
                Err(_) => {
                    // Could be a server log line on stdout — skip.
                    continue;
                }
            }
        }
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        // Best-effort kill on drop.
        let _ = self.child.start_kill();
    }
}

// ── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: 1,
            method: "initialize".into(),
            params: Some(serde_json::json!({"protocolVersion": "2024-11-05"})),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"initialize\""));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_deserialize_initialize_result() {
        let json = r#"{
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "test-server",
                "version": "1.0.0"
            }
        }"#;
        let result: InitializeResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.protocol_version, "2024-11-05");
        assert_eq!(result.server_info.name, "test-server");
        assert!(result.capabilities.tools.is_some());
    }

    #[test]
    fn test_deserialize_tool_def() {
        let json = r#"{
            "name": "read_file",
            "description": "Read a file",
            "inputSchema": {"type": "object", "properties": {}}
        }"#;
        let tool: ToolDef = serde_json::from_str(json).unwrap();
        assert_eq!(tool.name, "read_file");
        assert_eq!(tool.description, "Read a file");
    }

    #[test]
    fn test_deserialize_tool_call_result() {
        let json = r#"{
            "content": [{"type": "text", "text": "hello world"}],
            "isError": false
        }"#;
        let result: ToolCallResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.content[0].text, "hello world");
        assert!(!result.is_error);
    }
}
