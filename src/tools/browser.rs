//! Browser tool — placeholder for obscura browser-automation integration.
//!
//! Currently returns placeholder messages.  TODO: integrate
//! obscura's headless-browser capabilities.

use async_trait::async_trait;

use super::{ToolHandler, ToolInfo, ToolResult};

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

#[async_trait]
impl ToolHandler for BrowserTool {
    fn name(&self) -> &str {
        "browser"
    }

    fn info(&self) -> ToolInfo {
        ToolInfo {
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

    async fn execute(&self, arguments: serde_json::Value) -> ToolResult {
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
                ToolResult {
                    content: format!(
                        "## Browser Navigate (placeholder)\n\n\
                         URL: {}\n\n\
                         > TODO: Integrate obscura for real browser navigation.",
                        url
                    ),
                    is_error: false,
                }
            }
            "screenshot" => ToolResult {
                content: "## Browser Screenshot (placeholder)\n\n\
                     > TODO: Integrate obscura to capture page screenshots."
                    .into(),
                is_error: false,
            },
            "get_html" => {
                let selector = arguments
                    .get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("body");
                ToolResult {
                    content: format!(
                        "## Browser Get HTML (placeholder)\n\n\
                         Selector: {}\n\n\
                         > TODO: Integrate obscura to extract page HTML.",
                        selector
                    ),
                    is_error: false,
                }
            }
            "click" => {
                let selector = arguments
                    .get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("body");
                ToolResult {
                    content: format!(
                        "## Browser Click (placeholder)\n\n\
                         Selector: {}\n\n\
                         > TODO: Integrate obscura for element interaction.",
                        selector
                    ),
                    is_error: false,
                }
            }
            "type_text" => {
                let selector = arguments
                    .get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");
                ToolResult {
                    content: format!(
                        "## Browser Type (placeholder)\n\n\
                         Selector: {}\nText: {}\n\n\
                         > TODO: Integrate obscura for keyboard input.",
                        selector, text
                    ),
                    is_error: false,
                }
            }
            _ => ToolResult {
                content: format!("**Error:** unknown browser action: '{}'", action),
                is_error: true,
            },
        }
    }
}
