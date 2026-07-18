# TabbyML — 自托管代码助手 UA 源码分析

> **UA Rust 分析**：Cargo.toml (2.8KB+2.6KB)  
> **仓库**：https://github.com/TabbyML/tabby (33K⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★（candle 推理设计参考）

---

## 一、架构

```
tabby (33K⭐, Rust workspace)
├── Cargo.toml (2.8KB) — workspace
├── crates/tabby/ — AI 代码补全核心
│   ├── Cargo.toml (2.6KB)
│   └── 依赖: candle, tokenizers, axum, reqwest
├── cli/ — CLI 入口
├── server/ — HTTP API
└── web/ — React UI
```

## 二、对 Hermes 的作用

| 能力 | TabbyML | Hermes |
|------|---------|--------|
| 推理后端 | candle | candle (已有) |
| 补全 API | axum | axum (可加) |
| 模型管理 | hf-hub | hf-hub (已有) |

**关键**：TabbyML 的 candle 推理后端设计与 Hermes 的 EmbedAnything 一致，可互相参考。

### 评分：★★★

TabbyML 的 candle 推理设计已覆盖，其核心价值（代码补全）与 Agent 应用有重叠但非核心。
