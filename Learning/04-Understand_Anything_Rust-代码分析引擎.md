# 04 — Understand_Anything_Rust：代码分析引擎源码分析

> **仓库**：https://github.com/jinzechen/Understand_Anything_Rust (用户自建, Rust)  
> **UA Rust 自分析数据**：1,641 nodes / 1,623 edges / 3 layers / 28 Rust 文件  
> **Hermes_Rust_Operit_App 评分**：★★★★★（已集成，核心分析工具，本报告即用它生成）

---

## 一、源码结构（UA Rust 自分析）

```
Understand_Anything_Rust/
├── crates/
│   ├── ua-core/       → 核心库（扫描/解析/图谱/报告）
│   │   ├── src/
│   │   │   ├── lib.rs          (1,731行)  — 引擎入口
│   │   │   ├── scanner.rs      — 文件扫描 (30+ 语言)
│   │   │   ├── parser/         — 源码解析器
│   │   │   ├── graph.rs        — 知识图谱构建 (21节点×35边)
│   │   │   ├── report.rs       (915行)  — HTML/MD 报告生成
│   │   │   ├── dashboard.rs    — D3.js 交互式仪表盘
│   │   │   ├── incremental.rs  — Blake3 增量更新
│   │   │   ├── types.rs        — 类型系统
│   │   │   └── agent.rs        — Agent 分派
│   │   └── Cargo.toml
│   ├── ua-cli/        → CLI 命令行
│   │   └── src/main.rs (326行) — scan/parse/build/json 4 命令
│   └── ua-mcp/        → MCP 服务器
│       └── src/main.rs — JSON-RPC stdio MCP
```

---

## 二、核心实现

### 扫描器（scanner.rs）

```rust
pub fn scan_project(root: &Path) -> Result<ScanResult> {
    // 1. walkdir 遍历文件
    // 2. 按扩展名分类语言（30+ 种）
    // 3. 排除 .gitignore / target/
    // 4. 返回 ScanResult { files: [...], stats: {...} }
}
```

### 解析器（parser/）

```rust
pub struct ParserRegistry {
    // Rust 解析器用 regex（无需 tree-sitter）
    // 提取: 函数 / 结构体 / trait / impl / 导入
}

pub fn parse(path: &Path) -> Result<ParsedFile> {
    // 1. 按扩展名选择解析器
    // 2. regex 提取 definitions + imports
    // 3. 返回 ParsedFile { definitions: [...], imports: [...] }
}
```

### 知识图谱构建（graph.rs）

```rust
pub fn build_graph(root: &Path, scan: &ScanResult, parsed: &[ParsedFile]) -> KnowledgeGraph {
    // 21 种节点类型: File, Function, Struct, Trait, Module, Directory...
    // 35 种边类型: Contains, Imports, Calls, Defines, Extends...
    // 分层: 按目录结构自动分层
    // 导览: 自动生成学习路径
}
```

### 报告生成（report.rs, 915 行）

```rust
pub fn to_html(graph: &KnowledgeGraph) -> String  // D3.js 交互仪表盘
pub fn to_markdown(graph: &KnowledgeGraph) -> String  // 人机可读
// 内部包含: html_header / html_section_overview / html_section_layers
//          html_section_file_inventory / html_section_imports
//          md_title / md_section_overview / md_section_layers ...
```

---

## 三、三种输出格式

| 输出 | 代码路径 | 内容 |
|------|---------|------|
| JSON | serde_json 序列化 | 完整知识图谱 nodes/edges/layers/tour |
| HTML | dashboard.rs + D3.js | 交互式力导向图 + 搜索 + 过滤 + 导航 |
| MD | report.rs markdown 函数 | 分层结构 + 文件清单 + 导入图 + 导览 |

---

## 四、对 Hermes_Rust_Operit_App 的集成

### 已集成

```rust
// Cargo.toml
ua-core = { path = "../Understand_Anything_Rust/crates/ua-core" }

// tools/codebase_analyzer.rs
use ua_core::{scanner, parser, graph};
```

### 可扩展

| 能力 | 当前状态 | 可改进 |
|------|---------|--------|
| 知识图谱存储 | 仅 stdout | → 存入 redb/qdrant |
| Agent 主动分析 | 手动触发 | → 自动分析当前项目 |
| 增量更新 | Blake3 指纹 | → 自动检测变更 |

### 评分：★★★★★

UA Rust 是 Hermes_Rust_Operit_App 的**代码理解能力核心**。本报告的所有 UA 分析数据（700 nodes、18 crates、函数签名等）都由它生成。
