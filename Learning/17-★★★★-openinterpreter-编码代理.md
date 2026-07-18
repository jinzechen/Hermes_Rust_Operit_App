# 25 — openinterpreter (Codex CLI)：Rust 编码代理源码分析

> **仓库**：https://github.com/openinterpreter/openinterpreter (66,593⭐, Rust)  
> **UA Rust 分析**：16 nodes（13 个关键 crate Cargo.toml）  
> **备注**：已更名为 OpenAI Codex CLI  
> **Hermes_Rust_Operit_App 评分**：★★★★

---

## 一、UA Rust 发现的工作区结构

openinterpreter 现为 OpenAI Codex CLI，使用 Bazel 构建，其下 `codex-rs/` 包含 **100+ Rust crates**：

```
codex-rs/ → 100+ crates 的 Rust 工作区
├── 核心:
│   ├── cli/             — CLI 入口
│   ├── core/            — 核心引擎
│   ├── model-provider/  — LLM 提供者
│   └── tools/           — 工具系统
├── MCP 生态:
│   ├── codex-mcp/       — MCP 服务器
│   ├── mcp-server/      — MCP 服务
│   └── rmcp-client/     — Rust MCP 客户端 ★
├── 执行/沙盒:
│   ├── exec-server/     — 代码执行服务
│   ├── sandboxing/      — 沙盒隔离
│   ├── bwrap/           — bubblewrap 沙盒
│   └── shell-command/   — Shell 命令
├── 记忆/技能:
│   ├── skills/          — 技能系统
│   ├── memories/        — 记忆
│   └── state/           — 状态管理
├── 工具:
│   ├── file-search/     — 文件搜索
│   ├── file-system/     — 文件系统
│   ├── shell-command/   — Shell
│   └── tools/           — 工具注册
└── 其他:
    ├── app-server/      — 应用服务器
    ├── connectors/      — 连接器
    ├── prompts/         — Prompt 模板
    └── tui/             — 终端 UI
```

---

## 二、对 Hermes_Rust_Operit_App 的关键参考

| crate | 功能 | Hermes 对应 | 值得学习 |
|-------|------|-------------|---------|
| `codex-mcp` | MCP 服务器 | hermes-mcp | ★★★ |
| `rmcp-client` | Rust MCP 客户端（比自建更标准） | `mcp/client.rs` | ★★★★ |
| `sandboxing` | 沙盒执行 | ❌ 无 | ★★★★★ |
| `bwrap` | bubblewrap 沙盒 | ❌ 无 | ★★★★ |
| `exec-server` | 代码执行服务 | code_execution.rs | ★★★ |
| `model-provider` | LLM 提供者抽象 | provider.rs | ★★★ |
| `tools` | 工具注册 | tool_registry.rs | ★★★ |

### 关键发现：rmcp-client

OpenAI 的实现比 Hermes 自建的 `mcp/client.rs` (14.4KB) 更标准：

```
codex-rmcp-client
├── 完整的 JSON-RPC 2.0 实现
├── stdio 传输支持
├── SSE 传输支持（Hermes 不支持）
└── 流式工具调用
```

### 关键发现：sandboxing

```
codex-sandboxing
├── Linux 命名空间隔离
├── Docker 沙盒
├── bwrap（bubblewrap）沙盒
└── 资源限制（CPU/内存/网络）
```

---

## 三、评分：★★★★

openinterpreter 的 Rust 工作区（100+ crates）规模远超 Hermes（18 crates），但其 MCP 客户端（rmcp-client）和沙盒（sandboxing）实现值得 Hermes 参考。
