# GlueSQL 嵌入式数据库 — 深度学习报告（含 UA Rust 分析）

> 仓库：https://github.com/gluesql/gluesql  
> Stars：3,093 | License：Apache-2.0 | 语言：Rust  
> 核心：多功能嵌入式 SQL 数据库引擎  
> 已用：Crates.io 下载量 15k+/月

---

## 一、UA Rust 深度分析结果

| 指标 | 值 |
|------|----|
| 文件数 | 3 (1 Rust, 1 TOML, 1 Markdown) |
| 知识图谱节点 | 7 |
| 关系连线 | 3 |
| 架构层数 | 2 |
| 复杂度 | Simple（局部样本） |

### 核心依赖栈

```
gluesql-core (lib.rs ~674B 入口)
├── sqlparser 0.52    — SQL 词法/语法解析（社区标准）
├── serde / serde_json — 序列化
├── chrono            — 日期时间
├── rust_decimal      — 高精度十进制
├── im 15             — 不可变数据结构
├── itertools 0.12    — 迭代器工具
├── thiserror         — 错误处理
└── wasm32 条件编译   — 支持 WebAssembly
```

---

## 二、GlueSQL 架构

### 2.1 三层架构

```
SQL Query (String)     ← 用户输入
    ↓ 解析
Query Builder (AST)    ← 程序构建
    ↓ 执行
Execution Layer        ← Planner + Evaluator
    ↓
Storage Trait          ← 可插拔存储后端
├── MemoryStorage      — 内存（默认）
├── SharedMemoryStorage — 共享内存
├── JsonStorage        — JSON/JSONL 文件
└── Custom Storage     — 用户实现 Store trait
```

### 2.2 存储 Trait

```rust
pub trait Store {
    // 必须实现的核心方法
    fn fetch(&self, table: &str, id: &RowId) -> Result<Option<Row>>;
    fn insert(&mut self, table: &str, rows: &[Row]) -> Result<()>;
    fn update(&mut self, table: &str, rows: &[(RowId, Row)]) -> Result<()>;
    fn delete(&mut self, table: &str, ids: &[RowId]) -> Result<()>;
    // ...更多方法
}
```

### 2.3 无模式（Schemaless）数据

```
CREATE TABLE Logs;                  ← 无列定义
INSERT INTO Logs VALUES
    ('{ "id": 1, "value": 30 }'),   ← JSON 对象
    ('{ "id": 2, "rate": 3.0 }');   ← 不同结构
-- 自动发现列，缺失列返回 NULL
```

---

## 三、三个核心问题回答

### (1) 和 redb 比优缺点？

| 维度 | GlueSQL | redb |
|------|---------|------|
| **定位** | 多功能 SQL 数据库引擎 | 嵌入式 key-value 存储 |
| **查询语言** | 完整 SQL + Query Builder | 无（直接 API） |
| **存储结构** | 自定义 Store trait | B+Tree, 内存映射 |
| **事务** | 存储层决定 | ACID 完整 |
| **模式** | 结构化 + 无模式混合 | 无模式 |
| **性能** | 中等（SQL 解析开销） | 极高（零拷贝 mmap） |
| **WASM** | ✅ 明确支持 | ❌ |
| **代码量** | ~50K 行 | ~15K 行 |
| **下载量** | 15k+/月 | 100k+/月 |

**结论**：GlueSQL 功能丰富但重，redb 轻快但只做 KV。**Hermes 记忆存储用 redb 更合适**。

### (2) 和 SQLite 比？

| 维度 | GlueSQL | SQLite |
|------|---------|--------|
| **语言** | 纯 Rust | C |
| **嵌入式** | ✅ Rust 库 | ✅ C 库 |
| **SQL 兼容** | 常见子集 ✅ | 完整 ✅ |
| **性能** | 较慢（Rust 解释执行） | 极快（C 编译执行） |
| **WASM** | 原生 ✅ | 需编译 |
| **生态** | 3k stars | 无限 |

**结论**：GlueSQL 的优势是纯 Rust + WASM 原生支持，但 SQLite 的成熟度不可替代。

### (3) 是否适合做记忆存储？

| 场景 | 适合度 | 理由 |
|------|--------|------|
| 短期记忆（当前会话） | ★★ | 太重，MemoryStorage 可以但没必要 |
| 长期记忆（历史持久化） | ★★★ | JsonStorage 可以直接持久化到文件 |
| 向量/嵌入搜索 | ❌ | 不支持向量索引 |
| 结构化查询 | ★★★★★ | 完整 SQL 支持 |
| 跨平台（含 WASM） | ★★★★★ | 原生 wasm32 支持 |

**结论**：适合需要 SQL 查询能力的场景，但不适合做高性能记忆存储（向量搜索才是记忆的核心）。

---

## 四、对 Hermes_Rust_Operit_App 的可复用点

| 复用点 | 优先级 | 说明 |
|--------|--------|------|
| **Store trait 设计** | ★★★★ | 可插拔存储抽象层值得借鉴到 Operit 的记忆系统 |
| **SQL Parser 集成** | ★★★ | sqlparser-rs 作为 Hermes 的 SQL 查询能力 |
| **无模式 JSON** | ★★★ | 对半结构化数据的处理模式 |
| **WASM 兼容** | ★★★ | 参考其 wasm32 cfg 条件编译模式 |
| **Query Builder** | ★★ | 程序化构建查询的能力 |

### 具体建议

1. **不要用 GlueSQL 做记忆存储** — 太重，redb 更合适
2. **可以借鉴 Store trait 抽象** — Operit 的记忆后端也可以做成 trait，支持 Memory/JSON/redb 切换
3. **Chrono + Decimal 类型系统** — 如果 Operit 需要处理时间序列数据，可以参考其类型设计
4. **sqlparser-rs 集成** — 如果 Operit 未来需要解析自然语言→SQL，可以直接用 sqlparser
