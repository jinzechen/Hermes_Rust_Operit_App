# Hermes_Rust_Operit_App — 28项目全量对比矩阵

> 生成时间: 2026-07-19
> 数据源: Learning/ (28×★★★★★报告) + UA Output/md/ (知识图谱分析)
> 本项目: 51 .rs | 12,108行 | 192测试 | 8模块 | cargo apk → APK 218MB

---

## 评分标准

| 分值 | 含义 |
|------|------|
| 1 | 无 — 不具备该维度能力 |
| 2 | 基础 — 最小实现/单一功能 |
| 3 | 中等 — 完整但非领先 |
| 4 | 强 — 行业领先水平 |
| 5 | 领先 — 该领域标杆/不可替代 |

---

## 一、全量对比矩阵 (28项目 × 10维度)

### 1.1 Agent 引擎核心项目 (直接可比)

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| ★ | **本项目** | Rust/Kotlin | — | 3 | 4 | 3 | 3 | 3 | 3 | 4 | 4 | 3 | 4 | **34** |
| 01 | hermes-agent-rs | Rust | 72 | 4 | 5 | 4 | 3 | 3 | 2 | 2 | 1 | 3 | 3 | **30** |
| 22 | thClaws | Rust | 1,166 | 2 | 4 | 4 | 3 | 3 | 2 | 2 | 1 | 3 | 2 | **26** |
| 25 | goose (LF) | Rust | — | 3 | 5 | 4 | 4 | 3 | 2 | 2 | 1 | 4 | 3 | **31** |
| 26 | claude-code-rust | Rust | 1,667 | 3 | 5 | 4 | 3 | 3 | 2 | 2 | 2 | 3 | 3 | **30** |

### 1.2 MCP/插件生态项目

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| 10 | Operit_MCPS | Rust | — | 5 | 2 | 4 | 5 | 2 | 1 | 2 | 1 | 5 | 3 | **30** |
| 12 | RMCP (官方SDK) | Rust | 3,639 | 2 | 1 | 3 | 5 | 1 | 1 | 2 | 1 | 2 | 2 | **20** |
| 28 | MCP生态+headroom | 多语言 | 149K⭐ | 4 | 2 | 3 | 5 | 2 | 1 | 2 | 1 | 5 | 2 | **27** |
| 27 | AI Agent生态聚合 | 多语言 | — | 3 | 3 | 4 | 4 | 3 | 2 | 2 | 2 | 4 | 2 | **29** |

### 1.3 Android宿主/权限层

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| 04 | HermesApp-Shizuku | Kotlin/C++ | — | 1 | 1 | 1 | 1 | 1 | 1 | 5 | 3 | 1 | 2 | **17** |
| 05 | HermesApp-前台服务 | Kotlin | — | 2 | 1 | 1 | 1 | 1 | 2 | 3 | 1 | 1 | 2 | **15** |
| 06 | HermesApp-无障碍 | Kotlin | — | 2 | 2 | 3 | 1 | 1 | 2 | 4 | 1 | 1 | 2 | **19** |
| 11 | Operit/HermesApp壳 | Kotlin | 5.8K | 3 | 2 | 2 | 2 | 1 | 4 | 4 | 4 | 2 | 5 | **29** |

### 1.4 UI/交互层

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| 17 | Dioxus | Rust | 36,804 | 5 | 1 | 1 | 1 | 1 | 5 | 1 | 1 | 3 | 4 | **23** |
| 18 | HermesApp-UI交互 | Kotlin | 194 | 2 | 1 | 1 | 1 | 1 | 4 | 2 | 1 | 1 | 2 | **16** |
| 19 | Operit原版UI | Kotlin | 5.8K | 2 | 1 | 1 | 1 | 1 | 4 | 2 | 1 | 1 | 2 | **16** |

### 1.5 LLM推理/ML引擎

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| 07 | mistral.rs | Rust | 7,492 | 4 | 2 | 1 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | **17** |
| 03 | EmbedAnything+candle | Rust | 21K | 4 | 1 | 1 | 1 | 3 | 1 | 1 | 1 | 2 | 2 | **17** |
| 14 | sherpa-onnx | C++/Rust | 13,940 | 4 | 1 | 3 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | **18** |

