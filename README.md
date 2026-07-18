# Hermes Rust Operit App

[![Build & Test](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions/workflows/build.yml/badge.svg)](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions/workflows/build.yml)

**纯 Rust 重构的 Operit Android AI 助手应用** — 结合 `hermes-agent-rs` 内核与 Operit 外壳理念，零 JVM 开销，全 Rust 实现。

A pure Rust rewrite of the Operit Android AI assistant — combining the `hermes-agent-rs` kernel with Operit's shell design, zero JVM overhead.

---

## 上游来源 Upstream Sources

| 项目 | 仓库 | 角色 |
|------|------|------|
| **Operit** | [AAswordman/Operit](https://github.com/AAswordman/Operit) | Android AI 助手外壳 — 40+ 工具、MCP/Skill 市场、Ubuntu 环境、本地模型、语音等 |
| **HermesApp** | [SelectXn00b/HermesApp](https://github.com/SelectXn00b/HermesApp) | Hermes 内核 + Operit 壳的 Kotlin 融合版 — 参考架构 |
| **hermes-agent-rs** | [Lumio-Research/hermes-agent-rs](https://github.com/Lumio-Research/hermes-agent-rs) | Rust 版 Hermes 内核 — Agent Loop、10 LLM Provider、30+ Tool、记忆系统 |
| **Hermes Agent** | [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) | 原始 Hermes Agent (Python) — 设计理念 |
| **Operit MCPs** | [jinzechen/Operit_MCPS](https://github.com/jinzechen/Operit_MCPS) | 用户自维护的 MCP 插件集合 (9 个) |

---

## 架构 Architecture

```
┌─────────────────────────────────────────────────────┐
│              Dioxus UI (Android APK)                 │
│  ┌──────┐ ┌──────────┐ ┌──────────┐ ┌───────────┐  │
│  │ Chat │ │  Store   │ │ Settings │ │  Login    │  │
│  │ 聊天  │ │沙盒 Skill│ │ 模型配置  │ │GitHub OAuth│  │
│  │      │ │ MCP 我的 │ │ 角色卡   │ │           │  │
│  └──────┘ └──────────┘ └──────────┘ └───────────┘  │
└────────────────────┬────────────────────────────────┘
                     │ 内部异步通道
┌────────────────────▼────────────────────────────────┐
│           Rust Core (libhermes_operit_core)           │
│                                                      │
│  ┌──────────────────┐  ┌─────────────────────────┐  │
│  │   Agent Loop     │  │     Tool Registry       │  │
│  │ (hermes-agent-rs │  │  内置工具 (40+)          │  │
│  │  集成)           │  │  - 文件系统 (25+ ops)    │  │
│  │                  │  │  - 浏览器 (obscura)      │  │
│  │ 10 LLM Provider  │  │  - 视觉分析 (vision)    │  │
│  │ - OpenAI         │  │  - Markdown (typemill)  │  │
│  │ - Anthropic      │  │  - 系统命令              │  │
│  │ - OpenRouter     │  │  - 网络请求              │  │
│  │ - Qwen/Kimi/...  │  │  - 代码执行 (沙盒)       │  │
│  └──────────────────┘  └─────────────────────────┘  │
│                                                      │
│  ┌──────────────────┐  ┌─────────────────────────┐  │
│  │  Memory System   │  │    Plugin Manager       │  │
│  │  (redb 持久化)    │  │  - Skills 注册表         │  │
│  │  - 会话历史       │  │  - MCP 服务器池          │  │
│  │  - 用户偏好       │  │  - 沙盒启动器            │  │
│  │  - 角色卡         │  │  - 源聚合 (GitHub)      │  │
│  └──────────────────┘  └─────────────────────────┘  │
│                                                      │
│  ┌──────────────────┐  ┌─────────────────────────┐  │
│  │ MCP Client       │  │   Environment           │  │
│  │ stdio JSON-RPC   │  │   Alpine Linux (proot)  │  │
│  │ 外部 MCP 管理:     │  │   apk 包管理器          │  │
│  │ - rust_mcp_server│  │   Node/Python/Rust/Go   │  │
│  │ - rust_docs_mcp  │  │   并行环境配置           │  │
│  │ - hotnews        │  │                         │  │
│  │ - mcp_proxy      │  │   Sandbox               │  │
│  │ - mcp_research.. │  │   (seccomp+landlock)    │  │
│  └──────────────────┘  └─────────────────────────┘  │
│                                                      │
│  ┌──────────────────┐  ┌─────────────────────────┐  │
│  │ GitHub OAuth     │  │   语音 & 本地模型        │  │
│  │ (PKCE 流程)       │  │   - sherpa-rs (TTS/ASR) │  │
│  │ hermesapp://cb    │  │   - llama-cpp-rs        │  │
│  └──────────────────┘  │   - MNN 推理             │  │
│                        └─────────────────────────┘  │
└──────────────────────────────────────────────────────┘
```

---

## 完整功能矩阵 Feature Matrix

### LLM 提供商 (10 个 — 来自 hermes-agent-rs)

| Provider | 状态 |
|----------|------|
| OpenAI (GPT-4o, GPT-4, GPT-3.5) | ✅ |
| Anthropic (Claude 3.5/4) | ✅ |
| OpenRouter (300+ models) | ✅ |
| Generic (OpenAI-compatible) | ✅ |
| Qwen (通义千问) | TODO |
| Kimi (月之暗面) | TODO |
| MiniMax | TODO |
| DeepSeek | TODO |
| Copilot | TODO |
| Codex | TODO |

### 内置工具 (来自 Operit + hermes-agent-rs)

| 类别 | 工具 | 状态 |
|------|------|------|
| **文件系统** | read_file, write_file, list_directory, search_files, get_file_info, create_directory, delete_file, move_file, copy_file, head_file, tail_file, edit_file, find_duplicates, find_empty_dirs, directory_tree, calculate_dir_size, zip/unzip, read_media, read_multiple | ✅ 8 个 / TODO 17 个 |
| **浏览器** | navigate, screenshot, get_html, get_dom, click, type, network_intercept, js_execute | ✅ 骨架 / TODO obscura 集成 |
| **视觉分析** | analyze_image, ocr_text, image_info, describe_scene | ✅ 骨架 / TODO agentic-vision 集成 |
| **Markdown** | render_markdown, extract_code_blocks, strip_formatting, format_markdown, generate_toc, count_words | ✅ 3 个 / TODO 3 个 |
| **系统** | execute_command, get_system_info, manage_processes, network_status | TODO |
| **开发工具** | cargo_build, cargo_check, cargo_test, rustup_toolchain, workspace_introspect | TODO (rust_mcp_server 外部) |
| **文档** | search_crates, get_crate_info, get_rustdoc | ✅ (rust_docs_mcp v2) |
| **记忆** | save_memory, recall_memory, list_memories, delete_memory | ✅ (redb) |

### 外部 MCP 管理 (来自 jinzechen/Operit_MCPS)

| MCP 插件 | 决策 | 理由 |
|----------|------|------|
| **obscura** (无头浏览器) | 内置为库 | 高频浏览器操作，消除进程通信开销 |
| **agentic_vision** (视觉分析) | 内置为库 | 低延迟视觉请求 |
| **rust_mcp_filesystem** (文件系统) | 内置核心逻辑 | 高频文件操作，直接集成 |
| **typemill** (Markdown) | 内置 | 消息渲染必备 |
| **rust_mcp_server** (Rust 开发) | 外部 MCP | 按需启动子进程 |
| **rust_docs_mcp** (Rust 文档) | 外部 MCP | 非核心功能 |
| **hotnews** (新闻聚合) | 外部 MCP | 低频使用 |
| **mcp_proxy** (MCP 代理) | 外部 MCP | 独立部署 |
| **mcp_research_router** (研究路由) | 外部 MCP | 独立部署 |

### 插件商店 (4 个板块)

| 板块 | 功能 | 状态 |
|------|------|------|
| **沙盒** | 管理已安装沙盒环境 (Alpine/Python/Node) | TODO |
| **Skills** | 浏览/安装 Operit .skill 格式技能包 | ✅ PluginStore |
| **MCP** | 浏览/安装 MCP 服务器 (支持 GitHub 源) | ✅ PluginStore |
| **我的** | 查看已安装，一键启用/禁用 | TODO |

### 其他功能

| 功能 | 状态 |
|------|------|
| GitHub OAuth 登录 (PKCE) | ✅ |
| 角色卡 / 人设系统 | TODO |
| 本地模型 (llama.cpp/MNN) | TODO |
| 语音交互 (sherpa-rs TTS/ASR) | TODO |
| Alpine Linux 环境 (proot) | ✅ 骨架 |
| 沙盒 (seccomp + landlock) | TODO |
| 环境配置 (Node/Python/Rust/Go/Java) | TODO |
| 多语言界面 (中/英/日) | TODO |
| 流式响应 | TODO |
| 深度搜索 | TODO |
| 工作流自动化 | TODO |

---

## 源码结构 Source Structure

```
src/
├── lib.rs              ← 模块导出
├── main.rs             ← CLI 入口 (cargo run)
├── core/
│   ├── agent.rs        ← AgentManager + AgentLoop (消息→LLM→工具循环)
│   ├── provider.rs     ← LlmProvider trait + 3 实现 (OpenAI/Anthropic/Generic)
│   ├── tool_registry.rs← ToolHandler trait + HashMap 注册表
│   ├── memory.rs       ← redb 持久化 (会话/偏好/角色卡)
│   └── config.rs       ← AppConfig (模型/温度/token/端点)
├── tools/
│   ├── filesystem.rs   ← FileSystemTool (8 操作, 路径白名单)
│   ├── browser.rs      ← BrowserTool 骨架 (→ obscura 集成)
│   ├── vision.rs       ← VisionTool 骨架 (→ agentic-vision 集成)
│   └── markdown.rs     ← MarkdownTool (渲染/代码提取/格式化)
├── mcp/
│   └── client.rs       ← MCP JSON-RPC 2.0 stdio 客户端 (完整握手)
├── store/
│   └── mod.rs          ← PluginStore (Skill/MCP 安装/卸载/列表)
├── environment/
│   ├── mod.rs          ← Alpine Linux 环境 (proot + apk)
│   └── sandbox.rs      ← 沙盒封装 (seccomp/landlock TODO)
└── ui/
    ├── chat.rs         ← Chat 数据模型 (消息/回复/工具调用)
    ├── store_page.rs   ← 商店浏览器 (GitHub Release API)
    ├── settings.rs     ← 设置管理器 (YAML 持久化)
    └── login.rs        ← GitHub OAuth + PKCE 流程
```

---

## 构建 Build

```bash
# 本地编译 (需要 Rust 工具链)
cargo build --release

# 测试
cargo test

# CLI 模式
cargo run
# > help    查看命令
# > chat    进入对话模式
# > tools   列出注册的工具

# Android APK (待 Dioxus 启用后)
cargo install cargo-apk
cargo apk build --release
```

---

## CI/CD

GitHub Actions: `.github/workflows/build.yml`
- `cargo check` + `cargo test` (每次推送)
- Android APK 构建 (workflow_dispatch)

---

## License

MIT
