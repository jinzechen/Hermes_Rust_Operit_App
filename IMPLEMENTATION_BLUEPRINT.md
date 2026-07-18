# Hermes_Rust_Operit_App — 实施蓝图

> **状态**: 设计阶段 → 即将进入实现  
> **最后更新**: 2026-07-19  
> **仓库**: [jinzechen/Hermes_Rust_Operit_App](https://github.com/jinzechen/Hermes_Rust_Operit_App)

---

## 零、资料索引（63份，按星级排序）

### ★★★★★ (28份) — 直接复刻参考，融入核心架构

| # | 报告文件 | 对应仓库 | Stars | 实施角色 |
|---|---------|---------|-------|---------|
| 01 | `01-★★★★★-hermes-agent-rs-Rust Agent 引擎.md` | Lumio-Research/hermes-agent-rs | 72 | **Agent 引擎内核** — cargo add 直接依赖 |
| 03 | `03-★★★★★-rtk-LLM Token优化代理.md` | rtk-ai/rtk | 71,641 | **API 成本优化** — HTTP 代理层，节省60-90% token |
| 04 | `04-★★★★★-EmbedAnything-candle-本地ML.md` | huggingface/candle | 20,686 | **本地向量嵌入** — Model2Vec 8M参数, Android可用 |
| 07 | `07-★★★★★-HermesApp-Shizuku系统权限.md` | SelectXn00b/HermesApp | 194 | **Android 系统权**限 — ADB级操作桥接 |
| 08 | `08-★★★★★-HermesApp-前台服务.md` | SelectXn00b/HermesApp | 194 | **后台保活** — ForegroundService JNI桥接 |
| 09 | `09-★★★★★-HermesApp-无障碍服务.md` | SelectXn00b/HermesApp | 194 | **屏幕交互** — AccessibilityService JNI桥接 |
| 10 | `10-★★★★★-mistral.rs-纯Rust本地LLM推理.md` | EricLBuehler/mistral.rs | 7,492 | **离线推理** — feat gate, 纯Rust无C++依赖 |
| 11 | `11-★★★★★-obscura-无头浏览器.md` | h4ckf0r0day/obscura | 19,392 | **浏览器引擎** — CDP Protocol, 内置为ToolHandler |
| 12 | `12-★★★★★-openhuman-记忆系统.md` | tinyhumansai/openhuman | 35,015 | **记忆引擎** — tinycortex L0→L1→L2 摘要树 |
| 13 | `13-★★★★★-Operit_MCPS-MCP插件集.md` | jinzechen/Operit_MCPS | — | **插件目录** — 5内置化 + 4保留MCP |
| 14 | `14-★★★★★-Operit-Android宿主平台.md` | AAswordman/Operit | 5,800 | **功能清单** — 40+工具, Skills/MCP商店, 角色卡 |
| 15 | `15-★★★★★-RMCP-官方MCP-Rust-SDK.md` | modelcontextprotocol/rust-sdk | 3,639 | **MCP 协议** — 替换自建 mcp/client.rs |
| 16 | `16-★★★★★-Rust-网页搜索引擎.md` | — | — | **联网搜索** — reqwest+scraper, DuckDuckGo API |
| 17 | `17-★★★★★-sherpa-onnx-语音引擎.md` | k2-fsa/sherpa-onnx | 13,630 | **语音引擎** — ASR/TTS/VAD, Android预编译.so |
| 18 | `18-★★★★★-Understand_Anything_Rust-代码分析引擎.md` | jinzechen/Understand_Anything_Rust | — | **代码分析** — path dep已集成, 无需额外工作 |
| 19 | `19-★★★★★-wasmer-沙盒执行.md` | wasmerio/wasmer | 20,904 | **代码沙盒** — Wasm运行时, 毫秒启动, Android支持 |
| 20 | `20-★★★★★-Dioxus-RustUI框架.md` | DioxusLabs/dioxus | 36,804 | **UI框架** — 唯支一持Android的Rust UI |
| 44 | `44-★★★★★-HermesApp-UI交互源码分析.md` | SelectXn00b/HermesApp | 194 | **UI布局参考** — 50+页面, Compose→Dioxus映射 |
| 45 | `45-★★★★★-Operit原版UI交互源码分析.md` | AAswordman/Operit | 5,800 | **UI布局参考** — 原版三Tab商店+工具箱 |
| 48 | `48-★★★★★-awesome-rust系统工具.md` | rust-unofficial/awesome-rust | 58,000 | **Rust生态** — 系统工具, 文件管理, 终端模拟 |
| 50 | `50-★★★★★-clipboard-rs-剪贴板库.md` | — | — | **剪贴板** — Android剪贴板JNI桥接 |
| 52 | `52-★★★★★-thClaws-功能重合RustAgent.md` | thClaws/thClaws | 1,166 | **竞品参考** — 三TabUI, MCP, Skills, Agent Teams |
| 54 | `54-★★★★★-AwesomeRust完全补完.md` | — | — | **生态全景** — 1060项目扫描结果 |
| 55 | `55-★★★★★-AwesomeRust最终补完.md` | — | — | **生态全景** — 补充遗漏项目 |
| 59 | `59-★★★★★-goose-RustAI-Agent重合分析.md` | aaif-goose/goose | — | **竞品参考** — 15+Provider, Linux Foundation |
| 60 | `60-★★★★★-claude-code-rust源码分析.md` | lorryjovens-hub/claude-code-rust | 1,667 | **架构参考** — 20模块划分, Tauri+TypeScript→Dioxus |
| 65 | `65-★★★★★-AIAgent全生态聚合仓库.md` | — | — | **设计模式** — ReAct/Plan-Solve/Multi-Agent/RAG |
| 66 | `66-★★★★★-MCP生态-headroom源码分析.md` | — | 59,000 | **Token压缩** — JSON压缩60-95%, 与rtk互补 |

### ★★★★ (22份) — 重要参考，按需集成

| # | 报告 | 实施角色 |
|---|------|---------|
| 21 | `21-★★★★-AndroidRust生态-关键词驱动.md` | Android Rust生态全景 → **选型验证** |
| 22 | `22-★★★★-boa-嵌入式JS引擎.md` | JS引擎 → **可选**（Skills脚本执行） |
| 23 | `23-★★★★-cc-switch-AIAgent桌面中心.md` | 桌面Agent参考 → **架构思路** |
| 24 | `24-★★★★-cron-定时任务生态.md` | Rust定时任务 → **cronjob工具实现** |
| 25 | `25-★★★★-hermes-agent-ultra-安全层.md` | 安全层增强 → **guard.rs + redact.rs 集成** |
| 26 | `26-★★★★-HermesApp-Kotlin参考实现.md` | Kotlin实现对照 → **功能清单验证** |
| 27 | `27-★★★★-HermesApp-通知系统.md` | Android通知 → **Notification JNI桥接** |
| 28 | `28-★★★★-openinterpreter-编码代理.md` | 代码解释器 → **沙盒场景参考** |
| 29 | `29-★★★★-qdrant-向量数据库.md` | 向量数据库 → **备选**（tinycortex已含向量） |
| 30 | `30-★★★★-rustdesk-Android编译参考.md` | Android构建参考 → **构建配置** |
| 31 | `31-★★★★-Rust-密钥管理.md` | 密钥安全 → **credential_guard集成** |
| 32 | `32-★★★★-Rust-事件总线生态.md` | 事件总线 → **内部通信（可选）** |
| 33 | `33-★★★★-Rust-终端执行.md` | 终端模拟 → **terminal工具实现** |
| 34 | `34-★★★★-Tauri-Rust桌面框架.md` | 桌面方案 → **备选**（Dioxus已选） |
| 51 | `51-★★★★-moka-高性能缓存.md` | 高性能缓存 → **moka = "0.12" 已规划** |
| 56 | `56-★★★★-piccolo-LuaVM源码分析.md` | Lua运行时 → **可选**（Skills脚本） |
| 57 | `57-★★★★-tantivy-全文搜索引擎.md` | 全文搜索 → **本地FTS索引** |
| 58 | `58-★★★★-rhai-嵌入式脚本引擎.md` | 嵌入式脚本 → **可选**（Rhai比Lua更适合Rust） |
| 61 | `61-★★★★-MiMo-Code-当前Provider分析.md` | Provider分析 → **已有GenericProvider** |
| 62 | `62-★★★★-openclaw-小龙虾架构分析.md` | Agent架构参考 → **模块设计验证** |
| 63 | `63-★★★★-ZeptoClaw-轻量RustAgent源码分析.md` | 轻量Agent → **25模块对照, 4MB二进制** |
| 64 | `64-★★★★-awesome-claws-RustAgent生态全发现.md` | Agent生态全景 → **功能对标** |

### ★★★ (9份) — 参考了解

| # | 报告 | 说明 |
|---|------|------|
| 35 | `35-★★★-Fabric-AI模式库.md` | AI Patterns → 参考 |
| 36 | `36-★★★-nushell-结构化管道.md` | Shell → terminal工具灵感 |
| 37 | `37-★★★-shadowsocks-网络模式.md` | 网络 → 代理配置参考 |
| 38 | `38-★★★-TabbyML-代码助手.md` | 代码补全 → 可选功能 |
| 39 | `39-★★★-yazi-文件管理器.md` | 文件管理UI → 参考 |
| 46 | `46-★★★-awesome-rust精选新发现.md` | Rust生态新发现 |
| 47 | `47-★★★-awesome-rust终端和GUI方案.md` | UI方案对比 |
| 49 | `49-★★★-awesome-rust完全分析.md` | Rust生态全景 |
| 53 | `53-★★★-Bamboo-octomind功能重合.md` | 功能重合分析 |

### ★★ (4份) — 了解

| # | 报告 | 说明 |
|---|------|------|
| 40 | `40-★★-ds-free-api-DeepSeek代理.md` | DeepSeek代理 |
| 41 | `41-★★-gluesql-嵌入式SQL.md` | 嵌入式SQL（已有redb+SQLite） |
| 42 | `42-★★-oxideterm-AI终端.md` | AI终端 |
| 43 | `43-★★-servo-ds4-AgentS-openclaw.md` | 多项聚合 |

---

## 一、项目定位与核心原则

### 1.1 我们到底在做什么

**不是**从零造轮子。是**组合已有 Rust 生态项目**，复刻 Operit + HermesApp 的全部功能，并增强。

```
输入（已有）:
  hermes-agent-rs (Agent内核)   → Lumio-Research/hermes-agent-rs
  Operit (功能清单+UI设计)       → AAswordman/Operit (5.8K⭐)
  HermesApp (融合案例)           → SelectXn00b/HermesApp
  Operit_MCPS (9个MCP插件)       → jinzechen/Operit_MCPS
  63份学习报告 + 74份UA分析      → Learning/ + UA Output/

输出（目标）:
  Hermes_Rust_Operit_App
  = Dioxus UI + hermes-agent-rs + Operit功能 + Operit_MCPS内置化
  → 纯Rust, 单二进制, Android原生, 全功能AI助手
```

### 1.2 核心原则（铁则）

1. **不造轮子**: `cargo add` 优先于自己写。hermes-agent-rs 已有18个crate，直接用。
2. **性能为王**: ToolHandler(0ms) > MCP(15ms) > Skills(50ms)。高频工具必须内置。
3. **选贤举能**: 功能重合时，谁性能更强就取代谁。rmcp替换自建mcp/client.rs就是例子。
4. **功能不丢**: Operit的全部功能必须复刻到位。40+工具、Skills/MCP商店、角色卡、语音、Ubuntu环境——逐个对照。
5. **纯Rust**: UI用Dioxus，不用Kotlin/Compose写任何一行UI。

### 1.3 与原版功能的精确对照

以下是 Operit (AAswordman/Operit) 的核心功能清单，以及 Rust 复刻方案：

| Operit 功能 | Kotlin 实现 | Rust 复刻方案 | 状态 |
|------------|------------|-------------|------|
| **AI 对话** | Jetpack Compose ChatView | Dioxus ChatView (ui/chat.rs) | 🔧 已有框架 |
| **多Provider** | 13个LLM Provider | hermes-agent-rs 10+Provider + 自建GenericProvider | ✅ 60% |
| **Shizuku 权限** | ShizukuInstaller + ShizukuAuthorizer | android/shizuku.rs (JNI) | 🔜 Phase 2 |
| **无障碍服务** | AccessibilityUITools (33KB) | android/accessibility.rs (JNI) | 🔜 Phase 2 |
| **前台服务** | AIForegroundService (80KB) | android/foreground.rs (JNI) | 🔜 Phase 2 |
| **通知系统** | NotificationListener | android/notification.rs (JNI) | 🔜 Phase 2 |
| **40+ 内置工具** | Kotlin函数调用 | hermes-tools 35个工具 + 5个内置化 | 🔧 5/35 |
| **MCP/Skill 市场** | 三Tab商店 (Kotlin) | Dioxus MarketView + rmcp client | 🔜 Phase 2 |
| **Ubuntu 24 环境** | proot + rootfs | std::process调用proot | 🔜 Phase 2 |
| **本地模型 (MNN/llama.cpp)** | C++ JNI | mistral.rs (纯Rust) | 🔜 Phase 3 |
| **语音对话** | Android Speech API | sherpa-onnx (纯本地) | 🔜 Phase 3 |
| **角色卡** | ObjectBox存储 | redb + serde | 🔧 已有redb |
| **GitHub OAuth** | Android WebView | oauth2 crate + dioxus | 🔧 已有login.rs |
| **文件管理** | Android SAF | tools/filesystem.rs (367L) | ✅ 已实现 |
| **Markdown渲染** | 自建渲染器 | tools/markdown.rs (321L) | ✅ 已实现 |
| **剪贴板** | Android ClipboardManager | clipboard-rs + JNI | 🔜 Phase 2 |
| **深度搜索** | WebView搜索 | tools/web.rs (reqwest+scraper) | 🔜 Phase 1 |

---

## 二、最终技术栈（一行不差）

```toml
[package]
name = "hermes_rust_operit_app"
version = "0.1.0"
edition = "2021"

[dependencies]
# ═══ Agent 内核 ═══
hermes-agent-rs = { git = "https://github.com/Lumio-Research/hermes-agent-rs" }
# 替代自建 agent.rs (238行) — 获得7,001行AgentLoop + 35工具 + 8记忆插件

# ═══ UI 框架 ═══
dioxus = { version = "0.5", features = ["mobile"] }
dioxus-mobile = "0.5"
# 唯支一持Android的Rust UI框架 (36,804⭐)

# ═══ 异步运行时 ═══
tokio = { version = "1", features = ["full"] }

# ═══ 序列化 ═══
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"

# ═══ 代码分析 ═══
ua-core = { path = "../Understand_Anything_Rust/crates/ua-core" }  # 已有

# ═══ MCP 协议 ═══
rmcp = "0.8"                      # 替换自建 mcp/client.rs (14.4KB)
# 官方MCP Rust SDK (3,639⭐) — stdio + SSE双传输

# ═══ 记忆引擎 ═══
tinycortex = "0.1"                # 替换 HashMap-based memory.rs
# openhuman的记忆引擎 (35,015⭐) — L0→L1→L2 + 混合检索

# ═══ HTTP ═══
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }

# ═══ 数据库 ═══
redb = "1"                        # KV存储 (已有)

# ═══ 全文搜索 ═══
tantivy = "0.22"                  # 本地FTS索引

# ═══ 缓存 ═══
moka = { version = "0.12", features = ["future"] }

# ═══ 网页解析 ═══
scraper = "0.19"                  # HTML解析 (tools/web.rs)

# ═══ 认证 ═══
oauth2 = { version = "4.4", features = ["reqwest"] }

# ═══ Android 桥接 ═══
jni = "0.21"                      # JNI (android/*.rs)

# ═══ 工具库 ═══
parking_lot = "0.12"
once_cell = "1"
thiserror = "1"
anyhow = "1"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
walkdir = "2"
flate2 = "1"
regex = "1"
directories = "5"
log = "0.4"
env_logger = "0.11"
tracing = "0.1"
tracing-subscriber = "0.3"

# ═══ Phase 3 按需启用 (feature gates) ═══
wasmer = { version = "4", optional = true }              # 沙盒
mistralrs = { version = "0.3", optional = true }         # 本地推理
# sherpa-rs 通过预编译 .so 加载 (Android), 不需要 cargo dep

[features]
default = []
sandbox = ["wasmer"]
local-inference = ["mistralrs"]
full = ["sandbox", "local-inference"]

[profile.release]
lto = true
opt-level = "s"       # 体积优化 (Android APK)
strip = true
codegen-units = 1
```

---

## 三、模块结构（最终形态）

```
hermes_rust_operit_app/
├── Cargo.toml
├── ARCHITECTURE.md                    # 专业架构文档
├── IMPLEMENTATION_BLUEPRINT.md        # 本文档
├── SESSION_HANDOFF.md                 # flash→pro交接
├── HermesApp-Rust 重构思路参考.md      # 原版思路记录
│
├── Learning/                          # 63份学习报告 (只读参考)
│   ├── ★★★★★/   (28份)
│   ├── ★★★★/    (22份)
│   ├── ★★★/     (9份)
│   └── ★★/      (4份)
│
├── src/
│   ├── main.rs                        # Android入口 (Dioxus launch)
│   ├── lib.rs                         # 核心库入口
│   │
│   ├── core/                          # ───── 核心引擎 ─────
│   │   ├── mod.rs
│   │   ├── agent.rs                   # AgentManager → 替换为 hermes-agent-rs
│   │   ├── provider.rs                # GenericProvider + SmartRouter
│   │   ├── tool_registry.rs           # ToolHandler trait + ToolRegistry
│   │   ├── config.rs                  # AppConfig (YAML/JSON)
│   │   └── memory.rs                  # → 替换为 tinycortex
│   │
│   ├── tools/                         # ───── 工具系统 ─────
│   │   ├── mod.rs
│   │   ├── filesystem.rs   [367L]     # ✅ 8个文件操作
│   │   ├── markdown.rs     [321L]     # ✅ Markdown渲染+代码提取
│   │   ├── browser.rs      [109L]     # 🔜 替换为obscura CDP
│   │   ├── vision.rs                  # 🔜 替换为agentic_vision
│   │   ├── codebase_analyzer.rs       # ✅ UA Rust集成
│   │   ├── web.rs                     # 🔜 Phase 1 (reqwest+scraper)
│   │   ├── terminal.rs                # 🔜 Phase 1
│   │   ├── process.rs                 # 🔜 Phase 1
│   │   ├── tts.rs                     # 🔜 Phase 3 (sherpa-onnx)
│   │   ├── transcription.rs           # 🔜 Phase 3 (sherpa-onnx)
│   │   ├── session_search.rs          # 🔜 Phase 1 (tantivy)
│   │   └── cronjob.rs                 # 🔜 Phase 1
│   │
│   ├── mcp/                           # ───── MCP层 ─────
│   │   ├── mod.rs
│   │   └── client.rs       [14.4KB]   # 🔜 替换为 rmcp crate
│   │
│   ├── android/                       # ───── Android桥接 (Phase 2) ─────
│   │   ├── mod.rs
│   │   ├── jni.rs                     # JNI初始化
│   │   ├── shizuku.rs                 # Shizuku系统权限
│   │   ├── accessibility.rs           # 无障碍服务
│   │   ├── foreground.rs              # 前台服务保活
│   │   ├── notification.rs            # 通知管理
│   │   └── clipboard.rs               # 剪贴板
│   │
│   ├── ui/                            # ───── Dioxus UI ─────
│   │   ├── mod.rs
│   │   ├── app.rs                     # 主入口, Tab导航
│   │   ├── chat.rs                    # 对话界面 (流式渲染)
│   │   ├── market.rs                  # 三Tab市场 (Artifacts/Skills/MCP)
│   │   ├── toolbox.rs                 # 工具箱 (9工具)
│   │   ├── settings.rs                # 设置 (25+页面)
│   │   ├── memory_view.rs             # 记忆库
│   │   ├── login.rs                   # GitHub OAuth
│   │   ├── store_page.rs              # 商店页面
│   │   └── components/                # 可复用组件
│   │       ├── message_bubble.rs
│   │       ├── code_block.rs
│   │       ├── voice_input.rs
│   │       └── plugin_card.rs
│   │
│   ├── store/                         # ───── 插件商店 ─────
│   │   ├── mod.rs
│   │   ├── index.rs                   # 商店索引 (从GitHub Releases拉取)
│   │   ├── installer.rs               # 下载+验证+安装
│   │   └── manager.rs                 # 启用/禁用/卸载
│   │
│   └── environment/                   # ───── 沙盒 ─────
│       ├── mod.rs
│       └── sandbox.rs      [71L]      # 🔜 替换为 wasmer
│
└── tests/
    ├── agent_integration.rs
    ├── tool_tests.rs
    └── mcp_integration.rs
```

---

## 四、关键决策：什么内置、什么保留MCP、什么用cargo

### 4.1 Operit_MCPS 9个插件的处置（性能分析）

| 插件 | 调用频率 | 当前方案 | 推荐方案 | 理由 |
|------|---------|---------|---------|------|
| **obscura** | 极高 (每轮对话) | MCP子进程 (15ms) | **✅ 内置 ToolHandler (0ms)** | 浏览器是Agent核心能力, MCP开销不可接受 |
| **agentic_vision** | 高 (截图/OCR) | MCP子进程 (15ms) | **✅ 内置 ToolHandler (0ms)** | 视觉分析频繁调用, 内置化节省延迟 |
| **rust_mcp_filesystem** | 极高 (每操作) | MCP子进程 (15ms) | **✅ 内置 ToolHandler (0ms)** | 文件操作是基础能力, 已有filesystem.rs可直接扩展 |
| **typemill** | 高 (文档渲染) | MCP子进程 (15ms) | **✅ 内置 ToolHandler (0ms)** | Markdown渲染已有markdown.rs, 合并即可 |
| **sherpa** | 高 (语音) | MCP子进程 | **✅ 内置 → sherpa-onnx** | 语音是核心差异化, 用sherpa-onnx直接替代 |
| **rust_mcp_server** | 低 (按需) | MCP子进程 | **⚠️ 保留 MCP** | Rust工具链太重(多GB), 不适合内置 |
| **mcp_proxy** | 低 (按需) | MCP子进程 | **⚠️ 保留 MCP** | 本身就是代理, 必须外部进程 |
| **m3ux** | 极低 | MCP子进程 | **⚠️ 保留 MCP** | 低频音频工具 |
| **rust_docs_mcp** | 低 (按需) | MCP子进程 | **⚠️ 保留 MCP** | 文档按需查询 |

**结论**: 5个内置化 (0ms), 4个保留MCP。

### 4.2 自建 vs cargo add 决策表

| 自建组件 | 行数 | 上游替代 | 决策 | 理由 |
|---------|------|---------|------|------|
| `core/agent.rs` | 238L | `hermes-agent-rs` (7,001L) | **cargo add** | 18个crate, 35工具, 8记忆插件 — 不可能自建 |
| `mcp/client.rs` | ~400L | `rmcp` (3,639⭐) | **cargo add** | 官方SDK, stdio+SSE双传输, 比自己写可靠 |
| `core/memory.rs` | ~50L | `tinycortex` (35K⭐) | **cargo add** | L0→L1→L2摘要树, 混合检索, SQLite持久化 |
| `tools/browser.rs` | 109L | obscura CDP | **内置化** | CDP协议直接调用, 不绕MCP |
| `tools/vision.rs` | ~80L | agentic_vision | **内置化** | OCR/图片分析直接调用 |
| `tools/filesystem.rs` | 367L | 保留+扩展 | **保留扩展** | 已有8个操作, 补充rust_mcp_filesystem的25+操作 |
| `tools/markdown.rs` | 321L | 保留+合并 | **保留合并** | 已有渲染/提取, 合并typemill功能 |
| `environment/sandbox.rs` | 71L | `wasmer` (20,904⭐) | **cargo add** | Wasm沙盒, Android支持, 毫秒启动 |
| `core/provider.rs` | 386L | 保留扩展 | **保留扩展** | 已有GenericProvider+AnthropicProvider, 加入SmartRouter |

---

## 五、分阶段实施计划

### Phase 1: 基础融合（Week 1-2）— 让Agent跑起来

**目标**: 一个能对话、有工具、有记忆的CLI Agent。

```
优先级 P0 — 必须完成:
□ 1.1 cargo add hermes-agent-rs → 替换 core/agent.rs
      删除自建 agent.rs
      用 hermes-agent-rs 的 AgentLoop 替换

□ 1.2 cargo add rmcp → 替换 mcp/client.rs
      删除自建 client.rs
      用 rmcp::Service::call_tool() 替换

□ 1.3 cargo add tinycortex → 替换 core/memory.rs
      删除 HashMap-based 存储
      用 tinycortex::MemoryEngine 替换

□ 1.4 内置化 5 个 Operit_MCPS 插件:
      - tools/browser.rs → obscura CDP协议直接调用
      - tools/vision.rs → agentic_vision OCR/分析直接调用
      - tools/filesystem.rs → 扩展至25+操作
      - tools/markdown.rs → 合并 typemill
      - tools/speech.rs → 新建 (Phase 3启用sherpa-onnx)

□ 1.5 新增 4 个工具:
      - tools/web.rs → reqwest + scraper (DuckDuckGo搜索)
      - tools/terminal.rs → shell命令执行
      - tools/process.rs → 后台进程管理
      - tools/cronjob.rs → 定时任务

□ 1.6 cargo add moka + tantivy
      - moka缓存热门搜索结果
      - tantivy索引会话历史

优先级 P1 — 应该完成:
□ 1.7 Provider 增强:
      - SmartModelRouter (编译→pro, 文档→flash)
      - rtk HTTP代理集成 (节省60-90% token)
      - headroom JSON压缩 (节省60-95%)

□ 1.8 安全层 (ref: hermes-agent-ultra):
      - tools/guard.rs (19种危险模式 + 9种prompt injection检测)
      - core/redact.rs (30+密钥前缀 + PII脱敏)
```

### Phase 2: Android融合（Week 3-4）— 让它在手机上活

**目标**: 能在Android上运行, 有UI, 有系统权限。

```
□ 2.1 android/ 模块:
      - jni.rs → JNI初始化
      - shizuku.rs → Shizuku权限桥接 (input tap/swipe, screencap, pm)
      - accessibility.rs → 无障碍服务桥接
      - foreground.rs → 前台服务保活
      - notification.rs → 通知管理
      - clipboard.rs → 剪贴板

□ 2.2 Dioxus UI 启用:
      - Cargo.toml: 取消 dioxus + dioxus-mobile 注释
      - ui/app.rs → 5 Tab导航 (Chat/Market/Toolbox/Memory/Settings)
      - ui/chat.rs → 流式对话, Markdown渲染, 文本可选复制
      - ui/market.rs → 三Tab市场 (Artifacts/Skills/MCP)
      - ui/toolbox.rs → 9工具箱
      - ui/settings.rs → 设置 (Provider/Model/Voice/Account)

□ 2.3 GitHub OAuth:
      - oauth2 crate + hermesapp://callback URL scheme
      - 彻底解决登录404

□ 2.4 插件商店后端:
      - store/index.rs → 从GitHub Releases拉取索引
      - store/installer.rs → 下载+sha256验证+安装
      - store/manager.rs → 启用/禁用/卸载

□ 2.5 APK 构建:
      - cargo ndk -t aarch64-linux-android build --release
      - Gradle 打包
```

### Phase 3: 能力增强（Week 5-6）— 语音+沙盒+本地推理

```
□ 3.1 wasmer 沙盒 (feature gate: sandbox):
      - 替换 environment/sandbox.rs (71L占位符)
      - wasmer::Module + wasmer::Instance
      - 默认无文件系统/网络访问
      - 256MB内存限制, 30s超时

□ 3.2 语音引擎 (sherpa-onnx):
      - tools/tts.rs → sherpa-onnx TTS (VITS模型)
      - tools/transcription.rs → sherpa-onnx ASR (Whisper-Tiny)
      - 加载预编译 aarch64-linux-android .so

□ 3.3 本地推理 (feature gate: local-inference):
      - provider/mistral.rs → MistralRsProvider
      - Qwen2.5-1.5B-Instruct-GGUF (Q4, ~1GB)
      - 离线场景自动切换

□ 3.4 UI 完:
善:
      - 语音输入按钮
      - 角色卡编辑器
      - Ubuntu 24 终端 (proot调用)
      - 自定义音色管理
```

---

## 六、关键技术实现细节

### 6.1 Agent Loop 替换方案

hermes-agent-rs的AgentLoop (7,001行) 直接替换自建 agent.rs (238行):

```rust
// src/core/agent.rs → 删除大部分代码, 保留薄封装

use hermes_agent_rs::{AgentLoop, AgentConfig, AgentBuilder};

pub struct HermesAgent {
    inner: AgentLoop,
}

impl HermesAgent {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let agent_config = AgentConfig {
            model: config.model,
            api_key: config.api_key,
            api_endpoint: config.api_endpoint,
            // ... 映射其他配置
        };

        let inner = AgentBuilder::new(agent_config)
            .with_tools(self::register_all_tools())
            .with_memory(self::create_memory_manager())
            .build()
            .await?;

        Ok(Self { inner })
    }

    pub async fn chat(&self, session_id: &str, message: &str) -> Result<String> {
        self.inner.send_message(session_id, message).await
    }
}
```

### 6.2 ToolHandler 内置化细节

obscura 内置化示例:

```rust
// src/tools/browser.rs — 删除占位符, 实现CDP协议

use obscura_cdp::{Browser, Page, Runtime};  // 假设obscura提供Rust API

impl ToolHandler for BrowserTool {
    fn execute(&self, params: Value) -> Result<String> {
        let action = params["action"].as_str().unwrap();

        match action {
            "navigate" => {
                let url = params["url"].as_str().unwrap();
                let page = self.browser.new_page()?;
                page.navigate(url)?;
                Ok(page.content()?)
            }
            "screenshot" => {
                let data = self.browser.screenshot_full_page()?;
                Ok(format!("data:image/png;base64,{}", base64::encode(&data)))
            }
            // ... CDP操作直接调用, 0ms开销
        }
    }
}
```

### 6.3 三Tab插件商店UI

```rust
// src/ui/market.rs

#[derive(PartialEq, Clone)]
enum MarketTab { Artifacts, Skills, Mcp }

pub fn MarketView(cx: Scope) -> Element {
    let active_tab = use_state(cx, || MarketTab::Skills);
    let plugins = use_future(cx, (), |_| fetch_store_index());

    cx.render(rsx! {
        div { class: "market-container",
            // 三Tab切换
            div { class: "tab-bar",
                button {
                    class: if *active_tab == MarketTab::Artifacts { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(MarketTab::Artifacts),
                    "📦 Artifacts"
                }
                button {
                    class: if *active_tab == MarketTab::Skills { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(MarketTab::Skills),
                    "🧠 Skills"
                }
                button {
                    class: if *active_tab == MarketTab::Mcp { "tab active" } else { "tab" },
                    onclick: move |_| active_tab.set(MarketTab::Mcp),
                    "🔌 MCP"
                }
            }
            // 内容区
            match *active_tab.get() {
                MarketTab::Artifacts => rsx!(ArtifactList { plugins: plugins }),
                MarketTab::Skills => rsx!(SkillList { plugins: plugins }),
                MarketTab::Mcp => rsx!(McpList { plugins: plugins }),
            }
        }
    })
}
```

### 6.4 文本可复制（解决痛点2）

```rust
// ui/components/message_bubble.rs

pub fn MessageBubble(cx: Scope, content: String, role: String) -> Element {
    cx.render(rsx! {
        div {
            class: "message-bubble {role}",
            // Dioxus默认文本可选, 无需特殊处理
            // 关键: 不要设置 user-select: none
            div {
                class: "message-content",
                // Markdown渲染后的内容, 天然可选中
                dangerous_inner_html: render_markdown_to_html(&content)
            }
            // 添加显式复制按钮
            button {
                class: "copy-btn",
                onclick: move |_| {
                    // 调用系统剪贴板
                    clipboard_write(&content);
                },
                "📋"
            }
        }
    })
}
```

### 6.5 wasmer 沙盒（性能对比）

```
┌──────────┬──────────┬───────────┬───────────┐
│          │ wasmer   │ nsjail    │ Docker    │
├──────────┼──────────┼───────────┼───────────┤
│ 启动时间 │ <1ms     │ ~50ms     │ ~2s       │
│ Android  │ ✅       │ ❌        │ ❌        │
│ 默认隔离 │ 无I/O    │ 需配置     │ 全权限    │
│ 二进制   │ ~5MB     │ ~500KB    │ ~50MB+    │
│ 内存限制 │ ✅       │ ✅        │ ✅        │
│ CPU限制  │ ✅(opcode)│ ✅(cgroup)│ ✅(cgroup)│
└──────────┴──────────┴───────────┴───────────┘

结论: wasmer 在 Android 上是唯一可行的沙盒方案
```

---

## 七、构建与CI

### 7.1 本地开发

```bash
# Windows开发 (CLI模式, 无UI/无沙盒/无Android)
cd D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App
cargo build --release
cargo run --release

# Android构建
rustup target add aarch64-linux-android
cargo ndk -t aarch64-linux-android -o ../app/src/main/jniLibs build --release

# MCP插件构建 (aarch64-musl, 全静态)
cross build --target aarch64-unknown-linux-musl --release -p <plugin>
```

### 7.2 CI Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-linux-android
      - run: cargo build --target aarch64-linux-android --release

  build-plugins:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        plugin: [obscura, agentic_vision, filesystem, typemill, sherpa]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cross
      - run: cross build --target aarch64-unknown-linux-musl --release -p ${{ matrix.plugin }}
```

---

## 八、痛点解决方案总结

| 痛点 | 原因 | 解决方案 | 实现位置 |
|------|------|---------|---------|
| **1. GitHub登录404** | OAuth回调配置失效 | oauth2 crate + `hermesapp://callback` URL scheme | `ui/login.rs` |
| **2. 无法复制文字** | UI事件阻止 | Dioxus默认文本可选 + 显式复制按钮 | `ui/components/message_bubble.rs` |
| **3. 商店老旧无源码** | 闭源组件过时 | GitHub Releases开源分发 + 源管理 | `store/` + `ui/market.rs` |

---

## 九、立即开始的3个Action

```
Action 1: cargo add 四大件
  cd D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App
  cargo add rmcp tinycortex moka tantivy scraper
  # 验证: cargo check

Action 2: 替换 Agent 内核
  保留 core/agent.rs 做薄封装
  用 hermes-agent-rs 的 AgentLoop 替换自建循环
  验证: cargo test

Action 3: 内置化 5 个工具
  重写 tools/browser.rs → obscura CDP
  重写 tools/vision.rs → agentic_vision
  扩展 tools/filesystem.rs → 25+操作
  合并 typemill → tools/markdown.rs
  新建 tools/speech.rs (先占位, Phase 3启用sherpa-onnx)
  验证: cargo test --test tool_tests
```

---

> **参考仓库**:
> - hermes-agent-rs: https://github.com/Lumio-Research/hermes-agent-rs
> - Operit: https://github.com/AAswordman/Operit
> - HermesApp: https://github.com/SelectXn00b/HermesApp
> - Operit_MCPS: https://github.com/jinzechen/Operit_MCPS
> - Understand_Anything_Rust: https://github.com/jinzechen/Understand_Anything_Rust
> - 本项目: https://github.com/jinzechen/Hermes_Rust_Operit_App
