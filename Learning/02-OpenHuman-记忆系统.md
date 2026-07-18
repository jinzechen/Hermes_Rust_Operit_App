# 02 — OpenHuman 记忆系统 深度学习报告

> **UA Rust 分析时间**：2026-07-18  
> **项目**：https://github.com/tinyhumansai/openhuman (35,015⭐, Rust)  
> **关键**：`tinycortex` crate 可独立嵌入 Hermes 记忆系统  
> **Hermes 集成现状**：✅ 已有 `memory.rs` (6KB) + `store/mod.rs` (10.9KB, redb)

---

## 第一步：UA Rust 深度扫描

```bash
# 扫描项目
ua scan /d/.../openhuman
# → 6 文件，4 种语言

# 构建知识图谱
ua build /d/.../openhuman output/openhuman-memory.json
ua build /d/.../openhuman output/openhuman-memory.md --format=md
ua build /d/.../openhuman output/openhuman-memory.html --format=html
```

### UA Rust 发现的项目结构

```
项目概览：
  文件数: 6
  知识图谱: 11 节点 / 6 边 / 3 层
  复杂度: Simple（局部样本，完整项目含 50+ crates）

UA 发现的架构分层:
├── Core Code (7 节点)
│   ├── src/lib.rs          (1,931 行) — 主入口
│   └── crates/tinycortex/  — 记忆引擎核心
├── Configuration (3 节点)
│   ├── Cargo.toml          (28.9KB = 庞大工作区)
│   └── crates/tinycortex/Cargo.toml
└── Documentation (1 节点)
    └── README.md           (19KB)
```

---

## 第二步：Cargo.toml 解析（28.9KB 工作区）

Cargo.toml 揭示了 OpenHuman 的真实规模（50+ crates）：

| 核心 crate | 用途 | Hermes 对应 |
|------------|------|-------------|
| `tinycortex` | **记忆引擎核心**（可独立使用） | `memory.rs` |
| `memory_store` | SQLite 持久化（rusqlite, WAL） | `store/mod.rs` redb |
| `memory_tree` | 摘要树引擎（L0→L1→L2 级联） | ❌ 无 |
| `memory_search` | 混合检索（向量 70% + 关键词 30%） | ❌ 纯 key 匹配 |
| `memory_queue` | 异步作业管线（3 worker） | ❌ 无 |
| `memory_diff` | git2 变更账本 | ❌ 无 |
| `memory_conversations` | 对话转录 FTS5 | ❌ 线性存储 |
| `memory_sync` | 外部源同步 | ❌ 无 |

---

## 第三步：对 Hermes_Rust_Operit_App 的作用分析

### 当前 Hermes 记忆系统

```
memory.rs (6KB)
├── MemoryEntry — 记忆条目结构
├── store()     — 存入记忆
├── recall()    — 检索记忆（精确 key 匹配）
└── forget()    — 删除记忆

store/mod.rs (10.9KB)
├── Store trait — 存储接口
├── MemoryStore — 记忆持久化
├── MessageStore — 聊天历史
└── CacheStore  — 缓存
```

### 集成 tinycortex 后的变化

```
当前: 精确 key 匹配，无向量搜索，无摘要，无异步
     ↓
集成后: 混合检索 + 摘要树 + 异步管线 + git 追踪
```

### 具体集成方案

```toml
# Cargo.toml 添加
tinycortex = { version = "0.1", features = ["git-diff", "persona"] }
```

需要实现的适配器：

| Trait | 用途 | Hermes 现有 |
|-------|------|-------------|
| `EmbeddingBackend` | 向量嵌入 | 可用 provider API |
| `ChatProvider` | LLM 摘要 | `provider.rs` (13.7KB) ✅ |
| `EntityExtractor` | 实体提取 | 需新增 |
| `QueueDelegates` | worker 循环 | 需新增（tokio） |

---

## 第四步：三到五个 Hermes 可复用点

| # | 可复用点 | 具体做法 |
|---|---------|----------|
| 1 | **tinycortex 替换 memory.rs** | 直接嵌入，获得完整记忆引擎 |
| 2 | **摘要树引擎** | 对话历史自动 L0→L1→L2 级联摘要 |
| 3 | **混合检索** | 向量 70% + 关键词 30% 比纯 key 匹配强太多 |
| 4 | **异步作业管线** | 重活（摘要、嵌入）排队不阻塞 Agent 热路径 |
| 5 | **可解释记忆** | Markdown-native，记忆可读可编辑，不是黑盒 |
