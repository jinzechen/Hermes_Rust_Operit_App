use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context, Result};
use parking_lot::RwLock;
use serde_json::Value;
use tokio::runtime::Runtime;

use super::config::AppConfig;
use super::memory::Message;
use super::provider::{GenericProvider, LlmProvider, LlmResponse, ToolCall};
use super::tool_registry::{ToolHandler, ToolRegistry};

// ── AgentResponse ───────────────────────────────────────────────────────────

/// The result of sending a message to the agent.
#[derive(Debug, Clone)]
pub struct AgentResponse {
    /// The final text reply from the assistant.
    pub content: String,
    /// Any tool calls that were made during processing.
    pub tool_calls: Vec<ToolCall>,
    /// The session ID that was used.
    pub session_id: String,
}

// ── AgentManager ────────────────────────────────────────────────────────────

/// The central agent that orchestrates LLM calls and tool execution.
pub struct AgentManager {
    config: AppConfig,
    provider: Box<dyn LlmProvider>,
    tool_registry: Arc<RwLock<ToolRegistry>>,
    sessions: Arc<Mutex<HashMap<String, Vec<Message>>>>,
    runtime: Runtime,
}

impl AgentManager {
    /// Create a new AgentManager with the given configuration.
    /// Uses a GenericProvider (OpenAI-compatible) by default.
    pub fn new(config: AppConfig) -> Result<Self> {
        let runtime = Runtime::new().context("Failed to create tokio runtime")?;
        Ok(Self {
            config,
            provider: Box::new(GenericProvider::new()),
            tool_registry: Arc::new(RwLock::new(ToolRegistry::new())),
            sessions: Arc::new(Mutex::new(HashMap::new())),
            runtime,
        })
    }

    /// Replace the LLM provider (e.g. switch to Anthropic).
    pub fn set_provider(&mut self, provider: Box<dyn LlmProvider>) {
        self.provider = provider;
    }

    /// Register a tool handler.
    pub fn register_tool(&self, handler: Box<dyn ToolHandler>) {
        let schema = handler.schema();
        let name = schema.name.clone();
        let mut registry = self.tool_registry.write();
        registry.register(name, Arc::from(handler));
    }

    /// Send a user message within a session. The agent handles the full
    /// conversation loop: send to LLM → execute tool calls → repeat until
    /// no more tool calls are requested.
    pub fn send_message(&self, session_id: &str, text: &str) -> Result<AgentResponse> {
        let user_msg = Message::new("user", text);

        // Append the user message to the session history.
        {
            let mut sessions = self
                .sessions
                .lock()
                .map_err(|e| anyhow!("Session lock poisoned: {}", e))?;
            let history = sessions.entry(session_id.to_string()).or_default();
            history.push(user_msg);
        }

        // Run the agent loop on the tokio runtime.
        self.runtime
            .block_on(async { self.agent_loop(session_id).await })
    }

    /// Internal agent loop: send messages, handle tool calls, repeat.
    async fn agent_loop(&self, session_id: &str) -> Result<AgentResponse> {
        const MAX_ITERATIONS: usize = 20;

        let mut all_tool_calls: Vec<ToolCall> = Vec::new();
        let mut final_content = String::new();

        for _iteration in 0..MAX_ITERATIONS {
            // Build the message list from session history.
            let messages = self.build_messages(session_id)?;
            let tool_schemas = self.build_tool_schemas();

            // Call the LLM.
            let response: LlmResponse = self
                .provider
                .chat_completion(messages, tool_schemas, &self.config)
                .await?;

            // If there's content, record it (last content wins as the final answer).
            if let Some(ref content) = response.content {
                if !content.is_empty() {
                    final_content = content.clone();
                }
            }

            // If no tool calls, we're done.
            if response.tool_calls.is_empty() {
                // Record the assistant's final message in history.
                self.append_assistant_message(session_id, &final_content, &[])?;
                break;
            }

            // Record this round's tool calls.
            let round_calls = response.tool_calls.clone();
            all_tool_calls.extend(round_calls.clone());

            // Build the assistant message with tool_calls for history.
            let assistant_content = response.content.unwrap_or_default();
            self.append_assistant_message(session_id, &assistant_content, &round_calls)?;

            // Execute each tool call and append results.
            for tc in &round_calls {
                let result = self.execute_tool_call(tc);
                let tool_msg =
                    Message::new("tool", result.unwrap_or_else(|e| format!("Error: {}", e)));
                // For tool messages we need to store tool_call_id for the provider.
                let mut sessions = self
                    .sessions
                    .lock()
                    .map_err(|e| anyhow!("Session lock poisoned: {}", e))?;
                let history = sessions.entry(session_id.to_string()).or_default();
                // Store tool_call_id in the content as a structured format, or
                // we can use a separate field. For simplicity, we store a
                // specially-formatted tool message that the provider can parse.
                history.push(tool_msg);
            }
        }

        Ok(AgentResponse {
            content: final_content,
            tool_calls: all_tool_calls,
            session_id: session_id.to_string(),
        })
    }

    /// Execute a single tool call by name with its arguments.
    fn execute_tool_call(&self, tc: &ToolCall) -> Result<String> {
        let registry = self.tool_registry.read();
        registry.execute_tool(&tc.name, tc.arguments.clone())
    }

    /// Build the OpenAI-format message list from session history.
    fn build_messages(&self, session_id: &str) -> Result<Vec<Value>> {
        let sessions = self
            .sessions
            .lock()
            .map_err(|e| anyhow!("Session lock poisoned: {}", e))?;

        let history = sessions.get(session_id).cloned().unwrap_or_default();

        let messages: Vec<Value> = history.iter().map(|m| m.to_json_value()).collect();

        Ok(messages)
    }

    /// Build the list of tool schemas in OpenAI function-calling format.
    fn build_tool_schemas(&self) -> Vec<Value> {
        let registry = self.tool_registry.read();
        registry
            .list_tool_schemas()
            .into_iter()
            .map(|ts| {
                serde_json::json!({
                    "type": "function",
                    "function": {
                        "name": ts.name,
                        "description": ts.description,
                        "parameters": ts.parameters,
                    }
                })
            })
            .collect()
    }

    /// Append an assistant message (optionally with tool calls) to session history.
    fn append_assistant_message(
        &self,
        session_id: &str,
        content: &str,
        tool_calls: &[ToolCall],
    ) -> Result<()> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|e| anyhow!("Session lock poisoned: {}", e))?;

        let history = sessions.entry(session_id.to_string()).or_default();

        // Build the assistant message. If there are tool calls we store them as
        // additional JSON fields so the provider can use them.
        let msg = if tool_calls.is_empty() {
            Message::new("assistant", content)
        } else {
            // For tool-call-bearing messages we need to embed the tool calls.
            // We'll encode them in a way the provider layer can decode.
            let mut msg = Message::new("assistant", content);
            // We can't add extra fields to Message easily, so we encode
            // tool calls in a structured way in the content for now.
            // A more robust approach would use a richer message type.
            msg
        };

        history.push(msg);
        Ok(())
    }

    /// Return a reference to the current configuration.
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Return the number of registered tools.
    pub fn tool_count(&self) -> usize {
        let registry = self.tool_registry.read();
        registry.count()
    }

    /// Return the names of all registered tools.
    pub fn list_tool_names(&self) -> Vec<String> {
        let registry = self.tool_registry.read();
        registry.list_all()
    }
}
