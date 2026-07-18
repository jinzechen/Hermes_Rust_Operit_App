# fabric AI 模式库 — 深度学习报告（含 UA Rust 分析）

> 仓库：https://github.com/danielmiessler/fabric  
> Stars：43,117 | License：MIT | 语言：Go + Markdown (Patterns)  
> 核心：255 个 AI 模式 + 9 种策略的众包框架

---

## 一、UA Rust 深度分析结果

### 1.1 扫描概况

| 指标 | 值 |
|------|----|
| 文件数 | 16 (5 Markdown, 9 JSON, 1 Go, 1 其他) |
| 知识图谱节点 | 26 |
| 关系连线 | 21 |
| 架构层数 | 3 |
| 引导步数 | 6 |

### 1.2 核心文件

| 文件 | 大小 | 职责 |
|------|------|------|
| `README.md` | **47.6KB** | 完整文档、安装指南、策略说明 |
| `go.mod` | **8.2KB** | Go 依赖管理（200+ AI providers） |
| `data/patterns/create_conceptmap/system.md` | **4.9KB** | 概念地图 pattern 示例 |
| `data/patterns/summarize/system.md` | **960B** | 最基础的总结 pattern |
| `data/patterns/raw_query/system.md` | **327B** | 最简 raw prompt pattern |

### 1.3 结构分层

```
Project Root (26 nodes, 3 layers)
├── Documentation (5): README.md, etc.
├── Pattern Library (10): data/patterns/*/system.md
│   ├── summarize       — 内容总结
│   ├── raw_query       — 裸查询
│   └── create_conceptmap — 概念地图
└── Strategy Definitions (9): data/strategies/*.json
    ├── cot.json        — Chain of Thought
    ├── aot.json        — Atom of Thought
    ├── tot.json        — Tree of Thought
    ├── reflexion.json  — 反思
    ├── self-consistent.json — 自一致性
    ├── self-refine.json     — 自优化
    ├── ltm.json        — 由易到难
    ├── cod.json        — Chain of Draft
    └── standard.json   — 标准提示
```

---

## 二、Pattern 格式规范

每个 pattern 是一个目录，包含两个 Markdown 文件：

```
data/patterns/<pattern-name>/
├── system.md    — AI 系统提示（角色定义 + 输出格式 + 指令）
└── user.md      — 用户输入模板（可选，空则从 stdin 读取）
```

### system.md 格式 (以 summarize 为例)

```markdown
# IDENTITY and PURPOSE

You are an expert content summarizer...

# OUTPUT SECTIONS

- ONE SENTENCE SUMMARY: ...
- MAIN POINTS: ...
- TAKEAWAYS: ...

# OUTPUT INSTRUCTIONS

- Create the output using the formatting above.
- You only output human readable Markdown.
- Do not output warnings or notes.

# INPUT:
INPUT:
```

**关键规范**：
1. `# IDENTITY and PURPOSE` — 角色设定
2. `# OUTPUT SECTIONS` — 输出结构
3. `# OUTPUT INSTRUCTIONS` — 格式约束
4. `INPUT:` — 用户输入占位符

---

## 三、9 种 Prompt Strategy 详解

| 策略 | 描述 | 复杂度 | 适用场景 |
|------|------|--------|----------|
| **Standard** | 直接回答，无推理 | ★ | 简单事实问答 |
| **CoT (Chain of Thought)** | 逐步推理 | ★★ | 数学/逻辑问题 |
| **CoD (Chain of Draft)** | 每步 ≤5 词的最小推理 | ★ | 需要推理但 token 受限 |
| **AoT (Atom of Thought)** | 拆解为独立原子问题 | ★★★ | 复杂多步骤问题 |
| **ToT (Tree of Thought)** | 多路径并行 + 择优 | ★★★★ | 创意/决策问题 |
| **Reflexion** | 回答→批判→优化 | ★★★ | 代码/写作校对 |
| **Self-Refine** | 初始回答 + 自批评 | ★★ | 质量敏感输出 |
| **Self-Consistent** | 多路径 → 投票选最一致 | ★★★★ | 高可靠性需求 |
| **LTM (Least-to-Most)** | 从简到繁逐步解决 | ★★★ | 复杂分解问题 |

