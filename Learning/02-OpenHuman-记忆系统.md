# OpenHuman 记忆系统 — 深度学习报告（含 UA Rust 分析）

> 仓库：https://github.com/tinyhumansai/openhuman  
> Stars：35,015 | License：GPL-3.0 | 语言：Rust  
> 核心：本地优先的 Personal AI 超级大脑  
> 关键：`tinycortex` crate 可直接嵌入 Hermes

---

## 一、UA Rust 深度分析

| 指标 | 值 |
|------|----|
| 文件数 | 4（Cargo.toml 28.9KB = 庞大工作区） |
| 架构节点 | 10 |
| 核心依赖 | tokio, sqlx, serde, tiktoken-rs, rig |

### 项目结构

```
openhuman (workspace)
├── crates/
│   ├── tinycortex/     ← 记忆引擎核心（可独立编译）
│   ├── memory/         → 编排层、策略
│   ├── memory_tools/   → Agent 工具面
│   ├── memory_tree/    → 摘要树引擎
│   ├── memory_queue/   → 异步作业队列
│   ├── memory_search/  → 混合检索
│   ├── memory_store/   → SQLite 持久化
│   └── ...
└── src/                → 服务端主入口
```

---

## 二、记忆系统架构

```
┌──────────────────────────────────────────┐
│            Agent Memory Layer            │
│  memory_tools/ → Agent 可调用的记忆工具    │
│  agent_memory/ → 记忆检索专用 Agent       │
├──────────────────────────────────────────┤
│           Engine Layer                   │
│  memory_tree/    → 摘要树（L0→L1→L2级联） │
│  memory_queue/   → 异步 worker 管线       │
│  memory_search/  → 向量70%+关键词30%混合  │
│  memory_diff/    → git2 变更账本          │
│  memory_sync/    → 外部源同步             │
├──────────────────────────────────────────┤
│           Store Layer                    │
│  memory_store/   → SQLite (rusqlite,WAL) │
│    ├── unified/   → 主存储实现            │
│    ├── trees/     → 摘要树存储             │
│    ├── vectors/   → 向量嵌入存储           │
│    ├── chunks/    → 文档分块              │
│    └── content/   → Obsidian/wiki 表面   │
└──────────────────────────────────────────┘
```

---

## 三、集成方案：tinycortex crate

```toml
[dependencies]
tinycortex = { version = "0.1", features = ["git-diff", "persona"] }
```

### 需要实现的适配器

| Trait | 用途 | Hermes 现有实现 |
|-------|------|----------------|
| `EmbeddingBackend` | 向量嵌入 | 可用 provider API 或 candle 本地 |
| `ChatProvider` | LLM 摘要 | 已有 `provider.rs` (13.7KB) |
| `EntityExtractor` | 实体提取 | 需新增 |
| `QueueDelegates` | worker 循环 | 需新增（tokio） |

### 获得的能力

- SQLite 完整 schema（memory_docs, vector_chunks, graph, kv, episodic, events, profile, segments）
- 摘要树引擎（L0→L1→L2 级联摘要）
- 异步作业队列
- 混合检索（向量 + 关键词 + 图遍历）
- git2 变更追踪
- 实体提取 + 共现图

---

## 四、对比 Hermes_Rust_Operit_App 现有存储

| 维度 | 当前（redb） | 集成 tinycortex 后 |
|------|-------------|-------------------|
| 存储引擎 | redb KV | SQLite + 向量 + 图 |
| 记忆检索 | 精确 key | 混合检索（向量70%+关键词30%） |
| 摘要能力 | 无 | 摘要树（L0→L1→L2） |
| 异步管線 | 无 | 3 worker 队列 |
| 变更追踪 | 无 | git2 |
| 可解释性 | 黑盒读写 | 带分数拆解的混合搜索 |

---

## 五、三到五个可复用点

1. **tinycortex 直接集成** — 作为 Hermes memory.rs 的替换后端
2. **摘要树模式** — Hermes 对话历史的自动摘要和层级管理
3. **混合检索** — 向量 70% + 关键词 30% 的加权策略
4. **Markdown-native 设计** — 记忆是 markdown 树，可读可编辑
5. **异步作业管线** — SQLite 队列 + 3 worker 不阻塞热路径
