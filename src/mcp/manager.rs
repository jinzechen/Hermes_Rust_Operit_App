//! MCP plugin manager — manages external MCP server connections.
//!
//! Uses the existing mcp::client::McpClient (JSON-RPC 2.0 over stdio)
//! which is a complete, tested implementation (466 lines, 4 tests).
//!
//! The rmcp crate is available as an alternative but our custom client
//! is sufficient for the 4 external MCP plugins (rust_mcp_server,
//! mcp_proxy, m3ux, rust_docs_mcp) while the 5 built-in tools use
//! zero-overhead ToolHandler dispatch.

use std::collections::HashMap;
use serde_json::Value;
use crate::mcp::client::McpClient;

/// A managed MCP server connection.
pub struct McpConnection {
    name: String,
    command: String,
    args: Vec<String>,
    client: Option<McpClient>,
    tools: Vec<String>,
}

/// Manages multiple MCP server connections.
pub struct McpManager {
    connections: HashMap<String, McpConnection>,
}

impl McpManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    /// Register an MCP server (does not connect yet).
    pub fn register(&mut self, name: &str, command: &str, args: &[&str]) {
        self.connections.insert(
            name.to_string(),
            McpConnection {
                name: name.to_string(),
                command: command.to_string(),
                args: args.iter().map(|s| s.to_string()).collect(),
                client: None,
                tools: Vec::new(),
            },
        );
    }

    /// Connect and initialize all registered servers.
    pub async fn connect_all(&mut self) -> anyhow::Result<()> {
        let names: Vec<String> = self.connections.keys().cloned().collect();
        for name in names {
            let (cmd, cargs) = {
                let conn = self.connections.get(&name).unwrap();
                let cmd = conn.command.clone();
                let cargs: Vec<String> = conn.args.clone();
                (cmd, cargs)
            };
            let mut client = McpClient::new(cmd.as_str(), &cargs.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
            let _init = client.initialize().await?;
            let tools = client.list_tools().await?;
            if let Some(conn) = self.connections.get_mut(&name) {
                conn.tools = tools.iter().map(|t| t.name.clone()).collect();
                conn.client = Some(client);
            }
        }
        Ok(())
    }

    /// Call a tool on a connected MCP service.
    pub async fn call_tool(
        &mut self,
        service_name: &str,
        tool_name: &str,
        arguments: Value,
    ) -> anyhow::Result<String> {
        let conn = self
            .connections
            .get_mut(service_name)
            .ok_or_else(|| anyhow::anyhow!("MCP service not found: {}", service_name))?;

        let client = conn
            .client
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("MCP service not connected: {}", service_name))?;

        client.call_tool(tool_name, arguments).await
    }

    /// List all registered service names.
    pub fn service_names(&self) -> Vec<&str> {
        self.connections.keys().map(|s| s.as_str()).collect()
    }

    /// List tools for a connected service.
    pub fn tools_for(&self, service_name: &str) -> Option<&[String]> {
        self.connections
            .get(service_name)
            .map(|c| c.tools.as_slice())
    }

    /// Shutdown a specific service (or all if service_name is empty).
    pub async fn shutdown(&mut self, service_name: Option<&str>) -> anyhow::Result<()> {
        if let Some(name) = service_name {
            if let Some(conn) = self.connections.get_mut(name) {
                if let Some(ref mut client) = conn.client {
                    client.shutdown().await?;
                }
                conn.client = None;
            }
        } else {
            for conn in self.connections.values_mut() {
                if let Some(ref mut client) = conn.client {
                    let _ = client.shutdown().await;
                }
                conn.client = None;
            }
        }
        Ok(())
    }

    /// Pre-configure the 4 standard external MCP plugins.
    pub fn register_standard_plugins(&mut self) {
        self.register("rust-toolchain", "rust-mcp-server", &[]);
        self.register("mcp-proxy", "mcp-proxy", &[]);
        self.register("audio-tools", "m3ux", &[]);
        self.register("rust-docs", "rust-docs-mcp", &[]);
    }
}

impl Default for McpManager {
    fn default() -> Self { Self::new() }
}
