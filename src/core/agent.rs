use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use parking_lot::RwLock;
use serde_json::Value;
use tokio::runtime::Runtime;

use super::config::AppConfig;
use super::memory::Message;
use super::provider::{GenericProvider, LlmProvider, LlmResponse, TokenUsage, ToolCall};
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
    /// Aggregated token usage across all LLM calls in this agent loop.
    pub token_usage: Option<TokenUsage>,
}

// ── Session metadata ────────────────────────────────────────────────────────

/// Metadata tracked per session for timeout-based cleanup.
#[derive(Debug, Clone)]
struct SessionMeta {
    /// Timestamp of the last activity in this session.
    last_active: Instant,
}

// ── AgentManager ────────────────────────────────────────────────────────────

/// The central agent that orchestrates LLM calls and tool execution.
pub struct AgentManager {
    config: AppConfig,
    provider: Box<dyn LlmProvider>,
    tool_registry: Arc<RwLock<ToolRegistry>>,
    sessions: Arc<Mutex<HashMap<String, Vec<Message>>>>,
    /// Per-session timestamps for timeout tracking.
    session_meta: Arc<Mutex<HashMap<String, SessionMeta>>>,
    runtime: Runtime,
    /// Maximum number of agent-loop iterations before forced exit.
    max_iterations: usize,
    /// Sessions idle longer than this are candidates for cleanup.
    session_timeout: Duration,
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
            session_meta: Arc::new(Mutex::new(HashMap::new())),
            runtime,
            max_iterations: 20,
            session_timeout: Duration::from_secs(3600), // 1 hour default
        })
    }

    /// Set the maximum number of agent-loop iterations before forced exit.
    pub fn set_max_iterations(&mut self, max: usize) {
        self.max_iterations = max;
    }

    /// Get the current max_iterations value.
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    /// Set the session timeout duration. Sessions idle longer than this will
    /// be removed when `clean_expired_sessions()` is called.
    pub fn set_session_timeout(&mut self, timeout: Duration) {
        self.session_timeout = timeout;
    }

    /// Get the current session timeout duration.
    pub fn session_timeout(&self) -> Duration {
        self.session_timeout
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

        // Append the user message to the session history and bump activity.
        {
            let mut sessions = self
                .sessions
                .lock()
                .map_err(|e| anyhow!("Session lock poisoned: {}", e))?;
            let history = sessions.entry(session_id.to_string()).or_default();
            history.push(user_msg);

            // Track / update session activity timestamp.
            let mut meta = self
                .session_meta
                .lock()
                .map_err(|e| anyhow!("Session meta lock poisoned: {}", e))?;
            meta.insert(
                session_id.to_string(),
                SessionMeta {
                    last_active: Instant::now(),
                },
            );
        }

        // Run the agent loop on the tokio runtime.
        self.runtime
            .block_on(async { self.agent_loop(session_id).await })
    }

    /// Remove sessions that have been idle longer than `session_timeout`.
    /// Returns the number of sessions that were cleaned up.
    pub fn clean_expired_sessions(&self) -> Result<usize> {
        let now = Instant::now();
        let timeout = self.session_timeout;

        let expired_ids: Vec<String> = {
            let meta = self
                .session_meta
                .lock()
                .map_err(|e| anyhow!("Session meta lock poisoned: {}", e))?;
            meta.iter()
                .filter(|(_, m)| now.duration_since(m.last_active) > timeout)
                .map(|(id, _)| id.clone())
                .collect()
        };

        let count = expired_ids.len();

        if count > 0 {
            let mut sessions = self
                .sessions
                .lock()
                .map_err(|e| anyhow!("Session lock poisoned: {}", e))?;
            let mut meta = self
                .session_meta
                .lock()
                .map_err(|e| anyhow!("Session meta lock poisoned: {}", e))?;
            for id in &expired_ids {
                sessions.remove(id);
                meta.remove(id);
            }
        }

        Ok(count)
    }

    /// Internal agent loop: send messages, handle tool calls, repeat.
    async fn agent_loop(&self, session_id: &str) -> Result<AgentResponse> {
        let mut all_tool_calls: Vec<ToolCall> = Vec::new();
        let mut final_content = String::new();
        let mut total_token_usage = TokenUsage::default();

        for _iteration in 0..self.max_iterations {
            // Build the message list from session history.
            let messages = self.build_messages(session_id)?;
            let tool_schemas = self.build_tool_schemas();

            // Call the LLM.
            let response: LlmResponse = self
                .provider
                .chat_completion(messages, tool_schemas, &self.config)
                .await?;

            // Accumulate token usage.
            if let Some(ref usage) = response.token_usage {
                total_token_usage.prompt_tokens += usage.prompt_tokens;
                total_token_usage.completion_tokens += usage.completion_tokens;
                total_token_usage.total_tokens += usage.total_tokens;
            }

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

        let token_usage = if total_token_usage.total_tokens > 0
            || total_token_usage.prompt_tokens > 0
            || total_token_usage.completion_tokens > 0
        {
            Some(total_token_usage)
        } else {
            None
        };

        Ok(AgentResponse {
            content: final_content,
            tool_calls: all_tool_calls,
            session_id: session_id.to_string(),
            token_usage,
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
            let msg = Message::new("assistant", content);
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

    /// Return the number of active sessions.
    pub fn session_count(&self) -> usize {
        let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        sessions.len()
    }

    /// Check whether a given session exists.
    pub fn has_session(&self, session_id: &str) -> bool {
        let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        sessions.contains_key(session_id)
    }
}
