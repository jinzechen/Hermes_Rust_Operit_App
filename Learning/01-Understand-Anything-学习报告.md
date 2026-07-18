# Understand-Anything — 深度学习报告（含 UA Rust 分析）

> 上游：https://github.com/Egonex-AI/Understand-Anything (TypeScript, 74,987⭐)  
> Rust 版：https://github.com/jinzechen/Understand_Anything_Rust (Rust, v0.2.0)  
> 用途：代码库分析 & 知识图谱引擎 → Hermes_Rust_Operit_App 的「代码理解」能力

---

## 一、UA Rust 自身深度分析

**Understand_Anything_Rust（Rust 端口）** 的 UA 自分析结果：

| 指标 | 值 |
|------|----|
| 文件总数 | 1,327 |
| Rust 源文件 | 28 |
| 知识图谱节点 | **1,641** |
| 关系连线 | **1,623** |
| 架构层数 | 3 |
| 引导步数 | 6 |
| 复杂度 | Complex |

### 核心模块规模

| 模块 | 路径 | 行数 | 职责 |
|------|------|------|------|
| 报告生成 | `crates/ua-core/src/report.rs` | **915 行** | HTML/MD/JSON 报告引擎 |
| 仪表盘 | `crates/ua-core/src/dashboard.rs` | D3.js | 交互式力导向图 |
| 扫描器 | `crates/ua-core/src/scanner.rs` | — | 30+ 语言文件扫描 |
| 解析器 | `crates/ua-core/src/parser.rs` | — | 函数/类/trait/模块提取 |
| 图谱构建 | `crates/ua-core/src/graph.rs` | — | 21 节点类型 + 35 边类型 |
| 增量更新 | `crates/ua-core/src/incremental.rs` | — | Blake3 指纹比对 |
| CLI | `crates/ua-cli/src/main.rs` | 326 行 | scan/parse/build/json 命令 |

---

## 二、架构分析

### 三层管线

```
scan-project.mjs / ua scan     ← 文件扫描（30+ 语言）
    ↓
compute-batches.mjs            ← 分批（按复杂度分组）
    ↓
ParserRegistry::parse()        ← 源码解析（函数/类/trait/模块）
    ↓
build_graph()                  ← 知识图谱构建（21节点×35边）
    ↓
to_html() / to_markdown()      ← 报告输出（D3.js 仪表盘 / Markdown）
```

### 三种输出格式

| 格式 | 大小（对自身） | 内容 |
|------|---------------|------|
| JSON | 82KB | 完整知识图谱（节点+边+层级+导览） |
| HTML | 91KB | 自包含 D3.js 交互式仪表盘 |
| MD | 25KB | 人机可读 Markdown 报告 |

### 输出路径

默认 → `<project>/.understand-anything/`  
指定 → `ua build [path] [output] --format html|md|json`

---

## 三、与上游 TypeScript 版的对比

| 对比项 | 上游 TS (74k⭐) | Rust 版 (v0.2.0) |
|--------|----------------|-------------------|
| 运行时 | Node.js ≥22 + pnpm | 单一二进制 4.6MB |
| 启动时间 | ~2s | 即时 |
| 内存 | ~150MB | ~15MB |
| 速度 | 1x（基准） | **5x** 端到端 |
| HTML 报告 | ❌（仅 React SPA） | ✅ 自包含 D3.js |
| Markdown 报告 | ❌ | ✅ |
| MCP Server | ❌ | ✅ JSON-RPC |
| 增量更新 | Git diff | Blake3 指纹（更精确） |

---

## 四、对 Hermes_Rust_Operit_App 的集成

### 已实现（codebase_analyzer.rs）

```rust
// tools/codebase_analyzer.rs
// 通过 ua-core path dependency 直接调用
//
// ToolHandler: ToolHandler(0开销) > MCP(15ms) > Skills(50ms)
//
// 输入: {"path": "...", "format": "json|html|md"}
// 输出: 知识图谱 JSON / HTML / Markdown
```

### 待改进

1. **存储持久化** — 分析结果存入 redb GraphStore
2. **增量支持** — 利用 UA Rust 的 Blake3 指纹做增量分析
3. **Agent 集成** — 对话中自动触发"分析当前项目"

---

## 五、实际使用

```bash
# 扫描项目结构
ua scan /path/to/project
# → "28 files | Moderate"

# 构建 HTML 仪表盘
ua build /path/to/project output/report.html --format=html
# → "145 nodes, 134 edges, 3 layers, 6 tour steps"

# 构建 Markdown 报告
ua build /path/to/project output/report.md --format=md
```

---

## 六、三到五个可复用点

1. **ToolHandler 直接嵌入** — 0 开销，无需序列化
2. **双输出格式** — HTML 仪表盘 + MD 报告，人机双读
3. **增量分析** — Blake3 指纹只重分析变更文件
4. **MCP Server** — 可同时作为 MCP 工具暴露
5. **极低资源** — 15MB 内存，适合 Android
