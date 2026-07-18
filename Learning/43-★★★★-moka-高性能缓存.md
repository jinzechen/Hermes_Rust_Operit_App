# moka — Rust 高性能缓存库 UA 源码分析

> **UA Rust 分析**：lib.rs (12.7KB)  
> **仓库**：https://github.com/moka-rs/moka  
> **Hermes_Rust_Operit_App 评分**：★★★★（Token/API 结果缓存）

---

## 一、核心 API

```rust
use moka::sync::Cache;

// 同步缓存
let cache: Cache<String, String> = Cache::builder()
    .max_capacity(100)
    .time_to_live(Duration::from_secs(3600))
    .build();

cache.insert("api-key".into(), response);
let cached = cache.get("api-key");

// 异步缓存
use moka::future::Cache;
```

## 二、Hermes 中的应用

| 场景 | 缓存内容 | TTL | 作用 |
|------|---------|-----|------|
| LLM API 响应 | 相同 prompt 的响应 | 5m | 节省 Token |
| 搜索结果 | 搜索结果缓存 | 10m | 减少网络请求 |
| 模型列表 | Provider 模型列表 | 1h | 减少 API 调用 |
| 嵌入向量 | 文本→向量 | 1h | 加速记忆检索 |

### 评分：★★★★

moka 的缓存能力可以大幅降低 Agent 的 API 调用次数和 Token 消耗。
