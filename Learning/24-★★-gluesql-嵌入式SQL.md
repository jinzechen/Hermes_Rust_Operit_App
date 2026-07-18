# gluesql：嵌入式 SQL 数据库分析

> **仓库**：https://github.com/gluesql/gluesql (3,093⭐, Rust, Apache-2.0)  
> **核心**：多功能嵌入式 SQL 引擎（SQL+Query Builder）  
> **Hermes_Rust_Operit_App 评分**：★★（不推荐，redb更轻量）

---

## 一、源码结构

```
gluesql (workspace)
├── core/           → SQL 解析器(sqlparser-rs)+执行层
├── storage/
│   ├── memory/     → 内存存储
│   ├── shared-memory/ → 共享内存
│   └── json/       → JSON/JSONL 文件存储
├── pkg/            → 语言绑定
└── examples/
```

### 依赖

```toml
[dependencies]
sqlparser = "0.52"  # SQL 解析
serde / serde_json   # 序列化
chrono               # 日期时间
rust_decimal         # 高精度
im = "15"            # 不可变数据结构
```

---

## 二、集成可行性

```rust
use gluesql::*;

let storage = MemoryStorage::default();
let mut glue = Glue::new(storage);
let rows = glue.execute("SELECT id, name FROM Foo;")?;
```

### 对比 redb

| 维度 | gluesql | redb（Hermes已有） |
|------|---------|-------------------|
| 查询方式 | SQL | 直接 API |
| 性能 | 中等（SQL 解析开销） | 极高（零拷贝 mmap） |
| 大小 | 50K 行 | 15K 行 |
| 适用场景 | 需要 SQL 查询 | KV 存储 |

Hermes 已有 redb 做 KV 存储。如果需要 SQL 查询，tinycortex 已内置 SQLite。
