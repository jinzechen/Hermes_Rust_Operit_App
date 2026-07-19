// Tools module — built-in tools (browser automation, image analysis, filesystem, etc.)
// All tools implement crate::core::tool_registry::ToolHandler.

pub mod browser;
pub mod codebase_analyzer;
pub mod cronjob;
pub mod filesystem;
pub mod markdown;
pub mod process;
pub mod speech;
pub mod terminal;
pub mod vision;
pub mod web;

pub use browser::BrowserTool;
pub use codebase_analyzer::CodebaseAnalyzer;
pub use cronjob::CronJobTool;
pub use filesystem::FileSystemTool;
pub use markdown::MarkdownTool;
pub use process::ProcessTool;
pub use speech::SpeechTool;
pub use terminal::TerminalTool;
pub use vision::VisionTool;
pub use web::WebTool;
