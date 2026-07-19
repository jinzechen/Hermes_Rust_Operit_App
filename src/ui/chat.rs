use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use crate::ui::components::MessageBubble;
use chrono::Local;

// ──────────────────────────────────────────────────────────────────────────────
// Data models (preserved from original skeleton)
// ──────────────────────────────────────────────────────────────────────────────

/// A single message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub role: String,     // "user" or "assistant"
    pub content: String,
    pub timestamp: String,
}

impl ChatMessage {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            timestamp: Local::now().format("%H:%M:%S").to_string(),
        }
    }
}

/// A tool call embedded in an assistant reply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

/// The data model for an assistant's reply, which may include text and tool calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReply {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
}

impl ChatReply {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tool_calls: Vec::new(),
        }
    }

    pub fn with_tool_calls(text: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            text: text.into(),
            tool_calls,
        }
    }
}

/// A text-based chat session (data model only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub session_id: String,
    messages: Vec<ChatMessage>,
}

impl Chat {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            messages: Vec::new(),
        }
    }

    pub fn add_user_message(&mut self, text: impl Into<String>) {
        self.messages.push(ChatMessage::new("user", text));
    }

    pub fn add_assistant_message(&mut self, text: impl Into<String>) {
        self.messages.push(ChatMessage::new("assistant", text));
    }

    pub fn get_messages(&self) -> Vec<(String, String)> {
        self.messages
            .iter()
            .map(|m| (m.role.clone(), m.content.clone()))
            .collect()
    }

    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Dioxus Chat UI Component
// ──────────────────────────────────────────────────────────────────────────────

/// The chat page component — full conversation interface.
#[component]
pub fn ChatPage() -> Element {
    let mut messages = use_signal(Vec::<ChatMessage>::new);
    let mut input_text = use_signal(String::new);

    let mut send_message = {
        let mut messages = messages;
        let mut input_text = input_text;
        move |_| {
            let text = input_text().trim().to_string();
            if text.is_empty() {
                return;
            }
            // Add user message
            messages.write().push(ChatMessage::new("user", text.clone()));
            input_text.set(String::new());

            // Simulated assistant response (placeholder until backend is wired)
            let mut msgs = messages;
            let reply = text;
            spawn(async move {
                // Simulate network delay
                #[cfg(target_arch = "wasm32")]
                gloo_timers::future::TimeoutFuture::new(800).await;
                #[cfg(not(target_arch = "wasm32"))]
                tokio::time::sleep(std::time::Duration::from_millis(800)).await;

                msgs.write().push(ChatMessage::new(
                    "assistant",
                    format!("收到你的消息：「{}」\n\n这是一个模拟回复。当后端连接后，这里会显示 AI 的真实回复。", reply),
                ));
            });
        }
    };

    let handle_keydown = {
        let mut send_message = send_message.clone();
        move |evt: Event<KeyboardData>| {
            if evt.key() == Key::Enter && !evt.modifiers().shift() {
                send_message(());
            }
        }
    };

    rsx! {
        div { class: "chat-page",
            // ── Message list ──
            div { class: "chat-messages",
                if messages().is_empty() {
                    div { class: "chat-empty",
                        p { "👋 欢迎使用 Hermes 对话" }
                        p { class: "chat-hint", "在下方输入消息，按 Enter 发送，Shift+Enter 换行" }
                    }
                } else {
                    for msg in messages().iter() {
                        MessageBubble {
                            key: "{msg.timestamp}-{msg.role}",
                            role: msg.role.clone(),
                            content: msg.content.clone(),
                            timestamp: msg.timestamp.clone(),
                        }
                    }
                }
            }

            // ── Input area ──
            div { class: "chat-input-area",
                textarea {
                    class: "chat-input",
                    value: "{input_text}",
                    placeholder: "输入消息... (Enter 发送, Shift+Enter 换行)",
                    oninput: move |evt: FormEvent| input_text.set(evt.value()),
                    onkeydown: handle_keydown,
                    rows: "2",
                }
                button {
                    class: "chat-send-btn",
                    onclick: move |_| send_message(()),
                    disabled: input_text().trim().is_empty(),
                    "📨 发送"
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
    fn test_chat_basics() {
        let mut chat = Chat::new("session-1");
        chat.add_user_message("Hello");
        chat.add_assistant_message("Hi there!");
        let msgs = chat.get_messages();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].0, "user");
        assert_eq!(msgs[0].1, "Hello");
        assert_eq!(msgs[1].0, "assistant");
        assert_eq!(msgs[1].1, "Hi there!");
    }

    #[test]
    fn test_chat_reply() {
        let reply = ChatReply::new("Done");
        assert_eq!(reply.text, "Done");
        assert!(reply.tool_calls.is_empty());

        let reply_with_tools = ChatReply::with_tool_calls(
            "Running tool...",
            vec![ToolCall {
                name: "read_file".into(),
                arguments: serde_json::json!({"path": "/tmp/test"}),
            }],
        );
        assert_eq!(reply_with_tools.tool_calls.len(), 1);
    }

    #[test]
    fn test_chat_message_new() {
        let msg = ChatMessage::new("user", "测试消息");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "测试消息");
        assert!(!msg.timestamp.is_empty());
    }
}
