# 08 — openhuman：个人 AI 记忆系统源码分析

> **仓库**：https://github.com/tinyhumansai/openhuman (35,015⭐, Rust)  
> **核心 crate**：`tinycortex`（记忆引擎，可独立使用）  
> **Hermes_Rust_Operit_App 评分**：★★★★★（直接替换 memory.rs）

---

## 一、源码结构

```
openhuman (Rust, 35K⭐)
├── crates/
│   ├── tinycortex/       → ★独立记忆引擎（可直接 cargo add）
│   ├── memory/           → 编排层
│   ├── memory_tools/     → Agent 工具接口
│   ├── memory_tree/      → 摘要树引擎（L0→L1→L2 级联）
│   ├── memory_queue/     → 异步 worker 管线
│   ├── memory_search/    → 混合检索（向量70%+关键词30%）
│   ├── memory_store/     → SQLite 持久化
│   └── memory_sync/      → 外部源同步
└── src/                  → 主服务
```

## 二、tinycortex 记忆引擎

```rust
// 可直接引入
use tinycortex::{MemoryEngine, MemoryConfig};

let engine = MemoryEngine::new(MemoryConfig::default());
engine.store("user 偏好使用深色主题").await?;
let results = engine.recall("主题偏好").await?;  // 混合检索
```

## 三、对 Hermes_Rust_Operit_App 的作用

| 当前 memory.rs | 加 tinycortex 后 |
|---------------|-----------------|
| 精确 key 匹配 | 混合检索（向量+关键词） |
| 无摘要 | 摘要树 L0→L1→L2 |
| 同步执行 | 异步 worker 管线 |
| 1 种存储 | SQLite + 向量 + 图 |

### Rust 复刻总结

```toml
[dependencies]
tinycortex = { version = "0.1" }
```

替换 Hermes_Rust_Operit_App 的 core/memory.rs：

```rust
// 之前: memory: HashMap<String, MemoryEntry> → 精确 key 匹配
// 之后: engine = MemoryEngine::new(config) → 混合检索

let results = engine.recall("用户偏好").await?;
// 返回: 向量 70% + 关键词 30% 混合排序结果
```

获得能力：摘要树 L0→L1→L2、异步 worker 管线、SQLite+向量+图存储。

### 评分：★★★★★
