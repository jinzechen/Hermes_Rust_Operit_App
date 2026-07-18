use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::config::AppConfig;

// ── TaskType ────────────────────────────────────────────────────────────────

/// Classification of the user's task, used by SmartModelRouter to pick
/// the best model / strategy for each request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskType {
    /// Compilation, build-script assistance, linker errors.
    Compile,
    /// Code review, style checks, correctness analysis.
    CodeReview,
    /// Generating documentation, doc-comments, READMEs.
    Documentation,
    /// General conversation / Q&A.
    Chat,
    /// Deep research, multi-step reasoning, data analysis.
    Research,
}

impl TaskType {
    /// Heuristic classification from the user's message text.
    pub fn classify(message: &str) -> Self {
        let lower = message.to_lowercase();
        if lower.contains("compile")
            || lower.contains("build")
            || lower.contains("linker")
            || lower.contains("cargo build")
            || lower.contains("compilation")
        {
            TaskType::Compile
        } else if lower.contains("review")
            || lower.contains("audit")
            || lower.contains("lint")
            || lower.contains("refactor")
        {
            TaskType::CodeReview
        } else if lower.contains("document")
            || lower.contains("readme")
            || lower.contains("doc comment")
            || lower.contains("javadoc")
        {
            TaskType::Documentation
        } else if lower.contains("research")
            || lower.contains("analyze")
            || lower.contains("deep dive")
            || lower.contains("investigate")
        {
            TaskType::Research
        } else {
            TaskType::Chat
        }
    }
}

// ── SmartModelRouter ────────────────────────────────────────────────────────

/// Routes requests to the most appropriate model / parameters based on
/// the detected TaskType. Uses a moka in-memory cache to avoid
/// re-classification of identical prompts.
pub struct SmartModelRouter {
    /// Caches TaskType classifications keyed by a hash of the message text.
    classification_cache: Cache<u64, TaskType>,
    /// Caches routing decisions (model override strings) for fast lookup.
    routing_cache: Cache<TaskType, RouteDecision>,
}

/// The result of routing a request — may suggest a model override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteDecision {
    /// If Some, use this model name instead of the one in AppConfig.
    pub model_override: Option<String>,
    /// If Some, override the default temperature.
    pub temperature_override: Option<f32>,
    /// If Some, override the default max_tokens.
    pub max_tokens_override: Option<u32>,
}

impl Default for RouteDecision {
    fn default() -> Self {
        Self {
            model_override: None,
            temperature_override: None,
            max_tokens_override: None,
        }
    }
}

impl SmartModelRouter {
    /// Create a new router with default caches.
    pub fn new() -> Self {
        Self {
            classification_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(std::time::Duration::from_secs(600))
                .build(),
            routing_cache: Cache::builder()
                .max_capacity(100)
                .time_to_live(std::time::Duration::from_secs(3600))
                .build(),
        }
    }

    /// Create a router with custom cache sizes.
    pub fn with_cache_sizes(
        classification_max: u64,
        routing_max: u64,
    ) -> Self {
        Self {
            classification_cache: Cache::builder()
                .max_capacity(classification_max)
                .time_to_live(std::time::Duration::from_secs(600))
                .build(),
            routing_cache: Cache::builder()
                .max_capacity(routing_max)
                .time_to_live(std::time::Duration::from_secs(3600))
                .build(),
        }
    }

    /// Route a user message: classify → look up or compute routing decision.
    pub async fn route(&self, message: &str) -> RouteDecision {
        let hash = Self::hash_message(message);
        let task_type = self
            .classification_cache
            .get_with(hash, async { TaskType::classify(message) })
            .await;

        self.routing_cache
            .get_with(task_type, async { Self::decide(task_type) })
            .await
    }

