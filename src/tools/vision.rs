//! Vision tool — placeholder for agentic-vision integration.
//!
//! Currently returns placeholder messages.  TODO: integrate
//! agentic-vision for image analysis capabilities.

use async_trait::async_trait;

use super::{ToolHandler, ToolInfo, ToolResult};

pub struct VisionTool;

impl VisionTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VisionTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolHandler for VisionTool {
    fn name(&self) -> &str {
        "vision"
    }

    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "vision".into(),
            description: "Analyze images — describe scenes, extract text, identify objects".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["analyze_image", "describe_scene"]
                    },
                    "path": {
                        "type": "string",
                        "description": "Path to image file"
                    },
                    "prompt": {
                        "type": "string",
                        "description": "Optional prompt to guide analysis"
                    }
                },
                "required": ["action", "path"]
            }),
        }
    }

    async fn execute(&self, arguments: serde_json::Value) -> ToolResult {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match action {
            "analyze_image" => {
                let path = arguments.get("path").and_then(|v| v.as_str()).unwrap_or("");
                let prompt = arguments
                    .get("prompt")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let mut msg = format!("## Vision Analyze (placeholder)\n\nImage: {}\n", path);
                if !prompt.is_empty() {
                    msg.push_str(&format!("Prompt: {}\n", prompt));
                }
                msg.push_str("\n> TODO: Integrate agentic-vision for real image analysis.");
                ToolResult {
                    content: msg,
                    is_error: false,
                }
            }
            "describe_scene" => {
                let path = arguments.get("path").and_then(|v| v.as_str()).unwrap_or("");
                ToolResult {
                    content: format!(
                        "## Vision Describe (placeholder)\n\n\
                         Image: {}\n\n\
                         > TODO: Integrate agentic-vision for scene description.",
                        path
                    ),
                    is_error: false,
                }
            }
            _ => ToolResult {
                content: format!("**Error:** unknown vision action: '{}'", action),
                is_error: true,
            },
        }
    }
}
