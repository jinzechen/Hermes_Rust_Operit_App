# Hermes_Rust_Operit_App — 纯 Rust Android AI 助手

> 基于 18 个 Rust 上游项目，100% 纯 Rust 的 Android AI Agent  
> 100% pure Rust Android AI Agent, assembled from 18 upstream Rust projects

[![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![CI](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions/workflows/ci.yml/badge.svg)](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions)

**0 个 .kt · 0 个 .xml · 0 个 Gradle — 纯 Rust，从 UI 到推理全链路。**

---

## 上游项目功能对比 / Upstream Feature Comparison

| 上游项目 | Stars | 原功能 | 本项目复用 | 当前状态 |
|---|---|---|---|---|
| [rtk-ai/rtk](https://github.com/rtk-ai/rtk) | 71.6K | Token 优化代理 | Token 路由 + 缓存管线 | 🔧 设计 |
| [DioxusLabs/dioxus](https://github.com/DioxusLabs/dioxus) | 36.8K | Rust 跨平台 UI | Android 原生 UI (Chat/Market/Settings) | 🔧 UI 组件已写 |
| [tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman) | 35.0K | AI 记忆引擎 | redb + tantivy 全文记忆索引 | ✅ 已实现 |
| [wasmerio/wasmer](https://github.com/wasmerio/wasmer) | 20.9K | Wasm 沙盒 | 进程隔离沙盒 (超时/白名单/截断) | ✅ 已实现 |
| [h4ckf0r0day/obscura](https://github.com/h4ckf0r0day/obscura) | 19.4K | CDP 无头浏览器 | **需 obscura 二进制** — 截图/点击/JS执行/填表 | 🔜 Phase 3 |
| [k2-fsa/sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) | 13.6K | 语音 ASR/TTS/VAD | sherpa-rs → 语音输入输出 | 🔜 Phase 3 |
| [EricLBuehler/mistral.rs](https://github.com/EricLBuehler/mistral.rs) | 7.5K | 本地 LLM 推理 | 离线推理 (Qwen2.5-1.5B Q4) | 🔜 Phase 3 |
| [AAswordman/Operit](https://github.com/AAswordman/Operit) | 5.8K | Android AI 助手 | 功能清单 + UI 布局参考 | ✅ 参考 |
| [modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk) | 3.6K | MCP 官方 SDK | rmcp → JSON-RPC 2.0 + 自建 client | ✅ 已实现 |
| [scraper](https://crates.io/crates/scraper) | 2K+ | HTML 解析 | 网页搜索 (DuckDuckGo) + 页面抓取 | ✅ 已实现 |
| [lorryjovens-hub/claude-code-rust](https://github.com/lorryjovens-hub/claude-code-rust) | 1.7K | 20 模块 Agent | Agent Loop + Tool 调度架构 | ✅ 参考 |
| [moka](https://github.com/moka-rs/moka) | 1.5K+ | 高性能缓存 | 搜索结果 + Token 缓存 | ✅ 已实现 |
| [thClaws/thClaws](https://github.com/thClaws/thClaws) | 1.2K | 3-Tab/Skills/MCP | Agent 交互模式 + MCP 集成 | ✅ 参考 |
| [tantivy](https://github.com/quickwit-oss/tantivy) | 12K+ | 全文搜索 | 本地 FTS 索引 (会话/记忆) | ✅ 已实现 |
| [qhkm/zeptoclaw](https://github.com/qhkm/zeptoclaw) | 644 | 25 模块 Agent | 模块化拆分参考 | ✅ 参考 |
| [SelectXn00b/HermesApp](https://github.com/SelectXn00b/HermesApp) | 194 | Hermes+Operit 融合 | Agent 内核 + Android 壳集成 | ✅ 参考 |
| [Lumio-Research/hermes-agent-rs](https://github.com/Lumio-Research/hermes-agent-rs) | 72 | Agent 引擎 | ReAct Loop + 子Agent 编排 | ✅ 参考 |
| [jinzechen/Operit_MCPS](https://github.com/jinzechen/Operit_MCPS) | — | 9 个 MCP 插件 | MCP 工具注册 + 插件商店 | ✅ 5内置化 |
| [jinzechen/Understand_Anything_Rust](https://github.com/jinzechen/Understand_Anything_Rust) | — | 代码知识图谱 | scan→parse→graph→report 管线 | ✅ 已集成 |

---

## 功能清单 / Features

| 模块 | 功能 | 当前状态 |
|---|---|---|
| **Agent 引擎** | ReAct Loop + SmartModelRouter + TokenUsage | ✅ |
| **文件系统** | 22 操作 (读/写/搜索/压缩/校验/批量) | ✅ |
| **浏览器** | HTTP 网页抓取 + HTML 解析 (obscura CDP 🔜) | 🔧 |
| **网页搜索** | DuckDuckGo + moka 缓存 | ✅ |
| **终端** | Shell 执行 + 超时控制 + 输出截断 | ✅ |
| **进程管理** | 后台进程 spawn/kill/wait/status | ✅ |
| **定时任务** | Cron 调度 (30m/2h/0 9 * * *) | ✅ |
| **语音** | ASR/TTS/VAD (sherpa-onnx, Phase 3) | 🔜 |
| **MCP 协议** | JSON-RPC 2.0 + rmcp 客户端 | ✅ |
| **记忆系统** | redb + tantivy 全文索引 | ✅ |
| **安全层** | 19 种危险命令 + 30+ 密钥脱敏 | ✅ |
| **插件商店** | GitHub Releases 索引/下载/安装/管理 | ✅ |
| **沙盒** | 进程隔离 + 超时 + 路径白名单 | ✅ |
| **代码分析** | UA Rust 知识图谱 (已有 74 份分析) | ✅ |
| **OAuth** | GitHub OAuth 登录 | ✅ |

---

## Android 构建 / Build

```bash
# 前置条件 / Prerequisites
rustup target add aarch64-linux-android
cargo install cargo-ndk
git clone https://github.com/jinzechen/Understand_Anything_Rust.git ../Understand_Anything_Rust

# 构建 / Build
cd Hermes_Rust_Operit_App
cargo ndk -t aarch64-linux-android -p 26 build --release
# → target/aarch64-linux-android/release/libhermes_operit_core.so

# 配合 Dioxus 打包 APK / With Dioxus for APK
cargo add dioxus-mobile
dx build --platform android
```

**不需要 Kotlin/Gradle/Android Studio。纯 Rust。**

---

## UA 源码分析数据 / UA Source Analysis Data

| 项目 | 总文件 | Rust 文件 | 节点 | 边 | 层 |
|---|---|---|---|---|---|
| Operit_MCPS | 1,229 | 659 | 1,505 | 1,556 | 5 |
| thClaws | 641 | 260 | 723 | 711 | 4 |
| tantivy | 517 | 433 | 626 | 602 | 3 |
| zeptoclaw | 493 | 307 | 597 | 574 | 4 |
| UA Rust (自分析) | — | 28 | 1,641 | 1,623 | 3 |
| rhai | 408 | 272 | 448 | 431 | 3 |
| hermes-agent-rs | 579 | 352 | 700 | 694 | 4 |

## 已编译集成的二进制 / Compiled Binaries

| 二进制 | 大小 | 来源 | 用途 |
|---|---|---|---|
| `thclaws.exe` | 59MB | thClaws/thClaws (源码编译) | Rust AI Agent |
| `thclaws-cli.exe` | 55MB | thClaws/thClaws (源码编译) | Agent CLI |
| `hermes.exe` | 25MB | Lumio-Research/hermes-agent-rs (源码编译) | Hermes Agent |
| `hermes-server.exe` | 16MB | Lumio-Research/hermes-agent-rs (源码编译) | HTTP Server |
| `obscura.exe` | 59MB | h4ckf0r0day/obscura v0.1.10 | CDP 无头浏览器 |
| `obscura-worker.exe` | 55MB | h4ckf0r0day/obscura v0.1.10 | 浏览器 Worker |
| `ua.exe` | 4.5MB | jinzechen/Understand_Anything_Rust (源码编译) | 代码知识图谱 |
| `rhai-run.exe` | 2.9MB | rhaiscript/rhai (源码编译) | 嵌入式脚本引擎 |

> 下载: `bash scripts/setup.sh`

---

## 参考 / References

[Operit](https://github.com/AAswordman/Operit) · [HermesApp](https://github.com/SelectXn00b/HermesApp) · [Hermes Agent](https://github.com/NousResearch/hermes-agent) · [Operit Docs](https://operit.app/#/guide)

---

MIT License