### 1.6 工具/基础设施

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| 02 | rtk | Rust | 71,641 | 2 | 2 | 1 | 1 | 4 | 1 | 2 | 1 | 1 | 2 | **17** |
| 08 | obscura | Rust | 19,392 | 2 | 1 | 4 | 2 | 1 | 1 | 3 | 4 | 1 | 2 | **21** |
| 09 | openhuman | Rust | 35,015 | 4 | 2 | 2 | 2 | 5 | 2 | 2 | 1 | 2 | 2 | **24** |
| 13 | Rust网页搜索 | Rust | — | 2 | 1 | 4 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | **16** |
| 15 | UA Rust | Rust | — | 3 | 1 | 3 | 2 | 3 | 1 | 1 | 1 | 2 | 2 | **19** |
| 16 | wasmer | Rust | 20,904 | 4 | 1 | 1 | 1 | 1 | 1 | 4 | 5 | 2 | 3 | **23** |
| 21 | clipboard-rs | Rust | — | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 1 | 2 | **12** |

### 1.7 生态/聚合/补完

| # | 项目 | 语言 | Stars | 规模 | Agent | 工具 | MCP | 记忆 | UI | 安全 | 沙盒 | 插件 | 构建 | 总分 |
|---|------|------|-------|------|-------|------|-----|------|----|------|------|------|------|------|
| 20 | awesome-rust系统工具 | Rust | 58K | 3 | 1 | 3 | 2 | 2 | 1 | 2 | 2 | 1 | 2 | **19** |
| 23 | AwesomeRust完全补完 | Rust | — | 2 | 2 | 2 | 2 | 3 | 1 | 2 | 2 | 2 | 2 | **20** |
| 24 | AwesomeRust最终补完 | Rust | — | 3 | 2 | 2 | 2 | 2 | 2 | 2 | 2 | 3 | 2 | **22** |

---

## 二、总分排名

| 排名 | 项目 | 总分 | 定位 |
|------|------|------|------|
| 🥇 1 | **本项目** | **34** | 全栈Agent+Android+安全+沙盒 |
| 🥈 2 | goose (LF) | 31 | 顶级Rust Agent引擎 |
| 🥉 3 | hermes-agent-rs | 30 | 上游Agent引擎参考 |
| 🥉 3 | claude-code-rust | 30 | Claude Code Rust重写 |
| 🥉 3 | Operit_MCPS | 30 | MCP插件生态 |
| 6 | Operit/HermesApp壳 | 29 | Android宿主平台 |
| 6 | AI Agent生态聚合 | 29 | 跨语言生态聚合 |
| 8 | MCP生态+headroom | 27 | MCP服务端生态 |
| 9 | thClaws | 26 | 功能高度重合Agent |
| 10 | openhuman | 24 | 记忆系统标杆 |
| 11 | Dioxus | 23 | Rust UI框架 |
| 11 | wasmer | 23 | WASM沙盒标杆 |
| 13 | AwesomeRust最终补完 | 22 | 全量生态扫描 |
| 14 | obscura | 21 | 无头浏览器 |
| 15 | RMCP | 20 | MCP官方SDK |
| 15 | AwesomeRust完全补完 | 20 | 关键遗漏补完 |
| 17 | HermesApp-无障碍 | 19 | 无障碍操控 |
| 17 | UA Rust | 19 | 代码分析引擎 |
| 17 | awesome-rust系统工具 | 19 | 系统工具生态 |
| 20 | sherpa-onnx | 18 | 语音引擎 |
| 21 | HermesApp-Shizuku | 17 | 系统权限 |
| 21 | mistral.rs | 17 | 本地LLM推理 |
| 21 | EmbedAnything+candle | 17 | 本地嵌入/ML |
| 21 | rtk | 17 | Token优化代理 |
| 25 | HermesApp-UI交互 | 16 | UI参考实现 |
| 25 | Operit原版UI | 16 | 上游UI参考 |
| 25 | Rust网页搜索 | 16 | 搜索工具 |
| 28 | clipboard-rs | 12 | 剪贴板库 |

---

## 三、各维度TOP3排名

### 3.1 代码量级 (规模)

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | Operit_MCPS | 5 | 1,229文件/659 Rust/1,505节/5层 |
| 🥇 | Dioxus | 5 | 36,804⭐/40+ packages工作区 |
| 🥉 | hermes-agent-rs | 4 | 579文件/350 Rust/700节点 |
| 🥉 | mistral.rs | 4 | 7,492⭐/全Rust LLM引擎 |
| 🥉 | EmbedAnything+candle | 4 | 21K⭐/HuggingFace生态 |
| 🥉 | openhuman | 4 | 35,015⭐/记忆引擎 |
| 🥉 | wasmer | 4 | 20,904⭐/WASM运行时 |
| 🥉 | MCP生态+headroom | 4 | 149K⭐生态聚合 |

