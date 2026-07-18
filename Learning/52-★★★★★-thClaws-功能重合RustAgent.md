# thClaws — 与 Hermes 功能高度重合的 Rust AI Agent

> **UA Rust 分析**：10 nodes / crates/core 14.5KB Cargo.toml + 4.5KB lib.rs  
> **仓库**：https://github.com/thClaws/thClaws (1,166⭐, Rust)  
> **与 Hermes_Rust_Operit_App 的功能重合度**：★★★★★（几乎完全相同）

---

## 一、thClaws 与 Hermes_Rust_Operit_App 对照

| 功能 | thClaws | Hermes_Rust_Operit_App | 对比 |
|------|---------|----------------------|------|
| **语言** | Rust ✅ | Rust ✅ | 相同 |
| **UI** | 三 Tab: Chat/Files/Terminal | 四 Tab: 沙盒/Skills/MCP/我的 | 同模式 |
| **Agent 引擎** | crates/core (自定义) | hermes-agent-rs | 同源 |
| **LLM Provider** | 多供应商 | 多供应商 | 相同 |
| **MCP** | ✅ MCP 集成 | ✅ MCP + RMCP | 相同 |
| **Skills** | ✅ 技能系统 | ✅ SKILL.md | 相同 |
| **Agent Teams** | ✅ 多 Agent | hermes-agent-rs sub_agent | 相同 |
| **桌面端** | ✅ Tauri WebApp | ❌ 暂无 | thClaws 领先 |
| **Android** | ❌ 不支持 | ✅ 目标平台 | 本项目的优势 |
| **Stars** | 1,166⭐ | — | 验证了市场需求 |

---

## 二、thClaws 技术栈（对比学习）

```toml
# thClaws core 依赖
tokio / axum / serde / reqwest / futures
rustyline (CLI REPL) / clap
tungstenite (WebSocket)
include_dir (嵌入资源)

# 前端
frontend/package.json → Tauri 2 + Vue.js WebApp
```

---

## 三、核心模块（从 lib.rs 导出）

```
thclaws-core (0.100.0)
├── agent          — Agent 核心
├── agent_runtime  — 运行时
├── api_v1         — HTTP API (axum)
├── commands       — 命令系统
├── config         — 配置
├── cloud          — 云同步
├── mcp            — MCP 集成
├── tools          — 工具系统
├── skills         — 技能
└── auto_learn     — 自动学习
```

---

## 四、对 Hermes_Rust_Operit_App 的参考价值

| thClaws 设计 | 可参考点 | 优先级 |
|------------|---------|--------|
| 三 Tab UI (Chat/Files/Terminal) | UI 布局与 HermesApp 一致 | ★★★★★ |
| axum HTTP API | API 层设计 | ★★★★ |
| include_dir 嵌入前端资源 | 单二进制分发 | ★★★★ |
| auto_learn 自动学习 | 记忆增强 | ★★★ |
| cloud 云同步 | 可选功能 | ★★★ |

### 评分：★★★★★

thClaws 是 Rust 生态中与 Hermes_Rust_Operit_App 功能最重合的项目（1,166⭐ 验证了需求）。Android 原生支持是 Hermes 对 thClaws 的核心优势。
