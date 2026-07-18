# 25 — MCP Rust 生态 学习报告

> **核心**：Rust 原生 MCP 协议库生态调研  
> **Hermes 集成现状**：✅ 已自建 `mcp/client.rs` (14.4KB) JSON-RPC 实现

---

## 一、Rust MCP 生态概览

| crate | 用途 | 成熟度 | Hermes 对比 |
|-------|------|--------|-------------|
| **rmcp** | Rust MCP 协议实现 | Alpha | Hermes 自建更轻量 |
| **mcp-core** | MCP 核心类型定义 | 实验性 | 可参考类型设计 |
| **mcp-sdk** | MCP 开发套件 | 实验性 | 过重 |
| **rust-mcp-sdk** | MCP 服务器框架 | 初期 | 可参考 |

**Hermes 自建优势**：
- 零外部依赖（仅 serde_json）
- 专为 stdio 传输优化
- 与 ToolHandler 无缝集成

## 二、Hermes 的 MCP 实现

`mcp/client.rs` (14.4KB) 已实现：

```
MCP Client
├── connect(stdio)     → 启动 MCP 子进程
├── list_tools()       → 获取工具列表
├── call_tool(name)    → 调用工具
├── notifications()    → 处理通知
└── disconnect()       → 清理
```

## 三、对 Hermes_Rust_Operit_App 的作用

Operit_MCPS 的 9 个 MCP 插件全部通过此客户端通信：

| 插件 | 类型 | 协议 |
|------|------|------|
| obscura | 浏览器 | MCP stdio |
| agentic_vision | 视觉 | MCP stdio |
| rust_mcp_filesystem | 文件系统 | MCP stdio |
| 余下 6 个 | 各种工具 | MCP stdio |

## 四、三到五个可复用点

1. **自建 > 外部库** — 零依赖更可控
2. **Stdio 传输** — 最轻量的进程间通信
3. **标准协议** — 与任何 MCP 兼容服务器互通
4. **异步支持** — Hermes 的 tokio 与 MCP stdio 天然适配
