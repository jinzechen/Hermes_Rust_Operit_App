# Rust 事件总线/消息队列生态

> **来源**：hermes-agent-rs 的 `hermes-bus` crate  
> **Hermes_Rust_Operit_App 评分**：★★★★（内部组件通信）

---

## 一、Rust 事件总线方案

| crate | 用途 | Hermes 适用 |
|-------|------|-----------|
| **tokio::sync::broadcast** | Tokio 内置广播 | ★★★★★ |
| **tokio::sync::mpsc** | Tokio 内置多生产者 | ★★★★★ |
| **eventbus** | 简单事件总线 | ★★★ |
| **dataloader** | 批量加载 | ★★ |

---

## 二、Hermes 中使用

```rust
use tokio::sync::broadcast;

// Agent 状态变更事件
#[derive(Clone)]
enum AgentEvent {
    ToolCalled(String),
    MemoryUpdated,
    TokenBudgetExceeded,
}

let (tx, mut rx) = broadcast::channel::<AgentEvent>(32);

// 发布事件
tx.send(AgentEvent::ToolCalled("browser_navigate".into()))?;

// 订阅事件
while let Ok(event) = rx.recv().await {
    match event {
        AgentEvent::MemoryUpdated => flush_to_db(),
        AgentEvent::TokenBudgetExceeded => degrade_model(),
        _ => {}
    }
}
```

### 评分：★★★★

Hermes 已有 `hermes-bus` crate 基于 tokio。不需要额外引入。
