//! Vision tool — placeholder for agentic-vision integration.
//!
//! Currently returns placeholder messages.  TODO: integrate
//! agentic-vision for image analysis capabilities.

use anyhow::bail;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

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

impl ToolHandler for VisionTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
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

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
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
                Ok(msg)
            }
            "describe_scene" => {
                let path = arguments.get("path").and_then(|v| v.as_str()).unwrap_or("");
                Ok(format!(
                    "## Vision Describe (placeholder)\n\n\
                     Image: {}\n\n\
                     > TODO: Integrate agentic-vision for scene description.",
                    path
                ))
            }
            _ => bail!("unknown vision action: '{}'", action),
        }
    }
}
