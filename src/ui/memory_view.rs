use dioxus::prelude::*;

/// A single memory entry displayed in the list.
#[derive(Debug, Clone, PartialEq)]
struct MemoryEntry {
    id: String,
    content: String,
    category: String,
    created_at: String,
}

/// Sample memory entries for demonstration.
fn sample_memories() -> Vec<MemoryEntry> {
    vec![
        MemoryEntry {
            id: "mem-001".into(),
            content: "用户喜欢使用 Rust 语言开发后端服务".into(),
            category: "偏好".into(),
            created_at: "2026-07-15 10:30".into(),
        },
        MemoryEntry {
            id: "mem-002".into(),
            content: "项目 Hermes_Operit_App 的目标是纯 Rust 重构 Android AI 助手".into(),
            category: "项目".into(),
            created_at: "2026-07-16 14:00".into(),
        },
        MemoryEntry {
            id: "mem-003".into(),
            content: "用户偏好暗色主题和中文界面".into(),
            category: "偏好".into(),
            created_at: "2026-07-17 09:15".into(),
        },
        MemoryEntry {
            id: "mem-004".into(),
            content: "上次对话讨论了 Dioxus UI 框架的使用方式".into(),
            category: "对话".into(),
            created_at: "2026-07-18 16:45".into(),
        },
    ]
}

/// Memory view page — lists known facts about the user and context.
#[component]
pub fn MemoryViewPage() -> Element {
    let mut search_query = use_signal(String::new);
    let memories = sample_memories();

    let filtered: Vec<&MemoryEntry> = {
        let q = search_query().to_lowercase();
        if q.is_empty() {
            memories.iter().collect()
        } else {
            memories
                .iter()
                .filter(|m| {
                    m.content.to_lowercase().contains(&q)
                        || m.category.to_lowercase().contains(&q)
                })
                .collect()
        }
    };

    rsx! {
        div { class: "memory-page",
            h2 { class: "page-title", "🧠 记忆库" }
            p { class: "page-subtitle", "助手对你的了解和上下文记忆" }

            // ── Search bar ──
            div { class: "memory-search",
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "🔍 搜索记忆...",
                    value: "{search_query}",
                    oninput: move |evt: FormEvent| search_query.set(evt.value()),
                }
            }

            // ── Stats ──
            div { class: "memory-stats",
                span { "共 {memories.len()} 条记忆" }
                if !search_query().is_empty() {
                    span { " · 匹配 {filtered.len()} 条" }
                }
            }

            // ── Memory list ──
            div { class: "memory-list",
                if filtered.is_empty() {
                    div { class: "memory-empty",
                        p { "📭 没有找到匹配的记忆" }
                    }
                } else {
                    for mem in filtered.iter() {
                        div {
                            key: "{mem.id}",
                            class: "memory-item",
                            div { class: "memory-item-header",
                                span { class: "memory-category", "🏷️ {mem.category}" }
                                span { class: "memory-time", "{mem.created_at}" }
                            }
                            p { class: "memory-content", "{mem.content}" }
                        }
                    }
                }
            }
        }
    }
}
