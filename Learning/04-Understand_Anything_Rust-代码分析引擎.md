# 04 — Understand_Anything_Rust：代码分析引擎源码深度分析

> **仓库**：https://github.com/jinzechen/Understand_Anything_Rust (用户自建, Rust)  
> **UA Rust 自分析**：1,641 nodes / 1,623 edges / 3 layers / 28 Rust 文件  
> **Hermes_Rust_Operit_App 评分**：★★★★★（已通过 path dep 集成）

---

## 一、源码结构（从实际源码读出来的）

```
crates/ua-core/src/
├── lib.rs          (1,731行) — 引擎入口，pub mod 声明
├── scanner.rs      (1个pub函数) — 文件扫描
├── parser/
│   └── mod.rs      — 解析器注册
├── graph.rs        (6个pub函数) — 知识图谱构建
├── report.rs       (26个函数) — HTML/MD 报告
├── dashboard.rs    — D3.js 仪表盘
├── incremental.rs  — Blake3 指纹
├── types.rs        (28个类型) — 所有数据结构
└── agent.rs        — Agent 分派

crates/ua-cli/src/main.rs (326行) — CLI 入口
crates/ua-mcp/src/main.rs — MCP 服务器
```

---

## 二、核心数据结构（types.rs）

### 28 个类型定义

```
NodeType (enum, 12种) → File | Directory | Function | Struct | Trait | Impl | Module | ...
EdgeType (enum, 8种)  → Contains | Imports | Calls | Defines | Extends | ...
GraphNode (struct)    → id / node_type / file_path / summary / complexity / line_count
GraphEdge (struct)    → source / target / edge_type / label
Layer (struct)        → name / description / node_ids
TourStep (struct)     → title / description / node_ids
KnowledgeGraph (struct) → project / nodes / edges / layers / tour
ScanEntry (struct)    → path / file_category / language / line_count
ScanResult (struct)   → files / total_files / stats
```

---

## 三、core 模块实现细节

### scanner.rs：1 个 pub 函数扫描整个项目

```rust
pub fn scan_project(root: &Path) -> anyhow::Result<ScanResult> {
    // 1. walkdir 递归遍历 root 下所有文件
    // 2. 按扩展名映射到 30+ 种语言
    // 3. 排除 .gitignore / target/ / node_modules
    // 4. 统计每种语言的文件数
    // 5. 返回 ScanResult { files: Vec<ScanEntry>, stats: ScanStats }
}
```

### graph.rs：6 个 pub 函数构建知识图谱

```rust
pub fn build_graph(root, scan, parsed) -> KnowledgeGraph
    // 入口函数，调用下方所有函数组装 KnowledgeGraph

pub fn file_category_to_node_type(category) -> NodeType
    // 文件分类→图谱节点类型映射

pub fn build_directory_edges(scan) -> Vec<GraphEdge>
    // 目录→文件的 Contains 边

pub fn build_import_edges(parsed) -> Vec<GraphEdge>
    // 文件间的 Imports 边

pub fn build_layers(nodes) -> Vec<Layer>
    // 自动分层：按节点类型分 Core/Config/Docs

pub fn build_tour(nodes) -> Vec<TourStep>
    // 自动生成学习导览路径
```

### report.rs：26 个函数生成 HTML + MD

```
HTML 生成:
  to_html(graph) → 完整 HTML 仪表盘（调用 dashboard::generate）
  to_html_static(graph) → 无 JS 静态 HTML
  内部: html_header / html_footer / html_section_overview
        html_section_layers / html_section_file_inventory
        html_section_imports / html_section_dependency_tree
        html_section_tour / html_node_ref / render_html_tree
  
MD 生成:
  to_markdown(graph) → 完整 Markdown 报告
  内部: md_title / md_section_overview / md_section_layers
        md_section_file_inventory / md_section_imports
        md_section_dependency_tree / md_section_tour / render_md_tree
```

---

## 四、4 个命令行的实现（ua-cli/src/main.rs, 326行）

```rust
fn main() {
    match cmd {
        "scan"  → scan_project() + 打印摘要
        "parse" → scan → parse 每个文件 + 打印 defs/imports 数
        "build" → cmd_build() → scan → parse → build_graph → write_output
        "json"  → scan_project() → serde_json::to_string_pretty
    }
}

fn cmd_build(root, flags, args):
    format = parse_format_flag(args)   // --format html|md|json
    if incremental: build_incremental()
    else: build_full()

fn build_full():
    1. scan_project()
    2. ParserRegistry::default().parse() 每个文件
    3. build_graph()
    4. compute_fingerprints() → 写 meta.json
    5. write_output() → HTML/MD/JSON

fn build_incremental():
    1. 读老 meta.json
    2. scan_project()
    3. compute_fingerprints() vs 老指纹 → 找出变更文件
    4. 全量重分析（保证拓扑正确）
    5. 更新 meta.json
    6. write_output()
```

---

## 五、三种输出格式代码路径

```rust
fn write_output(root, graph, format, output_arg) {
    match format {
        "html" → to_html(graph) → 文件 → ".understand-anything/report.html"
        "md"   → to_markdown(graph) → 文件 → ".understand-anything/report.md"
        _      → serde_json::to_string_pretty → 文件 → ".understand-anything/knowledge-graph.json"
    }
}
```

---

## 六、对 Hermes_Rust_Operit_App 的集成

### 已集成（codebase_analyzer.rs）

```rust
// tools/codebase_analyzer.rs
// 直接调用 ua-core 的三个步骤：
let scan = ua_core::scanner::scan_project(path)?;
let registry = ua_core::parser::ParserRegistry::default();
let graph = ua_core::graph::build_graph(path, &scan, &parsed);
// 输出 JSON/HTML/MD
```

### 可改进点

| 能力 | 当前 | 目标 |
|------|------|------|
| 分析结果存储 | stdout → 丢失 | → 存入 redb GraphStore |
| Agent 触发 | 手动 | → 对话中自动"分析当前项目" |
| 增量分析 | Blake3 已实现 | → 自动检测变更 |

### 评分：★★★★★

UA Rust 是这套学习系统的核心工具。本报告所有分析数据都由其生成。已作为 `ua-core` path dep 集成在 Hermes_Rust_Operit_App 中。
