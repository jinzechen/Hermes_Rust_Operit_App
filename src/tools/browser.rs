//! Browser tool — integrated with obscura CDP headless browser.
//!
//! Read operations (navigate, get_html) use reqwest+scraper for speed.
//! Interactive operations (screenshot, click, type_text, evaluate)
//! use the obscura binary via MCP (JSON-RPC 2.0 over stdio).
//!
//! The obscura binary is expected at `bin/obscura.exe` (downloaded by scripts/setup.sh).

use anyhow::{anyhow, bail, Context};
use scraper::{Html, Selector};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

/// Locate the obscura binary.
fn obscura_path() -> PathBuf {
    // Check relative to project root, then PATH
    for candidate in &["bin/obscura.exe", "bin/obscura", "../bin/obscura.exe"] {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return p;
        }
    }
    PathBuf::from("obscura") // fallback to PATH
}

pub struct BrowserTool {
    current_url: Mutex<String>,
    current_html: Mutex<String>,
}

impl BrowserTool {
    pub fn new() -> Self {
        Self {
            current_url: Mutex::new(String::new()),
            current_html: Mutex::new(String::new()),
        }
    }

    /// Spawn obscura in MCP mode and call a tool.
    fn call_obscura_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<String> {
        let bin = obscura_path();
        if !bin.exists() {
            bail!(
                "obscura binary not found at {}. Run: bash scripts/setup.sh",
                bin.display()
            );
        }

        // Use our existing MCP client to talk to obscura
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let mut client = crate::mcp::client::McpClient::new(bin.to_str().unwrap(), &["mcp"])?;
            let _init = client.initialize().await?;
            client.call_tool(tool_name, args).await
        })
    }
}

impl Default for BrowserTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolHandler for BrowserTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "browser".into(),
            description:
                "obscura CDP headless browser — navigate, screenshot, click, type, evaluate JS"
                    .into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["navigate", "screenshot", "get_html", "click", "type_text", "evaluate"]
                    },
                    "url": { "type": "string" },
                    "selector": { "type": "string" },
                    "text": { "type": "string" },
                    "javascript": { "type": "string", "description": "JS code for evaluate" },
                },
                "required": ["action"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        match action {
            "navigate" => self.op_navigate(&arguments),
            "get_html" => self.op_get_html(&arguments),
            "screenshot" => self.op_screenshot(&arguments),
            "click" => self.op_click(&arguments),
            "type_text" => self.op_type_text(&arguments),
            "evaluate" => self.op_evaluate(&arguments),
            _ => bail!("unknown browser action: '{}'", action),
        }
    }
}

// ── read operations (fast, no obscura needed) ──────────────────────

impl BrowserTool {
    fn op_navigate(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing 'url' argument"))?;

        let client = reqwest::blocking::Client::builder()
            .user_agent("HermesAgent/1.0")
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let resp = client
            .get(url)
            .send()
            .with_context(|| format!("HTTP GET failed: {}", url))?;

        let status = resp.status();
        let final_url = resp.url().to_string();
        let body = resp.text()?;

        if let Ok(mut u) = self.current_url.lock() {
            *u = final_url.clone();
        }
        if let Ok(mut h) = self.current_html.lock() {
            *h = body.clone();
        }

        let preview: String = body.chars().take(4000).collect();
        let suffix = if body.len() > 4000 { "…" } else { "" };

        Ok(format!(
            "## Navigate\n\nURL: {}\nStatus: {}\n\n```html\n{}{}\n```\n\n**{} bytes**",
            final_url,
            status.as_u16(),
            preview,
            suffix,
            body.len()
        ))
    }

    fn op_get_html(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let selector_str = args
            .get("selector")
            .and_then(|v| v.as_str())
            .unwrap_or("body");
        let html = self
            .current_html
            .lock()
            .map_err(|e| anyhow!("lock: {}", e))?;
        if html.is_empty() {
            bail!("no page loaded — call navigate first");
        }

        let doc = Html::parse_document(&html);
        let sel = Selector::parse(selector_str)
            .map_err(|e| anyhow!("invalid CSS selector '{}': {:?}", selector_str, e))?;

        let results: Vec<String> = doc
            .select(&sel)
            .map(|e| e.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(format!(
            "## Get HTML\n\nSelector: {}\nMatches: {}\n\n{}",
            selector_str,
            results.len(),
            if results.is_empty() {
                "(no matches)".into()
            } else {
                results.join("\n---\n")
            }
        ))
    }
}

// ── interactive operations (require obscura binary) ─────────────────

impl BrowserTool {
    fn op_screenshot(&self, _args: &serde_json::Value) -> anyhow::Result<String> {
        match self.call_obscura_tool("browser_screenshot", serde_json::json!({})) {
            Ok(result) => Ok(result),
            Err(e) => Ok(format!(
                "## Screenshot\n\n⚠ obscura unavailable: {}\n\nRun: bash scripts/setup.sh",
                e
            )),
        }
    }

    fn op_click(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let selector = args
            .get("selector")
            .and_then(|v| v.as_str())
            .unwrap_or("body");
        match self.call_obscura_tool("browser_click", serde_json::json!({"selector": selector})) {
            Ok(r) => Ok(r),
            Err(e) => Ok(format!("## Click\n\nSelector: {}\n\n⚠ obscura unavailable: {}\n\nFalling back to HTTP mode.", selector, e)),
        }
    }

    fn op_type_text(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let selector = args.get("selector").and_then(|v| v.as_str()).unwrap_or("");
        let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
        match self.call_obscura_tool(
            "browser_type",
            serde_json::json!({"selector": selector, "text": text}),
        ) {
            Ok(r) => Ok(r),
            Err(e) => Ok(format!(
                "## Type\n\nSelector: {}\nText: {}\n\n⚠ obscura unavailable: {}",
                selector, text, e
            )),
        }
    }

    fn op_evaluate(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let js = args
            .get("javascript")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        match self.call_obscura_tool("browser_evaluate", serde_json::json!({"expression": js})) {
            Ok(r) => Ok(r),
            Err(e) => Ok(format!("## Evaluate JS\n\n⚠ obscura unavailable: {}", e)),
        }
    }
}
