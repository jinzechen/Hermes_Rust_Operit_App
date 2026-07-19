# Hermes_Rust_Operit_App — 纯 Rust Android AI 助手

> 基于 20+ Rust 生态项目，100% 纯 Rust 的 Android AI Agent  
> A 100% pure Rust Android AI Agent, assembled from 20+ Rust ecosystem projects

[![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Android](https://img.shields.io/badge/Android-Rust%20Only-brightgreen)]()

**0 个 .kt · 0 个 .xml · 0 个 Gradle — 纯 Rust，从 UI 到推理全链路。**

---

## 上游项目功能对比 / Upstream Feature Comparison

| 上游项目 / Upstream | Stars | 功能 / Feature | 本项目复用 / Reused |
|---|---|---|---|
| [rtk-ai/rtk](https://github.com/rtk-ai/rtk) | 71.6K | LLM Token 优化代理 / Token optimization proxy | Token 路由 + 压缩管线 |
| [headroom](https://github.com/vacavaca/headroom) | 59.0K | 上下文 Token 压缩 / Context compression | 长对话 Token 截断策略 |
| [DioxusLabs/dioxus](https://github.com/DioxusLabs/dioxus) | 36.8K | Rust 跨平台 UI / Cross-platform UI framework | Android 原生 UI 渲染 (Chat/Market/Settings) |
| [tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman) | 35.0K | AI 记忆引擎 / Memory engine (tinycortex) | redb + tantivy 全文记忆索引 |
| [wasmerio/wasmer](https://github.com/wasmerio/wasmer) | 20.9K | WebAssembly 沙盒运行时 / Wasm sandbox | 工具执行沙盒 + 进程隔离 |
| [h4ckf0r0day/obscura](https://github.com/h4ckf0r0day/obscura) | 19.4K | CDP 无头浏览器 / Headless browser | 网页导航 / 截图 / JS 执行 |
| [k2-fsa/sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) | 13.6K | 语音引擎 ASR/TTS/VAD / Voice pipeline | sherpa-rs 绑定 → 语音输入输出 |
| [EricLBuehler/mistral.rs](https://github.com/EricLBuehler/mistral.rs) | 7.5K | 本地 LLM 推理 / Local inference engine | 离线模型推理后端 |
| [AAswordman/Operit](https://github.com/AAswordman/Operit) | 5.8K | Android AI 助手 / 40+ Tools, Skills/MCP Store, 角色卡 | 整体架构 + 功能清单参考 |
| [modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk) | 3.6K | MCP 官方 Rust SDK / Official MCP protocol | rmcp → JSON-RPC 2.0 客户端 |
| [lorryjovens-hub/claude-code-rust](https://github.com/lorryjovens-hub/claude-code-rust) | 1.7K | Claude Code Rust 架构 / 20 模块参考 | Agent Loop + Tool 调度架构 |
| [thClaws/thClaws](https://github.com/thClaws/thClaws) | 1.2K | Rust AI Agent / 3-Tab, MCP, Skills | Agent 交互模式 + MCP 集成 |
| [goose](https://github.com/block/goose) | — | 15+ LLM Provider 适配 / Multi-provider | SmartModelRouter 多模型路由 |
| [qhkm/zeptoclaw](https://github.com/qhkm/zeptoclaw) | 644 | 轻量 Agent / 25 模块, 4MB | 模块化拆分参考 |
| [SelectXn00b/HermesApp](https://github.com/SelectXn00b/HermesApp) | 194 | Hermes + Operit 融合 / Fusion case | Agent 内核 + Android 壳集成方案 |
| [Lumio-Research/hermes-agent-rs](https://github.com/Lumio-Research/hermes-agent-rs) | 72 | Hermes Agent 引擎 / ReAct Loop + 子Agent | Agent 核心调度引擎 |
| [jinzechen/Operit_MCPS](https://github.com/jinzechen/Operit_MCPS) | — | MCP 插件集 / 9 插件, 插件商店 | MCP 工具注册 + GitHub Releases 分发 |
| [jinzechen/Understand_Anything_Rust](https://github.com/jinzechen/Understand_Anything_Rust) | — | Rust 代码分析引擎 / Knowledge graph | scan→parse→graph→report 管线 |

---

## 功能清单 / Features

| 模块 | 功能说明 |
|---|---|
| **Agent 引擎** | ReAct Loop + SmartModelRouter 多模型路由 + 子 Agent 编排 |
| **工具系统** | 11 个内置 ToolHandler (文件/浏览器/网页搜索/终端/进程/Cron/语音/代码分析/Markdown/Vision) |
| **MCP 协议** | JSON-RPC 2.0 over stdio，兼容 MCP 生态全部插件 |
| **插件商店** | GitHub Releases 分发，一键安装/更新 MCP 插件 |
| **语音** | sherpa-onnx 驱动 ASR 语音识别 + TTS 语音合成 + VAD 静音检测 |
| **记忆系统** | redb 持久化 + tantivy 全文检索，支持会话记忆与知识库 |
| **浏览器** | CDP 无头浏览器 (导航/截图/点击/JS 执行/表单填写) |
| **终端** | Shell 执行 + 超时控制 + 后台进程管理 (spawn/kill/wait) |
| **定时任务** | Cron 表达式调度 (如 `30m` / `0 9 * * *`) |
| **安全层** | 19 种危险命令拦截 + 30+ 密钥脱敏 + 路径白名单 |
| **沙盒** | wasmer WebAssembly 隔离执行 + 进程超时 |
| **代码分析** | Rust 项目知识图谱 (scan→parse→graph→report) |
| **本地推理** | mistral.rs 离线 LLM，无需网络 |
| **Token 优化** | rtk 路由 + headroom 上下文压缩，节省 60%+ Token |
| **UI** | Dioxus 纯 Rust 渲染 Chat / Market / Memory / Settings 页面 |

---

## Android 构建 / Build (Pure Rust, No Kotlin/Gradle)

### 前置条件

- Rust 1.97+
- Android NDK r26+
- `aarch64-linux-android` target

### 一键构建

```bash
# 1. 安装 Android target & cargo-ndk
rustup target add aarch64-linux-android
cargo install cargo-ndk

# 2. 克隆仓库
git clone https://github.com/jinzechen/Hermes_Rust_Operit_App.git
cd Hermes_Rust_Operit_App

# 3. 克隆依赖
git clone https://github.com/jinzechen/Understand_Anything_Rust.git ../Understand_Anything_Rust

# 4. 构建 .so
cargo ndk -t aarch64-linux-android -p 26 build --release
# → target/aarch64-linux-android/release/libhermes_operit_core.so

# 5. 加载到 Android 项目 (JNI)
# libhermes_operit_core.so → app/src/main/jniLibs/arm64-v8a/
```

**不需要 Android Studio、Kotlin、Java、Gradle 来构建核心逻辑。** `.so` 通过 JNI 由任意 Android 壳（如 Operit）加载即可运行。

---

## 配置 / Config

```yaml
# ~/.hermes/config.yaml
model: "deepseek-v4-flash"
api_endpoint: "https://your-api.com/v1/chat/completions"
api_key: ""
temperature: 0.7
max_tokens: 4096
```

---

## 上游项目 UA 分析数据 / Upstream UA Analysis

| 项目 | Nodes | Edges | Layers | Rust Files |
|---|---|---|---|---|
| Operit_MCPS | 1,505 | 1,556 | 5 | 659 |
| Understand_Anything_Rust | 1,641 | 1,623 | 3 | — |
| hermes-agent-rs | 700 | 694 | 4 | 352 |

> UA = Understand Anything 代码知识图谱分析。数据来自 `jinzechen/Understand_Anything_Rust` 扫描结果。

---

## 许可证 / License

MIT
