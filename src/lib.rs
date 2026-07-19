pub mod core;
pub mod environment;
pub mod mcp;
pub mod store;
pub mod tools;

// UI module requires dioxus (cargo add dioxus@0.5 --features html)
// #[cfg(feature = "gui")]
pub mod ui;

#[cfg(target_os = "android")]
pub mod android;

// Re-exports
pub use core::config::AppConfig;
pub use core::agent::AgentManager;
