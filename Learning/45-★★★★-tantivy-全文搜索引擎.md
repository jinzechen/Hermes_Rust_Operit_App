# tantivy — Rust 全文搜索引擎 UA 源码分析

> **UA Rust 分析**：lib.rs (48KB) / 5 Rust 源文件  
> **仓库**：https://github.com/quickwit-oss/tantivy (14K+⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★（Agent 记忆全文搜索）

---

## 一、UA Rust 发现的模块

```
tantivy (48KB lib.rs)
├── indexer — 索引器
├── query — 查询引擎（布尔/短语/模糊/正则）
├── schema — 模式定义（类似 SQL 表结构）
├── tokenizer — 分词器
├── collector — 结果收集器
├── directory — 存储目录
├── fastfield — 快速字段
├── postings — 倒排索引
├── aggregation — 聚合
└── index — 索引管理
```

---

## 二、Hermes 使用方案

```rust
use tantivy::{Index, Document, schema::*};

// 为 Agent 记忆创建全文索引
let mut schema_builder = Schema::builder();
schema_builder.add_text_field("content", TEXT | STORED);
schema_builder.add_text_field("source", STRING | STORED);
let schema = schema_builder.build();

let index = Index::create_in_dir("memory_index", schema)?;

// 搜索记忆
let searcher = index.reader()?.searcher();
let query = parser.parse_query("用户偏好 AND Rust")?;
let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
```

**Hermes 作用**：替代 grep + SQL 的模糊搜索，用于记忆检索、代码搜索。

### 评分：★★★★

tantivy 14K⭐ 验证了成熟度。Hermes 可用作记忆系统的全文搜索引擎（替代简单的字符串匹配）。
