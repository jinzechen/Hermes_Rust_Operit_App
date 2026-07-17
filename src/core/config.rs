use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

fn default_model() -> String {
    "gpt-4o".to_string()
}

fn default_api_endpoint() -> String {
    "https://api.openai.com/v1/chat/completions".to_string()
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    4096
}

/// Central configuration for the agent.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// LLM model identifier (e.g. "gpt-4o", "claude-sonnet-4-20250514").
    #[serde(default = "default_model")]
    pub model: String,

    /// API key for the LLM provider.
    #[serde(default)]
    pub api_key: String,

    /// Full URL of the chat-completions endpoint.
    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: String,

    /// Sampling temperature (0.0 – 2.0).
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Maximum number of tokens to generate.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            api_key: String::new(),
            api_endpoint: default_api_endpoint(),
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
        }
    }
}

impl AppConfig {
    /// Load configuration from a YAML or JSON file.
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Self = if path.extension().map_or(false, |ext| ext == "json") {
            serde_json::from_str(&raw)
                .with_context(|| format!("Failed to parse JSON config: {}", path.display()))?
        } else {
            serde_yaml::from_str(&raw)
                .with_context(|| format!("Failed to parse YAML config: {}", path.display()))?
        };

        Ok(config)
    }
}