    /// Compute the routing decision for a given TaskType.
    fn decide(task_type: TaskType) -> RouteDecision {
        match task_type {
            TaskType::Compile => RouteDecision {
                temperature_override: Some(0.1),
                max_tokens_override: Some(2048),
                ..Default::default()
            },
            TaskType::CodeReview => RouteDecision {
                temperature_override: Some(0.2),
                max_tokens_override: Some(4096),
                ..Default::default()
            },
            TaskType::Documentation => RouteDecision {
                temperature_override: Some(0.3),
                max_tokens_override: Some(8192),
                ..Default::default()
            },
            TaskType::Chat => RouteDecision {
                temperature_override: Some(0.7),
                ..Default::default()
            },
            TaskType::Research => RouteDecision {
                temperature_override: Some(0.4),
                max_tokens_override: Some(16384),
                ..Default::default()
            },
        }
    }

    /// Simple non-cryptographic hash for message dedup in the cache.
    fn hash_message(msg: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        msg.hash(&mut h);
        h.finish()
    }

    /// Manually insert a routing decision into the cache.
    pub async fn set_route(&self, task_type: TaskType, decision: RouteDecision) {
        self.routing_cache.insert(task_type, decision).await;
    }

    /// Clear all cached data.
    pub fn clear(&self) {
        self.classification_cache.invalidate_all();
        self.routing_cache.invalidate_all();
    }
}

impl Default for SmartModelRouter {
    fn default() -> Self {
        Self::new()
    }
}

// ── Public types ────────────────────────────────────────────────────────────

/// Token usage statistics returned by the LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    /// Number of tokens in the prompt (input).
    pub prompt_tokens: u32,
    /// Number of tokens in the completion (output).
    pub completion_tokens: u32,
    /// Total tokens consumed.
    pub total_tokens: u32,
}

/// A single tool call the LLM wants the agent to execute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// Response returned by an LLM provider after a chat-completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// Text content from the assistant (may be None when only tool calls are returned).
    pub content: Option<String>,
    /// Tool calls the LLM wants the agent to execute.
    pub tool_calls: Vec<ToolCall>,
    /// Token usage information (if available from the provider).
    pub token_usage: Option<TokenUsage>,
}

// ── Provider trait ──────────────────────────────────────────────────────────

/// Abstraction over different LLM backends (OpenAI, Anthropic, generic OpenAI-compatible).
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Send messages + tool definitions to the LLM and return its response.
    ///
    /// `messages` is a list of chat messages, each a JSON object with at least
    /// `role` and `content` fields.
    ///
    /// `tools` is a list of tool schemas (JSON objects following the provider's
    /// expected format).
    async fn chat_completion(
        &self,
        messages: Vec<Value>,
        tools: Vec<Value>,
        config: &AppConfig,
    ) -> Result<LlmResponse>;
}

// ── OpenAI-compatible provider (works with OpenAI, Groq, Together, etc.) ────

/// Provider for OpenAI and OpenAI-compatible APIs.
pub struct GenericProvider {
    client: reqwest::Client,
}

impl GenericProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Build the request body expected by OpenAI-compatible endpoints.
    fn build_request_body(messages: &[Value], tools: &[Value], config: &AppConfig) -> Value {
        let mut body = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        if !tools.is_empty() {
            body["tools"] = serde_json::json!(tools);
        }

        body
    }

    /// Extract token usage from an OpenAI-format response body.
    fn parse_usage(resp_body: &Value) -> Option<TokenUsage> {
        let usage = resp_body.get("usage")?;
        Some(TokenUsage {
            prompt_tokens: usage.get("prompt_tokens")?.as_u64()? as u32,
            completion_tokens: usage.get("completion_tokens")?.as_u64()? as u32,
            total_tokens: usage.get("total_tokens")?.as_u64()? as u32,
        })
    }
}

