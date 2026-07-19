use crate::ui::chat::ChatPage;
use crate::ui::market::MarketPage;
use crate::ui::memory_view::MemoryViewPage;
use crate::ui::settings::SettingsPage;
use crate::ui::toolbox::ToolboxPage;
use dioxus::prelude::*;

/// Top-level tabs in the application.
#[derive(Debug, Clone, PartialEq)]
enum AppTab {
    Chat,
    Market,
    Toolbox,
    Memory,
    Settings,
}

impl AppTab {
    fn label(&self) -> &'static str {
        match self {
            AppTab::Chat => "💬 对话",
            AppTab::Market => "🏪 市场",
            AppTab::Toolbox => "🧰 工具箱",
            AppTab::Memory => "🧠 记忆",
            AppTab::Settings => "⚙️ 设置",
        }
    }
}

/// Main application component with tabbed navigation.
#[component]
pub fn App() -> Element {
    let mut active_tab = use_signal(|| AppTab::Chat);

    rsx! {
        div { class: "app-container",
            // ── Tab bar ──
            nav { class: "tab-bar",
                for tab in [AppTab::Chat, AppTab::Market, AppTab::Toolbox, AppTab::Memory, AppTab::Settings].iter() {
                    {
                        let tab = tab.clone();
                        let is_active = active_tab() == tab;
                        rsx! {
                            button {
                                class: if is_active { "tab-btn active" } else { "tab-btn" },
                                onclick: move |_| active_tab.set(tab.clone()),
                                "{tab.label()}"
                            }
                        }
                    }
                }
            }

            // ── Content area ──
            div { class: "tab-content",
                match active_tab() {
                    AppTab::Chat => rsx! { ChatPage {} },
                    AppTab::Market => rsx! { MarketPage {} },
                    AppTab::Toolbox => rsx! { ToolboxPage {} },
                    AppTab::Memory => rsx! { MemoryViewPage {} },
                    AppTab::Settings => rsx! { SettingsPage {} },
                }
            }
        }
    }
}