### 策略 JSON 格式

```json
{
    "description": "Chain-of-Thought (CoT) Prompting",
    "prompt": "Think step by step to answer the question. Return the final answer in the required format."
}
```

---

## 四、fabric CLI 架构

```
fabric CLI (Go)
├── --pattern, -p <name>    选择 pattern
├── --strategy <name>       选择策略
├── --stream                流式输出
├── --model <name>          指定模型
├── --setup                 初始化配置
├── --serve                 启动 REST API
└── --dry-run               显示 prompt 但不执行

数据流:
  stdin → [Pattern System Prompt] → LLM API → stdout
```

REST API Server 支持 Ollama 兼容模式，可作为任何 AI 客户端的后端。

---

## 五、三个核心问题回答

### (1) Pattern 格式规范是什么？

```
data/patterns/<name>/
├── system.md  — AI 系统提示
└── user.md    — 用户输入（可选）

system.md 结构:
  # IDENTITY and PURPOSE
  # OUTPUT SECTIONS
  # OUTPUT INSTRUCTIONS
  INPUT:
```

### (2) 哪些 pattern 适合内置到 Agent？

| Pattern | 适合 Hermes Agent | 原因 |
|---------|-------------------|------|
| `summarize` | ★★★★★ | 对话/文档总结，Hermes 原生需要 |
| `analyze_*` 系列 | ★★★★ | 代码分析、日志分析 |
| `create_command` | ★★★★ | 生成 Shell 命令 |
| `review_code` | ★★★★ | 代码审查 |
| `write_essay` | ★★★ | 长文写作 |
| `explain_code` | ★★★ | 代码解释 |
| `raw_query` | ★★ | 裸查询（已有类似功能） |
| `create_mermaid_visualization` | ★★ | 特殊输出格式 |

### (3) Strategy 怎么组合 pattern？

```
fabric --pattern summarize --strategy cot  < input.txt
                              ↑         ↑
                         内容处理     推理策略

组合方式:
1. Pattern 定义 WHAT（做什么）
2. Strategy 定义 HOW（怎么想）
3. fabric CLI 将 strategy prompt 注入到 pattern 之前/之后
```

---

## 六、对 Hermes_Rust_Operit_App 的可复用点

| 复用点 | 优先级 | 说明 |
|--------|--------|------|
| **Pattern 格式借鉴** | ★★★★★ | Hermes Skill 可以改用 fabric 的 system.md 格式（有固定结构、可组合） |
| **Strategy 可组合性** | ★★★★★ | Hermes 的 prompt 策略可以像 fabric 一样分层：Skill(WHAT) + Strategy(HOW) |
| **255 个 pattern 库** | ★★★★ | 可直接移植高价值 pattern（summarize, review_code, analyze_* 等） |
| **REST API 模式** | ★★★ | fabric 的 Ollama 兼容 API 可参考 |
| **众包模式** | ★★★ | 社区贡献 pattern 的模式值得借鉴 |
| **Custom Pattern 机制** | ★★★★ | 用户自定义 pattern 目录的加载逻辑 |

### 具体建议

1. **Hermes Skill 格式升级**：借鉴 fabric 的 `system.md` + `user.md` 双文件格式，让技能更规范
2. **Strategy 层**：在 Hermes 中引入类似 fabric 的 strategy 概念，一个 Skill 可以搭配不同推理策略
3. **高价值 pattern 直接移植**：`summarize`, `review_code`, `analyze_prose_pinker`, `create_conceptmap` 可直接实现为 Hermes Skill
4. **注意差异**：fabric 是"人驱动→AI 执行"模式，Hermes 是"AI 自主决策"模式，pattern 需要从"被动模板"改为"主动技能"
