use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON Schema description of a tool that the LLM can call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    /// JSON Schema object describing the tool's parameters.
    pub parameters: Value,
}

/// Trait that every tool must implement.
pub trait ToolHandler: Send + Sync {
    /// Execute the tool with the given JSON parameters.
    fn execute(&self, params: Value) -> Result<String>;

    /// Return the tool's schema so the LLM knows how to call it.
    fn schema(&self) -> ToolSchema;
}

/// Thread-safe registry of available tools, keyed by tool name.
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn ToolHandler>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool handler under the given name.
    pub fn register(&mut self, name: impl Into<String>, handler: Arc<dyn ToolHandler>) {
        self.tools.insert(name.into(), handler);
    }

    /// Look up a tool by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn ToolHandler>> {
        self.tools.get(name).cloned()
    }

    /// Return the names of all registered tools.
    pub fn list_all(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Return the number of registered tools.
    pub fn count(&self) -> usize {
        self.tools.len()
    }

    /// Return the schemas of all registered tools (for sending to the LLM).
    pub fn list_tool_schemas(&self) -> Vec<ToolSchema> {
        self.tools.values().map(|h| h.schema()).collect()
    }

    /// Execute a tool by name with the given JSON parameters.
    pub fn execute_tool(&self, name: &str, params: Value) -> Result<String> {
        let handler = self
            .tools
            .get(name)
            .ok_or_else(|| anyhow!("Tool not found: {}", name))?;
        handler.execute(params)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
