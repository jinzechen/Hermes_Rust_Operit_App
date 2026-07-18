# OpenHuman Memory System — 学习报告

> 项目：https://github.com/tinyhumansai/openhuman
> 关键发现：`tinycortex` crate 可直接整合
> 学习日期：2026-07-18

## 核心发现

OpenHuman（35k stars）的**记忆引擎已提取为独立 crate：`tinycortex`（v0.1.1）**，MIT 协议，可直接加到 Cargo.toml。

## 记忆系统架构

```
memory/          → 编排层、策略、RPC
memory_tools/    → Agent 工具面
agent_memory/    → 记忆检索 Agent
────────────────────────────────
memory_tree/     → 摘要树引擎（bucket-seal cascade）
memory_queue/    → 异步作业管线（SQLite 队列，3 worker）
memory_search/   → 混合检索（向量 70% + 关键词 30%）
memory_diff/     → git2 变更账本
memory_conversations/ → 对话转录（FTS5）
memory_sync/     → 外部源同步
────────────────────────────────
memory_store/    → SQLite（rusqlite, WAL 模式）
  ├── unified/   → 主实现
  ├── trees/     → 摘要树
  ├── vectors/   → 向量存储
  ├── chunks/    → 文档分块
  └── content/   → Obsidian/wiki 表面
```

## 可整合性

**直接方案**:
```toml
[dependencies]
tinycortex = { version = "0.1", features = ["git-diff", "persona"] }
```

需要实现的适配器 trait：
- `EmbeddingBackend` → 向量嵌入
- `ChatProvider/Summariser` → LLM 摘要
- `EntityExtractor` → 实体提取
- `QueueDelegates` → worker 循环

**获得的能力**（无需重写）：
- SQLite 完整 schema（memory_docs, vector_chunks, graph, kv, episodic, events, profile, segments）
- 摘要树引擎（L0→L1→L2 级联摘要）
- 异步作业队列
- 混合检索（向量 + 关键词 + 图遍历）
- git2 变更追踪
- 实体提取 + 共现图
- 来源标记（internal/external taint）

**关键设计原则**：
1. Local-first — 全部存在用户机器 SQLite
2. Markdown-native — 记忆是 markdown 树，不是黑盒向量
3. 可解释检索 — 带分数拆解的混合搜索
4. 异步管线 — 重活排队不阻塞热路径
