//! Browser tool — HTTP-based web page retrieval and content extraction.
//!
//! Uses `reqwest` (blocking) + `scraper` for real HTML parsing.
//! Interactive actions (click, type_text, screenshot) return placeholder
//! messages — they require a full CDP-based browser (Phase 3).

use anyhow::{bail, Context};
use scraper::{Html, Selector};
use std::sync::Mutex;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

/// Holds the HTML of the last-navigated page so subsequent operations
/// (like `get_html` with a selector) can work on cached content.
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
            description: "Headless browser automation — navigate, get HTML, screenshot, click, type"
                .into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["navigate", "screenshot", "get_html", "click", "type_text"]
                    },
                    "url": { "type": "string", "description": "URL to navigate to" },
                    "selector": { "type": "string", "description": "CSS selector for get_html/click" },
                    "text": { "type": "string", "description": "Text to type (type_text action)" },
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
            _ => bail!("unknown browser action: '{}'", action),
        }
    }
}

// ── operation implementations ─────────────────────────────────────

impl BrowserTool {
    fn op_navigate(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("missing 'url' argument for navigate"))?;

        let client = reqwest::blocking::Client::builder()
            .user_agent("HermesAgent/1.0 (AI assistant)")
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .context("failed to build HTTP client")?;

        let resp = client
            .get(url)
            .send()
            .with_context(|| format!("HTTP GET failed for: {}", url))?;

        // Extract metadata before consuming body.
        let status = resp.status();
        let final_url = resp.url().to_string();
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let body = resp
            .text()
            .with_context(|| format!("failed to read response body from {}", final_url))?;

        // Update cached state.
        if let Ok(mut u) = self.current_url.lock() {
            *u = final_url.clone();
        }
        if let Ok(mut h) = self.current_html.lock() {
            *h = body.clone();
        }

        // Truncate for display.
        let preview: String = body.chars().take(4000).collect();
        let suffix = if body.len() > 4000 { "…" } else { "" };

        Ok(format!(
            "## Navigate\n\nURL: {}\nStatus: {}\nContent-Type: {}\n\n```html\n{}{}\n```\n\n**{} bytes total**",
            final_url,
            status.as_u16(),
            content_type,
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

        let html = {
            let guard = self
                .current_html
                .lock()
                .map_err(|e| anyhow::anyhow!("lock error: {}", e))?;
            if guard.is_empty() {
                bail!("no page loaded — call navigate first");
            }
            guard.clone()
        };

        let document = Html::parse_document(&html);
        let selector = Selector::parse(selector_str)
            .map_err(|e| anyhow::anyhow!("invalid CSS selector '{}': {:?}", selector_str, e))?;

        let mut results = Vec::new();
        for element in document.select(&selector) {
            let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if !text.is_empty() {
                results.push(text.clone());
            }
            let html_frag = element.html();
            if !html_frag.is_empty() && html_frag != text {
                // avoid duplicating simple text nodes
                if results.is_empty() || results.last().map(|s| s.as_str()) != Some(html_frag.as_str()) {
                    results.push(html_frag);
                }
            }
        }

        let url = self.current_url.lock().map(|g| g.clone()).unwrap_or_default();
        Ok(format!(
            "## Get HTML\n\nPage: {}\nSelector: {}\nMatches: {}\n\n{}",
            url,
            selector_str,
            results.len(),
            if results.is_empty() {
                "(no matches)".into()
            } else {
                results.join("\n---\n")
            }
        ))
    }

    fn op_screenshot(&self, _args: &serde_json::Value) -> anyhow::Result<String> {
        Ok(
            "## Screenshot (placeholder)\n\n\
             Screenshot capture requires a full CDP-based headless browser (obscura).\n\
             This will be implemented in Phase 3.\n\
             For now, use `navigate` + `get_html` to inspect page content."
                .into(),
        )
    }

    fn op_click(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let selector = args
            .get("selector")
            .and_then(|v| v.as_str())
            .unwrap_or("body");
        Ok(format!(
            "## Click (placeholder)\n\n\
             Selector: {}\n\n\
             Element interaction requires a CDP-based headless browser (obscura).\n\
             This will be implemented in Phase 3.",
            selector
        ))
    }

    fn op_type_text(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let selector = args
            .get("selector")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
        Ok(format!(
            "## Type Text (placeholder)\n\n\
             Selector: {}\nText: {}\n\n\
             Keyboard input requires a CDP-based headless browser (obscura).\n\
             This will be implemented in Phase 3.",
            selector, text
        ))
    }
}
