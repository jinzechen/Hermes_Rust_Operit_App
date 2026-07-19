use dioxus::prelude::*;

/// Information about a single tool.
#[derive(Debug, Clone)]
struct ToolInfo {
    name: String,
    icon: &'static str,
    description: &'static str,
}

/// The 9 built-in tools displayed as cards.
fn builtin_tools() -> Vec<ToolInfo> {
    vec![
        ToolInfo {
            name: "文件管理".into(),
            icon: "📁",
            description: "浏览、编辑和管理文件系统",
        },
        ToolInfo {
            name: "终端".into(),
            icon: "💻",
            description: "内置终端模拟器",
        },
        ToolInfo {
            name: "Shell 执行".into(),
            icon: "⚡",
            description: "运行 Shell 命令和脚本",
        },
        ToolInfo {
            name: "权限管理".into(),
            icon: "🔐",
            description: "管理应用权限和沙箱",
        },
        ToolInfo {
            name: "UI 调试".into(),
            icon: "🔍",
            description: "检查和调试 UI 组件",
        },
        ToolInfo {
            name: "Logcat".into(),
            icon: "📋",
            description: "查看系统和应用日志",
        },
        ToolInfo {
            name: "SQL 查询".into(),
            icon: "🗄️",
            description: "数据库查询和管理工具",
        },
        ToolInfo {
            name: "Markdown".into(),
            icon: "📝",
            description: "Markdown 编辑和预览",
        },
        ToolInfo {
            name: "语音".into(),
            icon: "🎤",
            description: "语音识别和合成",
        },
    ]
}

/// A single tool card in the grid.
#[component]
fn ToolCard(name: String, icon: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "tool-card",
            span { class: "tool-icon", "{icon}" }
            span { class: "tool-name", "{name}" }
            p { class: "tool-desc", "{description}" }
        }
    }
}

/// Toolbox page — displays all built-in tools in a grid layout.
#[component]
pub fn ToolboxPage() -> Element {
    let tools = builtin_tools();

    rsx! {
        div { class: "toolbox-page",
            h2 { class: "page-title", "🧰 工具箱" }
            p { class: "page-subtitle", "内置工具集，点击启动对应功能" }

            div { class: "tool-grid",
                for tool in tools.iter() {
                    ToolCard {
                        key: "{tool.name}",
                        name: tool.name.clone(),
                        icon: tool.icon,
                        description: tool.description,
                    }
                }
            }
        }
    }
}
