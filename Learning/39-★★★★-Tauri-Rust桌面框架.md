# Tauri — Rust 桌面框架源码分析

> **仓库**：https://github.com/tauri-apps/tauri (101,288⭐, Rust)  
> **对应**：Electron 的 Rust 替代  
> **Hermes_Rust_Operit_App 评分**：★★★★（桌面端方案）

---

## 一、架构

```
tauri (Rust, 101K⭐)
├── core/tauri/           — Rust 核心
├── core/tauri-utils/     — 工具库
├── core/tauri-runtime/   — 运行时抽象（WebView）
├── core/tauri-macros/    — 过程宏
├── tooling/cli/          — 构建 CLI
└── examples/             — 使用示例
```

---

## 二、对 Hermes_Rust_Operit_App 的作用

| 场景 | 方案 |
|------|------|
| Android 端 | Dioxus（已定） |
| 桌面端（可选） | Tauri |
| 移动端原生 | Dioxus native |

### 评分：★★★★

Hermes_Rust_Operit_App 主要面向 Android（Dioxus），但如果将来需要桌面端，Tauri 是 Rust 生态的标准方案。
