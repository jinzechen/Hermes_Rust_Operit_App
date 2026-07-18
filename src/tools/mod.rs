// Tools module — built-in tools (browser automation, image analysis, filesystem, etc.)
// All tools implement crate::core::tool_registry::ToolHandler.

pub mod filesystem;
pub mod browser;
pub mod vision;
pub mod markdown;
pub mod codebase_analyzer;
pub mod web;
pub mod terminal;
pub mod speech;

pub use filesystem::FileSystemTool;
pub use browser::BrowserTool;
pub use vision::VisionTool;
pub use markdown::MarkdownTool;
pub use codebase_analyzer::CodebaseAnalyzer;
pub use web::WebTool;
pub use terminal::TerminalTool;
pub use speech::SpeechTool;