impl Default for GenericProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmProvider for GenericProvider {
    async fn chat_completion(
        &self,
        messages: Vec<Value>,
        tools: Vec<Value>,
        config: &AppConfig,
    ) -> Result<LlmResponse> {
        let body = Self::build_request_body(&messages, &tools, config);

        let resp = self
            .client
            .post(&config.api_endpoint)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to LLM provider")?;

        let status = resp.status();
        let resp_body: Value = resp
            .json()
            .await
            .context("Failed to parse LLM response body")?;

        if !status.is_success() {
            return Err(anyhow!(
                "LLM provider returned error {}: {}",
                status,
                serde_json::to_string_pretty(&resp_body).unwrap_or_default()
            ));
        }

        // Parse token usage
        let token_usage = Self::parse_usage(&resp_body);

        // Parse OpenAI response format:
        // { "choices": [ { "message": { "content": ..., "tool_calls": [...] } } ] }
        let choice = resp_body["choices"]
            .as_array()
            .and_then(|a| a.first())
            .ok_or_else(|| anyhow!("No choices in LLM response"))?;

        let msg = &choice["message"];

        let content = msg["content"].as_str().map(|s| s.to_string());

        let tool_calls = msg["tool_calls"]
            .as_array()
            .map(|tc_arr| {
                tc_arr
                    .iter()
                    .map(|tc| {
                        let id = tc["id"].as_str().unwrap_or("").to_string();
                        let function = &tc["function"];
                        let name = function["name"].as_str().unwrap_or("").to_string();
                        let arguments: Value = function["arguments"]
                            .as_str()
                            .and_then(|s| serde_json::from_str(s).ok())
                            .unwrap_or(Value::Object(serde_json::Map::new()));
                        ToolCall {
                            id,
                            name,
                            arguments,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(LlmResponse {
            content,
            tool_calls,
            token_usage,
        })
    }
}

/// Type alias for backward compatibility — the generic provider works
/// with any OpenAI-compatible API (OpenAI itself, Groq, Together, etc.).
pub type OpenAiProvider = GenericProvider;

// ── Anthropic provider ──────────────────────────────────────────────────────

/// Provider for the Anthropic Messages API.
pub struct AnthropicProvider {
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Convert OpenAI-format tool schemas to Anthropic's format.
    /// Anthropic expects: { name, description, input_schema }
    fn convert_tools_to_anthropic(tools: &[Value]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                let name = t["function"]["name"].as_str().unwrap_or("");
                let description = t["function"]["description"].as_str().unwrap_or("");
                let parameters = &t["function"]["parameters"];
                serde_json::json!({
                    "name": name,
                    "description": description,
                    "input_schema": parameters,
                })
            })
            .collect()
    }

    /// Convert OpenAI-format messages to Anthropic's format.
    /// Anthropic expects system as a top-level field, not as a message.
    fn convert_messages_to_anthropic(messages: &[Value]) -> (Vec<Value>, Option<String>) {
        let mut system_prompt: Option<String> = None;
        let mut anthropic_msgs: Vec<Value> = Vec::new();

        for msg in messages {
            let role = msg["role"].as_str().unwrap_or("user");
            if role == "system" {
                system_prompt = msg["content"].as_str().map(|s| s.to_string());
            } else {
                // Map roles: user→user, assistant→assistant, tool→user (with tool_result content)
                let anthro_role = match role {
                    "assistant" => "assistant",
                    "tool" => "user",
                    _ => "user",
                };

                let content: Value;

                if role == "tool" {
                    // Anthropic expects tool results as user messages with tool_result blocks
                    let tool_call_id = msg
                        .get("tool_call_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let result_content = msg["content"].as_str().unwrap_or("");
                    content = serde_json::json!([{
                        "type": "tool_result",
                        "tool_use_id": tool_call_id,
                        "content": result_content,
                    }]);
                } else if role == "assistant" && msg.get("tool_calls").is_some() {
                    // Assistant message with tool_use blocks
                    let text = msg["content"].as_str().unwrap_or("");
                    let mut content_blocks: Vec<Value> = Vec::new();
                    if !text.is_empty() {
                        content_blocks.push(serde_json::json!({
                            "type": "text",
                            "text": text,
                        }));
                    }
                    if let Some(tc_arr) = msg["tool_calls"].as_array() {
                        for tc in tc_arr {
                            let tc_id = tc["id"].as_str().unwrap_or("");
                            let tc_name = tc["function"]["name"].as_str().unwrap_or("");
                            let tc_args: Value = tc["function"]["arguments"]
                                .as_str()
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or(Value::Object(serde_json::Map::new()));
                            content_blocks.push(serde_json::json!({
                                "type": "tool_use",
                                "id": tc_id,
                                "name": tc_name,
                                "input": tc_args,
                            }));
                        }
                    }
                    content = Value::Array(content_blocks);
                } else {
                    content = serde_json::json!([{
                        "type": "text",
                        "text": msg["content"].as_str().unwrap_or(""),
                    }]);
                }

                anthropic_msgs.push(serde_json::json!({
                    "role": anthro_role,
                    "content": content,
                }));
            }
        }

        (anthropic_msgs, system_prompt)
    }

    /// Extract token usage from an Anthropic-format response body.
    fn parse_usage(resp_body: &Value) -> Option<TokenUsage> {
        let usage = resp_body.get("usage")?;
        Some(TokenUsage {
            prompt_tokens: usage
                .get("input_tokens")
                .and_then(|v| v.as_u64())? as u32,
            completion_tokens: usage
                .get("output_tokens")
                .and_then(|v| v.as_u64())? as u32,
            total_tokens: 0, // Anthropic doesn't always give total; compute it
        })
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn chat_completion(
        &self,
        messages: Vec<Value>,
        tools: Vec<Value>,
        config: &AppConfig,
    ) -> Result<LlmResponse> {
        let (anthropic_msgs, system_prompt) = Self::convert_messages_to_anthropic(&messages);
        let anthropic_tools = Self::convert_tools_to_anthropic(&tools);

        let mut body = serde_json::json!({
            "model": config.model,
            "max_tokens": config.max_tokens,
            "messages": anthropic_msgs,
        });

        if let Some(sys) = system_prompt {
            body["system"] = serde_json::json!(sys);
        }

        if !anthropic_tools.is_empty() {
            body["tools"] = serde_json::json!(anthropic_tools);
        }

        // Anthropic requires temperature to be in [0, 1]
        let temperature = config.temperature.clamp(0.0, 1.0);
        body["temperature"] = serde_json::json!(temperature);

        let endpoint = if config.api_endpoint == default_api_endpoint() {
            // Use Anthropic's default endpoint unless the user explicitly overrode it
            "https://api.anthropic.com/v1/messages"
        } else {
            &config.api_endpoint
        };

        let resp = self
            .client
            .post(endpoint)
            .header("x-api-key", &config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        let status = resp.status();
        let resp_body: Value = resp
            .json()
            .await
            .context("Failed to parse Anthropic response body")?;

        if !status.is_success() {
            return Err(anyhow!(
                "Anthropic returned error {}: {}",
                status,
                serde_json::to_string_pretty(&resp_body).unwrap_or_default()
            ));
        }

        // Parse token usage
        let token_usage = Self::parse_usage(&resp_body);

        // Parse Anthropic response: { "content": [ { "type": "text", "text": "..." }, ... ] }
        let content_blocks = resp_body["content"].as_array();
        let mut text_content = String::new();
        let mut tool_calls: Vec<ToolCall> = Vec::new();

        if let Some(blocks) = content_blocks {
            for block in blocks {
                match block["type"].as_str().unwrap_or("") {
                    "text" => {
                        if let Some(t) = block["text"].as_str() {
                            if !text_content.is_empty() {
                                text_content.push('\n');
                            }
                            text_content.push_str(t);
                        }
                    }
                    "tool_use" => {
                        let id = block["id"].as_str().unwrap_or("").to_string();
                        let name = block["name"].as_str().unwrap_or("").to_string();
                        let input = block["input"].clone();
                        tool_calls.push(ToolCall {
                            id,
                            name,
                            arguments: input,
                        });
                    }
                    _ => {}
                }
            }
        }

        let content = if text_content.is_empty() {
            None
        } else {
            Some(text_content)
        };

        Ok(LlmResponse {
            content,
            tool_calls,
            token_usage,
        })
    }
}

// Re-export default_api_endpoint helper for AnthropicProvider
fn default_api_endpoint() -> String {
    "https://api.openai.com/v1/chat/completions".to_string()
}
