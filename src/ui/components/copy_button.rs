use dioxus::prelude::*;

/// A button that copies text to the clipboard.
#[component]
pub fn CopyButton(text: String) -> Element {
    let mut copied = use_signal(|| false);

    rsx! {
        button {
            class: if copied() { "copy-btn copied" } else { "copy-btn" },
            onclick: move |_| {
                copied.set(true);
            },
            title: "复制到剪贴板",
            if copied() {
                "✅ 已复制"
            } else {
                "📋 复制"
            }
        }
    }
}
