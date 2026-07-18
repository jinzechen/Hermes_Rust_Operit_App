use dioxus::prelude::*;
use super::CopyButton;

/// A chat message bubble component that renders a message
/// with role-specific styling (user, assistant, or tool).
#[component]
pub fn MessageBubble(
    role: String,
    content: String,
    timestamp: String,
) -> Element {
    let bubble_class = match role.as_str() {
        "user" => "msg-bubble msg-user",
        "assistant" => "msg-bubble msg-assistant",
        "tool" => "msg-bubble msg-tool",
        _ => "msg-bubble msg-assistant",
    };

    let role_label = match role.as_str() {
        "user" => "👤 你",
        "assistant" => "🤖 助手",
        "tool" => "🔧 工具",
        _ => "🤖 助手",
    };

    rsx! {
        div { class: "msg-row msg-row-{role}",
            div { class: "{bubble_class}",
                div { class: "msg-header",
                    span { class: "msg-role", "{role_label}" }
                    span { class: "msg-time", "{timestamp}" }
                    CopyButton { text: content.clone() }
                }
                div { class: "msg-body",
                    pre { class: "msg-content", "{content}" }
                }
            }
        }
    }
}
