use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::config::AppConfig;

// ── Public types ────────────────────────────────────────────────────────────

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

                let mut content: Value;

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
        })
    }
}

// Re-export default_api_endpoint helper for AnthropicProvider
fn default_api_endpoint() -> String {
    "https://api.openai.com/v1/chat/completions".to_string()
}
