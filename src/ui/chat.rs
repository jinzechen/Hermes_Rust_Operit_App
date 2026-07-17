use serde::{Deserialize, Serialize};

/// A single message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,     // "user" or "assistant"
    pub content: String,
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

/// A text-based chat session (data model only — no Dioxus UI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub session_id: String,
    messages: Vec<ChatMessage>,
}

impl Chat {
    /// Create a new chat session.
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            messages: Vec::new(),
        }
    }

    /// Add a user message to the conversation.
    pub fn add_user_message(&mut self, text: impl Into<String>) {
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: text.into(),
        });
    }

    /// Add an assistant message to the conversation.
    pub fn add_assistant_message(&mut self, text: impl Into<String>) {
        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: text.into(),
        });
    }

    /// Return all messages as a vector of (role, content) tuples.
    pub fn get_messages(&self) -> Vec<(String, String)> {
        self.messages
            .iter()
            .map(|m| (m.role.clone(), m.content.clone()))
            .collect()
    }

    /// Return the raw messages (for serialization or advanced use).
    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }
}

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
        assert_eq!(msgs[0], ("user".to_string(), "Hello".to_string()));
        assert_eq!(msgs[1], ("assistant".to_string(), "Hi there!".to_string()));
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
}
