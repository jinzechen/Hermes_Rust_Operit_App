use dioxus::prelude::*;

/// A card component displaying plugin/artifact information.
#[component]
pub fn PluginCard(
    name: String,
    description: String,
    version: String,
    stars: u32,
    installed: bool,
) -> Element {
    let mut is_installed = use_signal(|| installed);

    let star_display = if stars >= 1000 {
        format!("{:.1}k", stars as f64 / 1000.0)
    } else {
        stars.to_string()
    };

    rsx! {
        div { class: "plugin-card",
            div { class: "plugin-card-header",
                span { class: "plugin-name", "{name}" }
                span { class: "plugin-version", "v{version}" }
            }
            div { class: "plugin-card-body",
                p { class: "plugin-desc", "{description}" }
            }
            div { class: "plugin-card-footer",
                span { class: "plugin-stars", "⭐ {star_display}" }
                button {
                    class: if is_installed() { "plugin-btn installed" } else { "plugin-btn install" },
                    onclick: move |_| is_installed.set(!is_installed()),
                    if is_installed() {
                        "✅ 已安装"
                    } else {
                        "📦 安装"
                    }
                }
            }
        }
    }
}
