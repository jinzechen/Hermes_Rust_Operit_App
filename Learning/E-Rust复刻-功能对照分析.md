# 核心三项目功能对照 & Rust 复刻可行性分析

> **基于执行蓝图**：HermesApp-Rust 复刻与增强执行指南  
> **目标**：分析 Operit + HermesApp 的功能清单，评估 Rust 生态覆盖情况

---

## 一、Operit 完整功能清单（需复刻）

基于 README + HermesApp 参考 + 用户文档：

| 功能大类 | 具体功能 | Rust 生态方案 | 复刻难度 |
|---------|---------|--------------|---------|
| **AI 对话** | 流式聊天、Markdown 渲染、代码高亮 | Dioxus + markdown crate | ★ |
| **工具调用** | 40+ 内置工具 | hermes-tools (69文件) | ★★ |
| **Ubuntu 环境** | 内置 Linux (Termux/proot) | Termux JNI 桥接 | ★★★ |
| **本地模型** | MNN / llama.cpp GGUF | candle / llama-cpp-rs | ★★★ |
| **记忆系统** | 自动分类、时序查询 | hermes-agent-rs memory | ★★ |
| **角色卡** | 性格定制、独立对话 | Dioxus UI + 本地存储 | ★★ |
| **语音交互** | TTS + STT + 语音唤醒 | sherpa-rs / tts-rs | ★★★ |
| **MCP/Skill 市场** | 插件安装/管理 | 自建 PluginManager | ★★★ |
| **无障碍服务** | 屏幕读取、自动点击 | Android JNI + AccessService | ★★★★ |
| **深度搜索** | 网页搜索 + 知识整合 | reqwest + 搜索API | ★★ |
| **工作流** | 自动化流水线 | hermes-agent-rs skill | ★★★ |
| **多语言界面** | i18n 中/英/日/韩 | Dioxus i18n | ★ |

---

## 二、HermesApp 额外功能（已含在 Operit 内的不重复）

| 功能 | 说明 | Rust 方案 |
|------|------|----------|
| **Hermes Agent Loop** | 核心自学习循环 | hermes-agent-rs (24.6K行) |
| **Skills 自创建** | 通过经验创建新技能 | hermes-agent-rs skill_orchestrator |
| **跨会话记忆** | 记住用户偏好 | hermes-agent-rs memory_manager |
| **子 Agent** | 多 Agent 协作 | sub_agent_orchestrator |
| **MCP Server 模式** | 将手机能力暴露给外部 | hermes-mcp |

---

## 三、Operit_MCPS 9 个 MCP 的内置化评估

| MCP | 功能 | 是否可内置 | 理由 |
|-----|------|-----------|------|
| **obscura** | 无头浏览器 | **✅ 内置** | Hermes 已有 browser.rs |
| **agentic_vision** | 视觉分析 | **✅ 内置** | Hermes 已有 vision.rs |
| **rust_mcp_filesystem** | 文件系统 | **✅ 内置** | Hermes 已有 filesystem.rs |
| **typemill** | Markdown | **✅ 内置** | Hermes 已有 markdown.rs |
| **rust_mcp_server** | Rust 工具链 | **⚠️ 保留 MCP** | 太重，适合按需加载 |
| **mcp_proxy** | MCP 代理 | **⚠️ 保留 MCP** | 外部连接用 |
| **sherpa** | 语音 | **✅ 内置** | 文件A已分析 |
| **m3ux** | 音频 | **⚠️ 保留 MCP** | 小众功能 |
| **rust_docs_mcp** | 文档查询 | **⚠️ 保留 MCP** | 按需查询 |

**内置化原则**：高频使用、低延迟要求的功能内置为 ToolHandler（0 开销）；低频、重量级功能保留为 MCP（15ms 开销）。

---

## 四、关键技术 Rust 生态覆盖

| 技术点 | 现有 Rust 方案 | 仓库/ crate | 成熟度 |
|--------|--------------|-------------|--------|
| **UI 框架** | Dioxus ✅ | github.com/DioxusLabs/dioxus | ★★★★★ 36k⭐ |
| **Agent 内核** | hermes-agent-rs ✅ | Lumio-Research/hermes-agent-rs | ★★★★ 72⭐ |
| **MCP 协议** | hermes-mcp ✅ | 同上 crate | ★★★★ |
| **文件系统** | rust-mcp-filesystem ✅ | rust-mcp-stack/rust-mcp-filesystem | ★★★★ |
| **无头浏览器** | obscura ✅ | h4ckf0r0day/obscura | ★★★★★ 19k⭐ |
| **视觉分析** | agentic-vision ✅ | agentralabs/agentic-vision | ★★★ |
| **语音** | sherpa-onnx ✅ | k2-fsa/sherpa-onnx | ★★★★ 13k⭐ |
| **Markdown** | comrak / typemill ✅ | 多个 crate | ★★★★★ |
| **本地模型** | candle ✅ | huggingface/candle | ★★★★★ 20k⭐ |
| **向量搜索** | qdrant-client ✅ | qdrant/qdrant | ★★★★★ 33k⭐ |
| **OAuth** | oauth2 crate ✅ | crates.io | ★★★★★ |
| **沙盒** | hermes code_execution ✅ | hermes-agent-rs 内置 | ★★★ |
