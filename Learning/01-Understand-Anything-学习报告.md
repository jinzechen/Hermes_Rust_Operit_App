# Understand-Anything 学习报告

> 项目：https://github.com/jinzechen/Understand-Anything  
> 学习日期：2026-07-18  
> 用途：为 Hermes_Rust_Operit_App 提供代码理解 & 知识图谱能力  

---

## 一、项目概述

Understand-Anything 是一个**多 Agent 知识图谱分析管线**，能将任意代码库 / LLM Wiki 分析为交互式知识图谱。

核心能力：
- 30+ 语言代码解析器（含 Rust tree-sitter extractor）
- 多 Agent 并行流水线（扫描→分析→建图→审核→导览）
- 交互式 Dashboard（React + ELK 力导向布局）
- 增量分析（fingerprint，只重分析变更文件）
- Diff 影响分析、语义搜索、业务域知识提取

## 二、安装状态

| 步骤 | 状态 | 路径 |
|------|------|------|
| 克隆仓库 | ✅ | `D:\Hermes_Agent_Desktop\Hermes_Download\Understand-Anything\` |
| pnpm 安装依赖 | ✅ | Node v22.22.3, pnpm 11.14.0 |
| tree-sitter 多语言解析器 | ✅ | Rust/TypeScript/Python/Go/Java/Kotlin/C++/C#/Ruby/PHP/Swift 等 30+ 语言 |
| 扫描 Hermes_Rust_Operit_App | ✅ | `scan-project.mjs` 返回 28 文件 / 0 过滤 / 复杂度 small |
| 生成知识图谱 | ✅ | 37 节点 / 46 边 / 3 层级 |
| Dashboard 服务 | ✅ | http://127.0.0.1:8765 (需 Vite build 后完整渲染) |

## 三、对 Hermes_Rust_Operit_App 的整合方案

### 3.1 内置「代码理解」工具 (CodebaseAnalyzerTool)

基于 Understand-Anything 的知识图谱数据模型（21 节点类型 + 35 边类型），实现 ToolHandler：

```
工具名: analyze_codebase
输入: path (项目路径), language (可选), incremental (增量)
输出: knowledge-graph.json
实现:
  Phase 1: 文件扫描 (tree-sitter 语言检测)
  Phase 2: 结构提取 (函数/类/trait/模块)
  Phase 3: 关系推导 (imports/contains/calls/depends_on)
  Phase 4: 输出 JSON → 存入 redb GraphStore
```

### 3.2 Rust tree-sitter Extractor

Understand-Anything 已有 TypeScript 版 Rust extractor (`plugins/extractors/rust-extractor.ts`)。

用 `tree-sitter` Rust crate 重写，编译进二进制，无需 Node.js：

```toml
[dependencies]
tree-sitter = "0.24"
tree-sitter-rust = "0.23"
```

### 3.3 知识图谱作为 Memory 增强

在 redb 中新增 `GraphStore` 表，存储分析结果。Agent 对话中可查询：
- "哪些模块与 auth 相关？" → 图遍历
- "修改 agent.rs 会影响哪些文件？" → 反向依赖分析
- "这个项目的整体架构？" → 层级视图

### 3.4 Dashboard 嵌入

方式一：Dioxus WebView 加载已构建的 Dashboard HTML  
方式二：作为独立的「代码理解」Tab 页

### 3.5 Skill 格式兼容

Understand-Anything 的 skill 定义格式：

```yaml
---
name: understand
description: Analyze a codebase to produce an interactive knowledge graph
argument-hint: ["[path] [--full|--auto-update...]"]
---
```

与 Operit 的 `.skill` 格式完全兼容。`PluginStore` 已支持 SKILL.md 扫描。

## 四、技术架构参考

### 数据分析流水线

```
scan-project.mjs          ← 文件扫描 (git ls-files / walkdir)
    │
    ▼
compute-batches.mjs       ← 分批 (按复杂度分组，平衡 LLM 负载)
    │
    ▼
file-analyzer.md (Agent)  ← 逐文件分析 (提取结构 + 依赖)
    │
    ▼
merge-batch-graphs.py     ← 合并批处理结果
    │
    ▼
graph-reviewer.md (Agent) ← LLM 审核 (补全缺失关系)
    │
    ▼
tour-builder.md (Agent)   ← 生成学习导览
    │
    ▼
knowledge-graph.json      ← 最终输出
```

### Dashboard 组件架构

```
App.tsx
├── GraphView.tsx         ← 力导向图 (ELK 布局)
├── DomainGraphView.tsx   ← 业务域横向图
├── KnowledgeGraphView.tsx ← LLM Wiki 聚类图
├── SearchBar.tsx         ← 模糊 + 语义搜索
├── FilterPanel.tsx       ← 按类型/层级/复杂度过滤
├── NodeInfo.tsx          ← 节点详情
├── CodeViewer.tsx        ← 代码预览
├── LearnPanel.tsx        ← 导览播放器
├── PersonaSelector.tsx   ← 角色切换 (新手/PM/专家)
├── ThemePicker.tsx       ← 主题切换
└── I18nContext.tsx       ← 多语言 (zh/en/ja/ko/ru)
```

## 五、实测数据

对 Hermes_Rust_Operit_App 的扫描结果：

```
filesScanned: 28
filteredByIgnore: 0
complexity: small

文件分布:
  code:   16 (src/*.rs)
  docs:   4  (README, ARCHITECTURE, HermesApp-Rust*.md)
  config: 5  (Cargo.toml, Cargo.lock, package.json, tsconfig, eslint)
  infra:  2  (.github/workflows/*.yml)
  script: 1  (build.sh)

语言分布:
  rust: 16, markdown: 4, toml: 2, json: 2, yaml: 1, typescript: 1, shell: 1
```

生成的知识图谱：
- **37 节点**：16 Rust 文件 + 4 文档 + 6 目录 + 5 配置 + 6 其他
- **46 边**：文件包含关系（目录→文件）
- **3 层级**：Rust Code / Documentation / Configuration
- **5 步导览**：README → lib.rs → agent.rs → tool_registry.rs → tools/*

## 六、下一步行动

1. [ ] 用 `tree-sitter` Rust crate 重写 `rust-extractor`
2. [ ] 实现 `CodebaseAnalyzerTool` (ToolHandler trait)
3. [ ] 在 redb 中添加 `GraphStore` 表
4. [ ] Agent 集成：对话中自动触发代码分析
5. [ ] Dashboard 通过 WebView 嵌入 Dioxus

---

*文档由 Hermes Agent 基于 Understand-Anything 源码分析自动生成*
