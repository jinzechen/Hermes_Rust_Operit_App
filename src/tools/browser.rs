//! Browser tool — placeholder for obscura browser-automation integration.
//!
//! Currently returns placeholder messages.  TODO: integrate
//! obscura's headless-browser capabilities.

use anyhow::bail;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

pub struct BrowserTool;

impl BrowserTool {
    pub fn new() -> Self {
        Self
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
            description: "Headless browser automation — navigate, click, type, screenshot".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["navigate", "screenshot", "get_html", "click", "type_text"]
                    },
                    "url": { "type": "string" },
                    "selector": { "type": "string" },
                    "text": { "type": "string" },
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
            "navigate" => {
                let url = arguments
                    .get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("about:blank");
                Ok(format!(
                    "## Browser Navigate (placeholder)\n\n\
                     URL: {}\n\n\
                     > TODO: Integrate obscura for real browser navigation.",
                    url
                ))
            }
            "screenshot" => Ok(
                "## Browser Screenshot (placeholder)\n\n\
                 > TODO: Integrate obscura to capture page screenshots."
                    .into(),
            ),
            "get_html" => {
                let selector = arguments
                    .get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("body");
                Ok(format!(
                    "## Browser Get HTML (placeholder)\n\n\
                     Selector: {}\n\n\
                     > TODO: Integrate obscura to extract page HTML.",
                    selector
                ))
            }
            "click" => {
                let selector = arguments
                    .get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("body");
                Ok(format!(
                    "## Browser Click (placeholder)\n\n\
                     Selector: {}\n\n\
                     > TODO: Integrate obscura for element interaction.",
                    selector
                ))
            }
            "type_text" => {
                let selector = arguments
                    .get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!(
                    "## Browser Type (placeholder)\n\n\
                     Selector: {}\nText: {}\n\n\
                     > TODO: Integrate obscura for keyboard input.",
                    selector, text
                ))
            }
            _ => bail!("unknown browser action: '{}'", action),
        }
    }
}
