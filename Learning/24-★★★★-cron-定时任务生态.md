# cron 生态 — Rust 定时任务方案

> **来源**：hermes-agent-rs 的 `hermes-cron` crate + crates.io 生态  
> **Hermes_Rust_Operit_App 评分**：★★★★（已在 hermes-agent-rs 中，无需额外引入）

---

## 一、Rust cron 生态

| crate | 用途 | 在 Hermes 中 |
|-------|------|-------------|
| `tokio-cron-scheduler` | 基于 tokio 的 cron 调度器 | hermes-cron 底层 |
| `cron` | cron 表达式解析 | hermes-cron 底层 |
| `schedule` | 类 cron 调度 | 可选替代 |
| `delay-timer` | 延迟任务定时器 | 可选替代 |

---

## 二、在 Hermes 中的集成

hermes-agent-rs 已有 `hermes-cron` crate：

```rust
// 使用 hermes-cron
use hermes_cron::CronScheduler;

let scheduler = CronScheduler::new();
scheduler.add("0 */6 * * *", || {
    // 每 6 小时清理记忆
    memory_manager.cleanup()?;
}).await?;
```

### 评分：★★★★

hermes-cron 已在 hermes-agent-rs 中，Hermes_Rust_Operit_App 直接引入即可，无需单独找 cron 项目。
