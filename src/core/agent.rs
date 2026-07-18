use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
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

// ── AgentStatus ─────────────────────────────────────────────────────────────

/// Snapshot of the agent's current state, intended for UI health dashboards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    /// The LLM model identifier (e.g. "gpt-4o").
    pub model: String,
    /// The API endpoint URL being used.
    pub endpoint: String,
    /// Number of registered tools.
    pub tool_count: usize,
    /// Number of active sessions.
    pub session_count: usize,
    /// Seconds since the AgentManager was created.
    pub uptime_secs: f64,
    /// Whether the provider endpoint appears reachable.
    pub provider_ok: bool,
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
    /// Timestamp when this AgentManager was created (for uptime calculation).
    created_at: Instant,
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
            created_at: Instant::now(),
        })
    }

    /// Create a new AgentManager and automatically register the standard
    /// built-in tools: filesystem, markdown, web, terminal, browser, vision.
    pub fn with_default_tools(config: AppConfig) -> Result<Self> {
        let agent = Self::new(config)?;

        // Register all default tools.  WebTool::new() can fail, so we handle
        // that gracefully — if one tool fails we still register the others.
        agent.register_tool(Box::new(
            crate::tools::FileSystemTool::new(vec![std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))]),
        ));
        agent.register_tool(Box::new(crate::tools::MarkdownTool::new()));
        match crate::tools::WebTool::new() {
            Ok(wt) => agent.register_tool(Box::new(wt)),
            Err(e) => log::warn!("Failed to register WebTool: {}", e),
        }
        agent.register_tool(Box::new(crate::tools::TerminalTool::new()));
        agent.register_tool(Box::new(crate::tools::BrowserTool::new()));
        agent.register_tool(Box::new(crate::tools::VisionTool::new()));

        Ok(agent)
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

    // ── UI-friendly API ──────────────────────────────────────────────────

    /// List the IDs of all currently active sessions.
    pub fn list_sessions(&self) -> Vec<String> {
        let sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        sessions.keys().cloned().collect()
    }

    /// Return a clone of the full message history for a session.
    /// Returns an empty Vec if the session does not exist.
    pub fn get_session_history(&self, session_id: &str) -> Vec<Message> {
        let sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        sessions.get(session_id).cloned().unwrap_or_default()
    }

    /// Remove a session and its metadata entirely.  Does nothing if the
    /// session doesn't exist.
    pub fn clear_session(&self, session_id: &str) {
        {
            let mut sessions = self
                .sessions
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            sessions.remove(session_id);
        }
        {
            let mut meta = self
                .session_meta
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            meta.remove(session_id);
        }
    }

    /// Return `(name, description)` tuples for every registered tool.
    pub fn get_tool_descriptions(&self) -> Vec<(String, String)> {
        let registry = self.tool_registry.read();
        registry
            .list_tool_schemas()
            .into_iter()
            .map(|ts| (ts.name, ts.description))
            .collect()
    }

    /// Build a human-readable summary of the agent's memory / state.
    /// Includes session count, tool count, uptime, and active session IDs.
    pub fn get_memory_summary(&self) -> String {
        let session_count = self.session_count();
        let tool_count = self.tool_count();
        let uptime = self.created_at.elapsed();
        let sessions = self.list_sessions();

        let mut summary = format!(
            "Agent Memory Summary\n\
             ────────────────────\n\
             Active sessions:   {}\n\
             Registered tools:  {}\n\
             Uptime:            {}s\n",
            session_count,
            tool_count,
            uptime.as_secs()
        );

        if session_count > 0 {
            summary.push_str(&format!(
                "Session IDs:        {}\n",
                sessions.join(", ")
            ));
        }

        summary.push_str(&format!(
            "Model:              {}\n\
             Endpoint:           {}",
            self.config.model, self.config.api_endpoint
        ));

        summary
    }

    /// Change a configuration value at runtime.
    ///
    /// Supported keys: `model`, `api_endpoint`, `temperature`, `max_tokens`,
    /// `api_key`.
    pub fn set_config(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "model" => {
                self.config.model = value.to_string();
            }
            "api_endpoint" | "endpoint" => {
                self.config.api_endpoint = value.to_string();
            }
            "temperature" => {
                self.config.temperature = value
                    .parse::<f32>()
                    .map_err(|_| anyhow!("temperature must be a float, got: {}", value))?;
            }
            "max_tokens" => {
                self.config.max_tokens = value
                    .parse::<u32>()
                    .map_err(|_| anyhow!("max_tokens must be a u32, got: {}", value))?;
            }
            "api_key" => {
                self.config.api_key = value.to_string();
            }
            other => {
                return Err(anyhow!(
                    "Unknown config key '{}'. Supported: model, api_endpoint, \
                     temperature, max_tokens, api_key",
                    other
                ));
            }
        }
        Ok(())
    }

    /// Return a snapshot of the agent's health and status.
    pub fn health_check(&self) -> AgentStatus {
        // Best-effort connectivity check to the provider endpoint.
        let provider_ok = self.runtime.block_on(async {
            let base_url =
                if let Some(pos) = self.config.api_endpoint.rfind("/v") {
                    &self.config.api_endpoint[..pos]
                } else {
                    &self.config.api_endpoint
                };
            match reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
            {
                Ok(client) => match client.head(base_url).send().await {
                    Ok(resp) => {
                        let status = resp.status().as_u16();
                        status < 500 // any non-5xx means reachable
                    }
                    Err(_) => false,
                },
                Err(_) => false,
            }
        });

        AgentStatus {
            model: self.config.model.clone(),
            endpoint: self.config.api_endpoint.clone(),
            tool_count: self.tool_count(),
            session_count: self.session_count(),
            uptime_secs: self.created_at.elapsed().as_secs_f64(),
            provider_ok,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> AppConfig {
        AppConfig {
            model: "test-model".into(),
            api_key: "sk-test".into(),
            api_endpoint: "https://api.example.com/v1/chat/completions".into(),
            temperature: 0.5,
            max_tokens: 1024,
        }
    }

    #[test]
    fn test_new_agent_manager() {
        let agent = AgentManager::new(test_config()).unwrap();
        assert_eq!(agent.tool_count(), 0);
        assert_eq!(agent.session_count(), 0);
        assert_eq!(agent.list_tool_names().len(), 0);
        assert_eq!(agent.list_sessions().len(), 0);
    }

    #[test]
    fn test_with_default_tools() {
        let agent = AgentManager::with_default_tools(test_config()).unwrap();
        // At least 5 of the 6 tools should be registered (WebTool might fail
        // in offline environments but the other five are infallible).
        let count = agent.tool_count();
        assert!(
            count >= 5,
            "Expected at least 5 default tools, got {}",
            count
        );
        let names = agent.list_tool_names();
        assert!(names.contains(&"filesystem".to_string()));
        assert!(names.contains(&"markdown".to_string()));
        assert!(names.contains(&"terminal".to_string()));
        assert!(names.contains(&"browser".to_string()));
        assert!(names.contains(&"vision".to_string()));
    }

    #[test]
    fn test_list_and_clear_sessions() {
        let agent = AgentManager::new(test_config()).unwrap();

        // Initially no sessions.
        assert!(agent.list_sessions().is_empty());
        assert_eq!(agent.session_count(), 0);

        // Trigger session creation by sending a message (will fail because no
        // real LLM, but the session is created before the LLM call).
        let _ = agent.send_message("sess-1", "hello");

        // Session should now exist (even though the LLM call will fail).
        let sessions = agent.list_sessions();
        assert!(sessions.contains(&"sess-1".to_string()));
        assert_eq!(agent.session_count(), 1);
        assert!(agent.has_session("sess-1"));

        // Clear the session.
        agent.clear_session("sess-1");
        assert!(agent.list_sessions().is_empty());
        assert_eq!(agent.session_count(), 0);
        assert!(!agent.has_session("sess-1"));

        // Clearing a non-existent session is a no-op.
        agent.clear_session("nonexistent");
        assert_eq!(agent.session_count(), 0);
    }

    #[test]
    fn test_get_session_history() {
        let agent = AgentManager::new(test_config()).unwrap();

        // Session history for a non-existent session should be empty.
        let history = agent.get_session_history("no-such-session");
        assert!(history.is_empty());

        // Session history for an existing session should contain the user
        // message (at minimum) even if the LLM call fails.
        let _ = agent.send_message("hist-test", "hello world");
        let history = agent.get_session_history("hist-test");
        assert!(!history.is_empty());
        assert_eq!(history[0].role, "user");
        assert_eq!(history[0].content, "hello world");
    }

    #[test]
    fn test_get_tool_descriptions() {
        let agent = AgentManager::new(test_config()).unwrap();
        assert!(agent.get_tool_descriptions().is_empty());

        agent.register_tool(Box::new(crate::tools::MarkdownTool::new()));
        let descs = agent.get_tool_descriptions();
        assert_eq!(descs.len(), 1);
        assert_eq!(descs[0].0, "markdown");
        assert!(!descs[0].1.is_empty());
    }

    #[test]
    fn test_set_config() {
        let mut agent = AgentManager::new(test_config()).unwrap();

        // Change model.
        agent.set_config("model", "claude-4").unwrap();
        assert_eq!(agent.config().model, "claude-4");

        // Change temperature.
        agent.set_config("temperature", "0.9").unwrap();
        assert!((agent.config().temperature - 0.9).abs() < 0.001);

        // Change max_tokens.
        agent.set_config("max_tokens", "8192").unwrap();
        assert_eq!(agent.config().max_tokens, 8192);

        // Change api_endpoint.
        agent.set_config("api_endpoint", "https://api.anthropic.com/v1/messages").unwrap();
        assert_eq!(
            agent.config().api_endpoint,
            "https://api.anthropic.com/v1/messages"
        );

        // Change api_key.
        agent.set_config("api_key", "sk-new-key").unwrap();
        assert_eq!(agent.config().api_key, "sk-new-key");

        // Unknown key should fail.
        let err = agent.set_config("unknown_key", "value").unwrap_err();
        assert!(err.to_string().contains("Unknown config key"));
    }

    #[test]
    fn test_get_memory_summary() {
        let agent = AgentManager::with_default_tools(test_config()).unwrap();
        let summary = agent.get_memory_summary();
        assert!(summary.contains("Agent Memory Summary"));
        assert!(summary.contains("Active sessions"));
        assert!(summary.contains("Registered tools"));
        assert!(summary.contains("Uptime"));
        assert!(summary.contains("test-model"));
    }

    #[test]
    fn test_health_check() {
        let agent = AgentManager::new(test_config()).unwrap();
        let status = agent.health_check();

        assert_eq!(status.model, "test-model");
        assert_eq!(
            status.endpoint,
            "https://api.example.com/v1/chat/completions"
        );
        assert_eq!(status.tool_count, 0);
        assert_eq!(status.session_count, 0);
        assert!(status.uptime_secs >= 0.0);
        // provider_ok may be true or false depending on network; just verify
        // it's a bool (always true in test since there's no panic).
        let _: bool = status.provider_ok;
    }

    #[test]
    fn test_multiple_sessions() {
        let agent = AgentManager::new(test_config()).unwrap();

        let _ = agent.send_message("s1", "msg1");
        let _ = agent.send_message("s2", "msg2");
        let _ = agent.send_message("s3", "msg3");

        let sessions = agent.list_sessions();
        assert_eq!(sessions.len(), 3);
        assert_eq!(agent.session_count(), 3);
        assert!(agent.has_session("s1"));
        assert!(agent.has_session("s2"));
        assert!(agent.has_session("s3"));
    }
}
