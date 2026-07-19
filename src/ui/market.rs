use crate::ui::components::PluginCard;
use dioxus::prelude::*;

/// Sub-tabs within the Market page.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketTab {
    Artifacts,
    Skills,
    Mcp,
}

impl MarketTab {
    fn label(&self) -> &'static str {
        match self {
            MarketTab::Artifacts => "📦 组件",
            MarketTab::Skills => "🎯 技能",
            MarketTab::Mcp => "🔌 MCP",
        }
    }
}

/// Sample plugin data for demonstration.
#[derive(Debug, Clone)]
struct PluginInfo {
    name: String,
    description: String,
    version: String,
    stars: u32,
}

fn sample_plugins(tab: &MarketTab) -> Vec<PluginInfo> {
    match tab {
        MarketTab::Artifacts => vec![
            PluginInfo {
                name: "代码解释器".into(),
                description: "在沙箱中执行 Python 代码并返回结果".into(),
                version: "1.2.0".into(),
                stars: 3420,
            },
            PluginInfo {
                name: "文件浏览器".into(),
                description: "可视化浏览和管理项目文件".into(),
                version: "0.9.1".into(),
                stars: 1280,
            },
            PluginInfo {
                name: "图表生成器".into(),
                description: "基于 Mermaid/PlantUML 生成架构图".into(),
                version: "2.0.5".into(),
                stars: 2156,
            },
        ],
        MarketTab::Skills => vec![
            PluginInfo {
                name: "代码审查".into(),
                description: "自动审查代码质量、安全漏洞和最佳实践".into(),
                version: "1.5.0".into(),
                stars: 5800,
            },
            PluginInfo {
                name: "文档生成".into(),
                description: "从代码注释自动生成 API 文档".into(),
                version: "2.1.3".into(),
                stars: 4320,
            },
            PluginInfo {
                name: "测试生成".into(),
                description: "根据函数签名自动生成单元测试".into(),
                version: "0.8.0".into(),
                stars: 1950,
            },
        ],
        MarketTab::Mcp => vec![
            PluginInfo {
                name: "GitHub MCP".into(),
                description: "连接 GitHub API，管理仓库、Issue 和 PR".into(),
                version: "3.0.1".into(),
                stars: 7200,
            },
            PluginInfo {
                name: "Database MCP".into(),
                description: "连接 PostgreSQL/MySQL 数据库查询".into(),
                version: "1.0.0".into(),
                stars: 3100,
            },
            PluginInfo {
                name: "Slack MCP".into(),
                description: "通过 Slack API 发送消息和管理频道".into(),
                version: "0.7.2".into(),
                stars: 890,
            },
        ],
    }
}

/// Plugin marketplace page with three sub-tabs.
#[component]
pub fn MarketPage() -> Element {
    let mut active_subtab = use_signal(|| MarketTab::Artifacts);
    let mut search_query = use_signal(String::new);

    rsx! {
        div { class: "market-page",
            h2 { class: "page-title", "🏪 插件市场" }

            // ── Search bar ──
            div { class: "market-search",
                input {
                    class: "search-input",
                    r#type: "text",
                    placeholder: "🔍 搜索插件...",
                    value: "{search_query}",
                    oninput: move |evt: FormEvent| search_query.set(evt.value()),
                }
            }

            // ── Sub-tab bar ──
            nav { class: "sub-tab-bar",
                for tab in [MarketTab::Artifacts, MarketTab::Skills, MarketTab::Mcp].iter() {
                    {
                        let tab = tab.clone();
                        let is_active = active_subtab() == tab;
                        rsx! {
                            button {
                                class: if is_active { "sub-tab-btn active" } else { "sub-tab-btn" },
                                onclick: move |_| active_subtab.set(tab.clone()),
                                "{tab.label()}"
                            }
                        }
                    }
                }
            }

            // ── Plugin grid ──
            div { class: "plugin-grid",
                {
                    let plugins = sample_plugins(&active_subtab());
                    let query = search_query().to_lowercase();
                    let filtered: Vec<&PluginInfo> = if query.is_empty() {
                        plugins.iter().collect()
                    } else {
                        plugins.iter().filter(|p| {
                            p.name.to_lowercase().contains(&query)
                                || p.description.to_lowercase().contains(&query)
                        }).collect()
                    };

                    if filtered.is_empty() {
                        rsx! {
                            div { class: "market-empty",
                                p { "😕 没有找到匹配的插件" }
                            }
                        }
                    } else {
                        rsx! {
                            for plugin in filtered.iter() {
                                PluginCard {
                                    key: "{plugin.name}",
                                    name: plugin.name.clone(),
                                    description: plugin.description.clone(),
                                    version: plugin.version.clone(),
                                    stars: plugin.stars,
                                    installed: false,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
