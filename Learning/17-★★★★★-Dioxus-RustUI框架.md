# Dioxus — Rust UI 框架

> **仓库**：https://github.com/DioxusLabs/dioxus (36,804⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（Android UI 唯一方案）

---

Dioxus 是 Hermes_Rust_Operit_App 的 UI 方案。40+ 个 package 的工作区：

| 包 | 用途 |
|-----|------|
| `dioxus-core` | 虚拟 DOM 核心 |
| `dioxus-hooks` | use_state, use_effect |
| `dioxus-native` | **Android 原生渲染** |
| `dioxus-html` | HTML 元素 |
| `dioxus-router` | 路由 |
| `dioxus-signals` | 响应式信号 |
| `dioxus-desktop` | 桌面端 |

### 使用

```rust
fn App(cx: Scope) -> Element {
    let msgs = use_state(cx, || vec![]);
    cx.render(rsx!{
        div { class: "chat",
            msgs.iter().map(|m| rsx!(p { "{m}" }))
        }
    })
}
```

### 评分：★★★★★

Dioxus 是唯一支持 Android 原生的 Rust UI 框架，Hermes_Rust_Operit_App 的 UI 层。
