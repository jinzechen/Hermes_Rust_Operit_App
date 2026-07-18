# 09 — qdrant：向量数据库源码分析

> **仓库**：https://github.com/qdrant/qdrant (33,364⭐, Rust)  
> **核心 crate**：`qdrant-client`（Rust 客户端库）  
> **Hermes_Rust_Operit_App 评分**：★★★★（语义记忆的可选后端）

---

## 一、核心架构

```
qdrant (Rust, 33K⭐)
├── lib/api/       → ★客户端库（可直接集成）
├── lib/segment/   → HNSW 向量索引引擎
└── src/           → 分布式服务端
```

## 二、集成方式

```toml
[dependencies]
qdrant-client = { version = "1.12", features = ["rustls"] }
```

```rust
use qdrant_client::Qdrant;
use qdrant_client::qdrant::SearchPoints;

let client = Qdrant::new("http://localhost:6334")?;
let results = client
    .search_points(&SearchPoints {
        collection_name: "memory".into(),
        vector: embedding,    // f32 向量
        limit: 10,
        with_payload: true,
        ..Default::default()
    })
    .await?;
```

## 三、对 Hermes_Rust_Operit_App 的作用

| 能力 | 当前 | 加 qdrant |
|------|------|----------|
| 记忆检索 | 精确 key 匹配 | 语义相似度搜索 |
| 代码分析 | stdout | 可向量化检索知识图谱节点 |
| RAG | 无 | 相关文档/代码语义检索 |

### Rust 复刻总结

```toml
[dependencies]
qdrant-client = { version = "1.12", features = ["rustls"] }
```

```rust
let client = Qdrant::new("http://localhost:6334")?;
let results = client.search_points(&SearchPoints {
    collection_name: "memory",
    vector: embedding,  // 来自 candle/EmbedAnything
    limit: 10,
    ..Default::default()
}).await?;
```

如果使用 tinycortex（内置混合检索），qdrant 可能不是必须。

### 评分：★★★★