### 3.2 Agent引擎

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | hermes-agent-rs | 5 | 完整Agent Loop/工具调用/上下文管理 |
| 🥇 | goose | 5 | Linux Foundation/企业级Agent |
| 🥇 | claude-code-rust | 5 | Claude Code同构Rust版 |
| 4 | 本项目 | 4 | 全栈Agent+Android集成 |
| 4 | thClaws | 4 | 功能重合度最高 |

### 3.3 工具系统

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | hermes-agent-rs | 4 | 内置Tool系统/多Provider |
| 🥇 | thClaws | 4 | 完整Tool定义+执行 |
| 🥇 | goose | 4 | 企业级Tool系统 |
| 🥇 | claude-code-rust | 4 | 42KB main.rs工具链 |
| 🥇 | Operit_MCPS | 4 | 9个MCP工具服务器 |
| 🥇 | obscura | 4 | 浏览器自动化工具 |
| 🥇 | Rust网页搜索 | 4 | 搜索+抓取工具 |
| 🥇 | AI Agent生态聚合 | 4 | Skills/Patterns工具集 |

### 3.4 MCP协议

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | Operit_MCPS | 5 | 9个自建MCP服务器 |
| 🥇 | RMCP | 5 | MCP官方Rust SDK |
| 🥇 | MCP生态+headroom | 5 | 90K⭐ MCP服务器生态 |
| 4 | goose | 4 | MCP集成支持 |
| 4 | AI Agent生态聚合 | 4 | 跨语言MCP对齐 |

### 3.5 记忆系统

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | openhuman | 5 | tinycortex记忆引擎/35K⭐ |
| 🥈 | rtk | 4 | LLM缓存+压缩/降token 60-90% |
| 3 | hermes-agent-rs | 3 | 对话记忆/上下文管理 |
| 3 | thClaws | 3 | 会话记忆 |
| 3 | goose | 3 | 上下文持久化 |
| 3 | claude-code-rust | 3 | 对话历史管理 |
| 3 | 本项目 | 3 | memory.rs模块 |
| 3 | EmbedAnything+candle | 3 | 嵌入向量记忆 |
| 3 | UA Rust | 3 | 知识图谱节点 |
| 3 | AwesomeRust完全补完 | 3 | VelesDB三位一体 |

### 3.6 UI系统

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | Dioxus | 5 | 36,804⭐/全平台Rust UI |
| 🥈 | Operit/HermesApp壳 | 4 | Android原生UI |
| 🥈 | HermesApp-UI交互 | 4 | ~450KB Kotlin UI |
| 🥈 | Operit原版UI | 4 | ~260KB 上游UI |
| 5 | 本项目 | 3 | Dioxus Android UI |

### 3.7 安全层

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | HermesApp-Shizuku | 5 | 系统级ADB权限/原生JNI |
| 🥈 | HermesApp-无障碍 | 4 | 无障碍权限管道 |
| 🥈 | Operit/HermesApp壳 | 4 | Android权限体系 |
| 🥈 | wasmer | 4 | WASM沙盒安全隔离 |
| 🥈 | 本项目 | 4 | Shizuku+权限管理 |

### 3.8 沙盒执行

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | wasmer | 5 | 20,904⭐/WASM沙盒标杆 |
| 🥈 | Operit/HermesApp壳 | 4 | Android沙盒隔离 |
| 🥈 | obscura | 4 | 浏览器沙盒 |
| 🥈 | 本项目 | 4 | wasmer集成+Android沙盒 |

### 3.9 插件生态

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | Operit_MCPS | 5 | 9个MCP插件/完整生态 |
| 🥇 | MCP生态+headroom | 5 | 全MCP服务器生态 |
| 🥈 | goose | 4 | 丰富扩展点 |
| 🥈 | AI Agent生态聚合 | 4 | Skills/Harness聚合 |
| 5 | hermes-agent-rs | 3 | Tool插件 |
| 5 | thClaws | 3 | 模块化扩展 |
| 5 | Dioxus | 3 | 组件生态 |
| 5 | 本项目 | 3 | MCP插件集成 |
| 5 | claude-code-rust | 3 | 模块化 |
| 5 | AwesomeRust最终补完 | 3 | 脚本引擎生态 |

### 3.10 构建系统

