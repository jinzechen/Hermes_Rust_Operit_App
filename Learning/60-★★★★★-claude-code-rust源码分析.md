# claude-code-rust — Rust 重写 Claude Code UA 源码分析

> **UA Rust 分析**：main.rs (42KB) / lib.rs 20+ 模块  
> **仓库**：https://github.com/lorryjovens-hub/claude-code-rust (1,667⭐)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（高度重合，Rust 架构参考）

---

## 一、UA Rust 发现的源码

```
claude-code-rust (Tauri 桌面应用)
├── src-tauri/src/main.rs (42KB) — ★核心入口
├── src-tauri/src/lib.rs (1KB) — 模块导出
│   ├── bridge — 桥接层
│   ├── api — API 调用
│   ├── commands — Tauri 命令
│   ├── engine — ★Agent 引擎
│   ├── tools — ★工具系统
│   ├── research — 研究能力
│   ├── prompt — 提示词
│   ├── mcp — ★MCP 协议
│   ├── streaming — 流式响应
│   ├── execution — 代码执行
│   ├── task — 任务管理
│   ├── skills — ★技能系统
│   ├── git — Git 操作
│   ├── github — GitHub 集成
│   ├── config — 配置
│   ├── fs — 文件系统
│   ├── terminal — 终端
│   ├── process — 进程管理
│   ├── watcher — 文件监控
│   └── clipboard — ★剪贴板
├── main.rs:
│   ├── init_tracing() — 日志初始化
│   ├── init_opentelemetry() — 遥测
│   ├── cleanup_all_processes() — 进程清理
│   └── main() — 入口
└── src/ — TypeScript 前端
```

---

## 二、与 Hermes_Rust_Operit_App 对照

| 模块 | claude-code-rust | Hermes_Rust_Operit_App |
|------|-----------------|----------------------|
| **engine** | ✅ agent 引擎 | hermes-agent |
| **tools** | ✅ 工具系统 | hermes-tools |
| **mcp** | ✅ MCP 协议 | RMCP |
| **skills** | ✅ 技能 | hermes-skills |
| **clipboard** | ✅ | clipboard-rs |
| **terminal** | ✅ | terminal.rs |
| **git/github** | ✅ Git 集成 | git2 crate |
| **execution** | ✅ 代码执行 | code_execution.rs + wasmer |
| **streaming** | ✅ 流式 | agent_loop.run_stream() |
| **research** | ✅ 研究 | web_search |
| **prompt** | ✅ 提示词 | SKILL.md |
| **config** | ✅ 配置 | hermes-config |
| **fs** | ✅ 文件系统 | file.rs |
| **watcher** | ✅ 文件监控 | — |
| **task** | ✅ 任务 | todo |

**关键发现**：claude-code-rust 的模块划分几乎与 Hermes 完全一致，其架构可作为直接参考。

### 评分：★★★★★

claude-code-rust 与 Hermes_Rust_Operit_App 的功能模块几乎完全相同（engine/tools/mcp/skills/clipboard/git）。其 42KB main.rs 的架构设计值得深入学习。
