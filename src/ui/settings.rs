use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ──────────────────────────────────────────────────────────────────────────────
// Data models (preserved from original skeleton)
// ──────────────────────────────────────────────────────────────────────────────

/// User-facing application UI settings.
///
/// Stored in a YAML file under the platform config directory.
/// This complements `core::AppConfig` which holds LLM provider settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// The default AI model identifier (e.g. "gpt-4", "claude-3-opus").
    pub model_config: String,
    /// UI theme: "dark", "light", or "system".
    pub theme: String,
    /// UI language code (e.g. "en", "zh-CN").
    pub language: String,
    /// API keys map (provider -> key). Serialized with redacted values in Display.
    pub api_keys: std::collections::HashMap<String, String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            model_config: "gpt-4".to_string(),
            theme: "system".to_string(),
            language: "zh-CN".to_string(),
            api_keys: std::collections::HashMap::new(),
        }
    }
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let redacted_keys: std::collections::HashMap<_, _> = self
            .api_keys
            .iter()
            .map(|(k, v)| {
                let redacted = if v.len() > 8 {
                    format!("{}...{}", &v[..4], &v[v.len() - 4..])
                } else if v.is_empty() {
                    "<empty>".to_string()
                } else {
                    "***".to_string()
                };
                (k.clone(), redacted)
            })
            .collect();
        write!(
            f,
            "AppSettings {{ model: {}, theme: {}, lang: {}, api_keys: {:?} }}",
            self.model_config, self.theme, self.language, redacted_keys
        )
    }
}

/// Manages loading, saving, and resetting application settings.
///
/// Uses `core::AppConfig` internally for LLM provider configuration,
/// and manages UI-level `AppSettings` separately via a YAML file on disk.
pub struct SettingsManager {
    /// Path to the settings file on disk.
    config_path: PathBuf,
    /// UI / application-level settings.
    settings: AppSettings,
}

impl SettingsManager {
    /// Default config path inside the user's platform config directory.
    pub fn default_path() -> PathBuf {
        let base = directories::ProjectDirs::from("com", "operit", "HermesOperitApp")
            .map(|d| d.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("settings.yaml")
    }

    /// Create a manager by loading settings from the default path (or defaults on error).
    pub fn load() -> Self {
        let path = Self::default_path();
        Self::load_from(&path).unwrap_or_else(|_| Self {
            config_path: path,
            settings: AppSettings::default(),
        })
    }

    /// Create a manager from an explicit config path.
    pub fn load_from(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let contents = std::fs::read_to_string(path)?;
        let settings: AppSettings = serde_yaml::from_str(&contents)?;
        Ok(Self {
            config_path: path.clone(),
            settings,
        })
    }

    /// Return a reference to the current settings.
    pub fn settings(&self) -> &AppSettings {
        &self.settings
    }

    /// Return a mutable reference to the current settings (for in-place editing).
    pub fn settings_mut(&mut self) -> &mut AppSettings {
        &mut self.settings
    }

    /// Persist the current settings to disk (YAML).
    pub fn save(&self) -> Result<(), anyhow::Error> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let yaml = serde_yaml::to_string(&self.settings)?;
        std::fs::write(&self.config_path, yaml)?;
        Ok(())
    }

    /// Reset settings to defaults and persist.
    pub fn reset(&mut self) -> Result<(), anyhow::Error> {
        self.settings = AppSettings::default();
        self.save()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Dioxus Settings UI Component
// ──────────────────────────────────────────────────────────────────────────────

/// A single settings row with label and action/content.
#[component]
fn SettingsRow(
    icon: &'static str,
    label: &'static str,
    description: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div { class: "settings-row",
            div { class: "settings-row-icon", "{icon}" }
            div { class: "settings-row-info",
                span { class: "settings-row-label", "{label}" }
                span { class: "settings-row-desc", "{description}" }
            }
            div { class: "settings-row-action",
                {children}
            }
        }
    }
}

/// Settings page component.
#[component]
pub fn SettingsPage() -> Element {
    let mut logged_in = use_signal(|| false);
    let mut theme = use_signal(|| "system".to_string());

    rsx! {
        div { class: "settings-page",
            h2 { class: "page-title", "⚙️ 设置" }

            div { class: "settings-list",

                // ── 助手配置 ──
                SettingsRow {
                    icon: "🤖",
                    label: "助手配置",
                    description: "配置 AI 模型、温度、最大 Token 等参数",
                    button { class: "settings-action-btn", "配置 →" }
                }

                // ── API 密钥 ──
                SettingsRow {
                    icon: "🔑",
                    label: "API 密钥",
                    description: "管理 OpenAI、Claude 等第三方 API 密钥",
                    button { class: "settings-action-btn", "管理 →" }
                }

                // ── 角色卡 ──
                SettingsRow {
                    icon: "🃏",
                    label: "角色卡",
                    description: "编辑助手的系统提示词和角色设定",
                    button { class: "settings-action-btn", "编辑 →" }
                }

                // ── 语音服务 ──
                SettingsRow {
                    icon: "🎙️",
                    label: "语音服务",
                    description: "配置语音识别 (STT) 和语音合成 (TTS)",
                    button { class: "settings-action-btn", "配置 →" }
                }

                // ── 主题 ──
                SettingsRow {
                    icon: "🎨",
                    label: "界面主题",
                    description: "选择暗色、亮色或跟随系统主题",
                    select {
                        class: "theme-select",
                        value: "{theme}",
                        onchange: move |evt: FormEvent| theme.set(evt.value()),
                        option { value: "system", "🌓 跟随系统" }
                        option { value: "dark", "🌙 暗色模式" }
                        option { value: "light", "☀️ 亮色模式" }
                    }
                }

                // ── 账户 / GitHub OAuth ──
                SettingsRow {
                    icon: "🔗",
                    label: "账户绑定",
                    description: "通过 GitHub OAuth 登录同步数据和设置",
                    if logged_in() {
                        div { class: "account-status",
                            span { class: "status-dot logged-in" }
                            span { "已登录 GitHub" }
                            button {
                                class: "settings-action-btn logout",
                                onclick: move |_| logged_in.set(false),
                                "退出登录"
                            }
                        }
                    } else {
                        button {
                            class: "github-login-btn",
                            onclick: move |_| logged_in.set(true),
                            "🐙 使用 GitHub 登录"
                        }
                    }
                }

                // ── 关于 ──
                SettingsRow {
                    icon: "ℹ️",
                    label: "关于",
                    description: "Hermes Operit App v0.1.0 · Pure Rust AI Assistant",
                    span { class: "version-text", "v0.1.0" }
                }

            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests (preserved from original)
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let defaults = AppSettings::default();
        assert_eq!(defaults.model_config, "gpt-4");
        assert_eq!(defaults.theme, "system");
        assert_eq!(defaults.language, "zh-CN");
        assert!(defaults.api_keys.is_empty());
    }

    #[test]
    fn test_display_redaction() {
        let mut settings = AppSettings::default();
        settings
            .api_keys
            .insert("openai".into(), "sk-abc...wxyz".into());
        let display = format!("{}", settings);
        // Long keys are truncated: first 4 + "..." + last 4
        assert!(display.contains("sk-a...wxyz"));
        assert!(!display.contains("abcdefghijklmnop"));
    }
}