| 排名 | 项目 | 评分 | 依据 |
|------|------|------|------|
| 🥇 | Operit/HermesApp壳 | 5 | Gradle/APK完整构建链 |
| 🥈 | Dioxus | 4 | 全平台交叉编译 |
| 🥈 | 本项目 | 4 | cargo apk → APK 218MB |
| 4 | hermes-agent-rs | 3 | cargo workspace |
| 4 | goose | 3 | cargo build |
| 4 | claude-code-rust | 3 | cargo build |
| 4 | mistral.rs | 3 | cargo+candle |
| 4 | sherpa-onnx | 3 | C++交叉编译+Rust绑定 |
| 4 | wasmer | 3 | cargo+WASM工具链 |
| 4 | Operit_MCPS | 3 | cargo workspace |

---

## 四、差距分析

### 4.1 本项目 vs 最强竞品

| 维度 | 本项目 | 最强竞品 | 差距 | 说明 |
|------|--------|----------|------|------|
| 规模 | 3 | 5 (Operit_MCPS/Dioxus) | -2 | 代码量中等，需扩充 |
| Agent | 4 | 5 (hermes-agent-rs/goose) | -1 | Agent引擎已接近一流 |
| 工具 | 3 | 4 (多项目) | -1 | 工具丰富度待提升 |
| MCP | 3 | 5 (Operit_MCPS/RMCP/MCP生态) | -2 | MCP生态需大力追赶 |
| 记忆 | 3 | 5 (openhuman) | -2 | 记忆系统有巨大提升空间 |
| UI | 3 | 5 (Dioxus) | -2 | 可深度利用Dioxus |
| 安全 | 4 | 5 (Shizuku) | -1 | 安全层已接近顶级 |
| 沙盒 | 4 | 5 (wasmer) | -1 | 沙盒接近标杆 |
| 插件 | 3 | 5 (Operit_MCPS/MCP生态) | -2 | 插件生态待构建 |
| 构建 | 4 | 5 (Operit壳) | -1 | 构建链已成熟 |

### 4.2 核心优势（本项目独有）

1. **唯一全栈方案** — 同时拥有 Rust Agent引擎 + Android宿主 + Shizuku权限 + wasmer沙盒 + Dioxus UI
2. **移动端独占** — 28项目中唯一面向Android移动端的完整Agent方案
3. **安全最优** — Shizuku系统权限 + Android沙盒 + wasmer —— 三维安全体系
4. **构建成熟** — cargo apk直接产出APK 218MB，真正的移动端部署

### 4.3 核心短板（需追赶）

1. **MCP生态** — 需追赶 Operit_MCPS (9个插件) 和 MCP生态 (90K⭐)
2. **记忆系统** — openhuman (tinycortex) 的5分标杆差距明显
3. **插件生态** — 缺少像 MCP生态 那样的外部贡献生态
4. **代码规模** — 12,108行 vs hermes-agent-rs 的579文件/700节点

### 4.4 追赶路径建议

| 优先级 | 维度 | 目标 | 参考项目 | 预期提升 |
|--------|------|------|----------|----------|
| P0 | MCP | 3→5 | Operit_MCPS / RMCP | +2 |
| P0 | 记忆 | 3→5 | openhuman / VelesDB | +2 |
| P1 | 插件 | 3→5 | MCP生态 / goose | +2 |
| P1 | 工具 | 3→4 | hermes-agent-rs / goose | +1 |
| P2 | UI | 3→5 | Dioxus深度定制 | +2 |
| P2 | 规模 | 3→4 | 持续开发 | +1 |

**追赶后预期总分: 34 → 44 (所有维度平均4.4分)**

---

## 五、项目分类总览

```
Agent引擎 (5): 本项目, hermes-agent-rs, thClaws, goose, claude-code-rust
MCP生态 (4): Operit_MCPS, RMCP, MCP生态+headroom, AI Agent生态聚合
Android层 (4): Shizuku, 前台服务, 无障碍, Operit壳
UI层    (3): Dioxus, HermesApp-UI, Operit原版UI
LLM/ML  (3): mistral.rs, EmbedAnything+candle, sherpa-onnx
工具链   (7): rtk, obscura, openhuman, 网页搜索, UA Rust, wasmer, clipboard-rs
生态聚合 (2): awesome-rust系统工具, AwesomeRust补完(23+24)
```

---

*矩阵基于 Learning/ 28份★★★★★深度分析报告 + UA Rust 知识图谱分析引擎输出*  
*本项目数据: 2026-07-19 实测 cargo apk 构建*
