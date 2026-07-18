# 05 — Qdrant 向量搜索 深度学习报告

> **UA Rust 分析时间**：2026-07-18  
> **项目**：https://github.com/qdrant/qdrant (33,364⭐, Rust)  
> **核心**：高性能向量数据库（HNSW 索引）  
> **Hermes 集成现状**：❌ 未集成，但已有 redb KV 存储

---

## 第一步：UA Rust 深度扫描

```bash
ua scan analysis/qdrant → 5 文件（含 lib/api Cargo.toml + lib.rs）
ua build → 9 节点 / 4 边
```

## 第二步：核心发现

### qdrant-client 集成

```toml
[dependencies]
qdrant-client = { version = "1.12", features = ["rustls"] }
```

API 极简：

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

## 第三步：对 Hermes_Rust_Operit_App 的作用

| 当前 Hermes | 加 qdrant 后 |
|-------------|-------------|
| memory 精确 key 匹配 | 语义相似度搜索（向量） |
| 无向量索引 | HNSW ANN 索引 |
| 对话线性存储 | 向量+时间混合排序 |
| 代码分析仅 stdout | 可向量化检索 |

## 第四步：三到五个可复用点

1. **语义记忆** — 替代 memory.rs 的精确 key 匹配
2. **代码分析索引** — UA Rust 知识图谱节点向量化后检索
3. **qdrant-client** — 轻量 gRPC 客户端，Android 可用
4. **混合搜索** — 向量 70% + 标量过滤 30%
