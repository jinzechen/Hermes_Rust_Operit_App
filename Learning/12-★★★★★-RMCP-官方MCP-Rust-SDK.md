# RMCP — 官方 MCP Rust SDK 源码分析

> **仓库**：https://github.com/modelcontextprotocol/rust-sdk (3,639⭐, Rust)  
> **crate**：`rmcp` (crates.io)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（应替换自建 MCP 客户端）

---

## 一、UA Rust 发现

```
rmcp (6 Rust 源文件)
├── lib.rs        (1.3KB) — 入口，重导出
├── model/        — 协议类型（JSON-RPC 2.0）
├── service/      — 客户端/服务端
├── handler/      — Handler traits
├── transport/    — stdio + SSE 传输
├── error/        — 错误处理
└── task_manager/ — 任务管理
```

---

## 二、关键导出

```rust
pub use service::{RoleClient, RoleServer, ClientHandler, ServerHandler};
pub use service::{Peer, Service, ServiceError, ServiceExt};
pub use service::{serve_client, serve_server};
pub use transport::Transport;
pub use rmcp_macros::*;   // #[tool] 宏
```

---

## 三、Hermes 使用方式

```toml
# 替换自建 mcp/client.rs (14.4KB)
[dependencies]
rmcp = "0.8"
```

```rust
use rmcp::{ServiceExt, Service};
use rmcp::transport::stdio;

// MCP 客户端（替代自建实现）
let service = Service::new(transport)
    .serve(handler)
    .await?;

// 调用工具
let result = service
    .call_tool("filesystem_read", json!({"path": "/sdcard/test.txt"}))
    .await?;
```

### 评分：★★★★★

官方 MCP Rust SDK 比 Hermes 自建的 `mcp/client.rs` (14.4KB) 更标准化、更完整（支持 stdio + SSE 双传输）。Hermes_Rust_Operit_App 应改用 `rmcp` crate。
