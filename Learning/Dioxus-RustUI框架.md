# Dioxus：Rust UI 框架源码深度分析

> **仓库**：https://github.com/DioxusLabs/dioxus (36,804⭐, Rust, Apache-2.0)  
> **版本**：0.8.0-alpha (40+ packages)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（UI 框架首选）

---

## 一、源码结构（Cargo.toml 30KB 工作区）

```
dioxus (workspace, 40+ packages)
├── dioxus/               → 统一入口 crate
├── packages/
│   ├── core/             → ★虚拟 DOM 核心
│   ├── core-types/       → 类型系统
│   ├── core-macro/       → rsx! 宏
│   ├── hooks/            → ★React 风格 hooks
│   ├── html/             → HTML 元素
│   ├── router/           → 路由
│   ├── signals/          → ★响应式信号
│   ├── stores/           → 全局状态
│   ├── native/           → ★Android/iOS 原生渲染
│   ├── native-dom/       → 原生 DOM 桥接
│   ├── web/              → Web 渲染
│   ├── desktop/          → Electron 桌面
│   ├── ssr/              → 服务端渲染
│   ├── fullstack/        → 全栈
│   ├── devtools/         → 开发者工具
│   ├── logger/           → 日志
│   ├── asset-resolver/   → 资源解析
│   └── rsx/              → RSX 解析器
├── Cargo.toml            → 30KB workspace 定义
```

---

## 二、Hermes_Rust_Operit_App 的关键包

| 包 | 用途 | Hermes 中做什么 |
|-----|------|----------------|
| **dioxus** | 统一入口 | `use dioxus::prelude::*` |
| **dioxus-core** | 虚拟 DOM | 声明式 UI 渲染 |
| **dioxus-hooks** | use_state, use_effect | 聊天状态管理 |
| **dioxus-native** | **Android 原生渲染** | Dioxus Blitz 渲染器 |
| **dioxus-signals** | 响应式信号 | 对话流更新 |
| **dioxus-router** | 路由 | 四 Tab 页面切换 |
| **dioxus-stores** | 全局状态 | 跨组件共享配置 |

### Android 构建命令

```bash
dx serve --platform android
# 或：
cargo build --target aarch64-linux-android
```

---

## 三、Dioxus 使用方法（Hermes UI 参考）

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

fn App(cx: Scope) -> Element {
    let messages = use_state(cx, || vec![]);

    cx.render(rsx! {
        div { class: "chat-container",
            h1 { "Hermes Rust" }
            messages.iter().map(|msg| rsx! {
                div { class: "message",
                    p { "{msg}" }  // 可选中可复制
                }
            })
            button {
                onclick: move |_| messages.push("新消息".to_string()),
                "发送"
            }
        }
    })
}
```

### 四 Tab 商店页面

```rust
fn StoreView(cx: Scope) -> Element {
    let tab = use_state(cx, || 0);
    cx.render(rsx! {
        div { class: "tabs",
            button { onclick: move |_| tab.set(0), "沙盒" }
            button { onclick: move |_| tab.set(1), "Skills" }
            button { onclick: move |_| tab.set(2), "MCPs" }
            button { onclick: move |_| tab.set(3), "我的" }
        }
        match *tab {
            0 => rsx!{ SandboxView {} }
            1 => rsx!{ SkillsView {} }
            2 => rsx!{ McpView {} }
            3 => rsx!{ ProfileView {} }
        }
    })
}
```

---

## 四、评分：★★★★★

Dioxus 是 Hermes 的 UI 方案唯一推荐。36K⭐ 验证了成熟度，Android 原生支持（dioxus-native Blitz），声明式 UI 适合聊天界面。
