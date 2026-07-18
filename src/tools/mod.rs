// Tools module — built-in tools (browser automation, image analysis, filesystem, etc.)
// All tools implement crate::core::tool_registry::ToolHandler.

pub mod filesystem;
pub mod browser;
pub mod vision;
pub mod markdown;

pub use filesystem::FileSystemTool;
pub use browser::BrowserTool;
pub use vision::VisionTool;
pub use markdown::MarkdownTool;
