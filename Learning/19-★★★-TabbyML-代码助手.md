# 13 — TabbyML：自托管代码助手

> **仓库**：https://github.com/TabbyML/tabby (33,728⭐, Rust)  
> **核心能力**：自托管 AI 代码补全（GitHub Copilot 替代）  
> **Hermes_Rust_Operit_App 评分**：★★★（可选集成，非核心）

---

## 一、架构

```
tabby (Rust, 33K⭐)
├── cli/       → 命令行
├── server/    → HTTP API（补全+聊天）
├── core/      → candle/llama.cpp 推理
├── web/       → React UI
└── lib/       → 共享库
```

## 二、对 Hermes_Rust_Operit_App 的作用

| 可复用点 | 说明 |
|----------|------|
| Candle 推理后端 | 与 EmbedAnything 同方案 |
| 补全 API | 可为 Hermes 添加代码补全能力 |

### Rust 复刻总结

TabbyML 的 candle 推理后端设计与 Hermes 已有的 EmbedAnything 一致：

```rust
// 复用 Hermes 已有的 candle 推理
// TabbyML 的代码补全 API 可作为可选功能
```

### 评分：★★★
