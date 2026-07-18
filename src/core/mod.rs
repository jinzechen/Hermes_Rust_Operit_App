pub mod agent;
pub mod config;
pub mod memory;
pub mod provider;
pub mod tool_registry;

// Re-export the most commonly used types at the core module level.
pub use agent::{AgentManager, AgentResponse};
pub use config::AppConfig;
pub use memory::{MemoryEntry, MemoryStore, Message};
pub use provider::{
    AnthropicProvider, GenericProvider, LlmProvider, LlmResponse, OpenAiProvider,
    RouteDecision, SmartModelRouter, TaskType, TokenUsage, ToolCall,
};
pub use tool_registry::{ToolHandler, ToolRegistry, ToolSchema};
