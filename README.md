# Hermes Rust Operit App

> 纯 Rust 重构的 Android AI 助手 — 100% Rust, 0% Kotlin/Java/Gradle  
> Pure Rust Android AI Assistant — 53 `.rs` files, zero non-Rust code

[![CI](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions/workflows/ci.yml/badge.svg)](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions)
[![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

---

## 项目定位 / What is this?

Hermes_Rust_Operit_App 是对 [Operit](https://github.com/AAswordman/Operit) (5.8K⭐ Android AI助手) 和 [HermesApp](https://github.com/SelectXn00b/HermesApp) (Hermes内核+Operit壳的融合) 的 **纯 Rust 复刻与增强**。

**不是从零造轮子** — 而是组合 Rust 生态中最优秀的项目，在 Android 上打造一个高性能、全功能的 AI Agent 操作系统。

Hermes_Rust_Operit_App is a **pure Rust rewrite** of [Operit](https://github.com/AAswordman/Operit) and [HermesApp](https://github.com/SelectXn00b/HermesApp), combining the best Rust ecosystem projects into a single Android AI agent OS.

**Not reinventing wheels** — it assembles the finest Rust crates to deliver high-performance agent capabilities natively on Android.

---

## 上游项目 / Upstream Projects

本项目深度参考并依赖以下 Rust 生态项目（按星级排序）：

This project deeply references and depends on the following Rust ecosystem projects (sorted by stars):

### ★★★★★ 核心依赖 / Core Dependencies

| 项目 / Project | Stars | 用途 / Role |
|---------------|-------|-------------|
| [rtk-ai/rtk](https://github.com/rtk-ai/rtk) | 71,641 | LLM Token 优化代理 / Token optimization proxy |
| [rust-unofficial/awesome-rust](https://github.com/rust-unofficial/awesome-rust) | 58,000 | Rust 生态全景扫描 / Ecosystem survey |
| [DioxusLabs/dioxus](https://github.com/DioxusLabs/dioxus) | 36,804 | Rust UI 框架 (Android 原生) / Cross-platform UI |
| [tinyhumansai/openhuman](https://github.com/tinyhumansai/openhuman) | 35,015 | 记忆引擎 / Memory engine (tinycortex) |
| [wasmerio/wasmer](https://github.com/wasmerio/wasmer) | 20,904 | WebAssembly 沙盒 / Wasm sandbox runtime |
| [huggingface/candle](https://github.com/huggingface/candle) | 20,686 | 纯 Rust ML 框架 / Pure Rust ML |
| [h4ckf0r0day/obscura](https://github.com/h4ckf0r0day/obscura) | 19,392 | 无头浏览器 / Headless browser (CDP) |
| [k2-fsa/sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) | 13,630 | 语音引擎 / Voice (ASR/TTS/VAD) |
| [EricLBuehler/mistral.rs](https://github.com/EricLBuehler/mistral.rs) | 7,492 | 本地 LLM 推理 / Local inference |
| [AAswordman/Operit](https://github.com/AAswordman/Operit) | 5,800 | Android AI 助手原版 / Original Android AI Assistant |
| [modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk) | 3,639 | MCP 官方 Rust SDK / Official MCP protocol |
| [lorryjovens-hub/claude-code-rust](https://github.com/lorryjovens-hub/claude-code-rust) | 1,667 | Claude Code Rust 重写参考 / Architecture reference |
| [StarlightSearch/EmbedAnything](https://github.com/StarlightSearch/EmbedAnything) | 1,283 | 本地嵌入 / Local embeddings |
| [thClaws/thClaws](https://github.com/thClaws/thClaws) | 1,166 | Rust AI Agent 参考 / Agent reference |
| [qhkm/zeptoclaw](https://github.com/qhkm/zeptoclaw) | 644 | 轻量 Agent 架构参考 / Lightweight agent |
| [thewh1teagle/sherpa-rs](https://github.com/thewh1teagle/sherpa-rs) | 310 | sherpa-onnx Rust 绑定 / Rust bindings |
| [SelectXn00b/HermesApp](https://github.com/SelectXn00b/HermesApp) | 194 | Hermes+Operit 融合参考 / Fusion reference |
| [Lumio-Research/hermes-agent-rs](https://github.com/Lumio-Research/hermes-agent-rs) | 72 | Hermes Agent Rust 内核 / Agent kernel |
| [sheawinkler/hermes-agent-ultra](https://github.com/sheawinkler/hermes-agent-ultra) | 72 | 安全增强层 / Security layer |
| [jinzechen/Operit_MCPS](https://github.com/jinzechen/Operit_MCPS) | — | MCP 插件集 (用户自建) / MCP plugins |
| [jinzechen/Understand_Anything_Rust](https://github.com/jinzechen/Understand_Anything_Rust) | — | 代码分析引擎 (用户自建) / Code analysis |
| [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) | — | 原版 Hermes Agent / Original Hermes |

### ★★★★ 参考项目 / Reference Projects

[openclaw](https://github.com/openclaw), [moka](https://github.com/moka-rs/moka), [tantivy](https://github.com/quickwit-oss/tantivy), [rhai](https://github.com/rhaiscript/rhai), [piccolo](https://github.com/kyren/piccolo), [boa](https://github.com/boa-dev/boa), [qdrant](https://github.com/qdrant/qdrant), [rustdesk](https://github.com/rustdesk/rustdesk), [openinterpreter](https://github.com/OpenInterpreter/open-interpreter)

**完整 63 份学习报告及分析见 `/Learning` 目录。**  
**Full 63 learning reports in `/Learning` directory.**

---

## 功能 / Features

| 模块 / Module | 功能 / Feature | 来源 / From |
|--------------|---------------|-------------|
| **Agent 引擎** | ReAct Loop + SmartModelRouter + 子Agent编排 | hermes-agent-rs |
| **工具系统** | 11 个内置 ToolHandler (0ms 开销) | hermes-tools + Operit_MCPS |
| **文件系统** | 25+ 操作 (读/写/搜索/压缩/校验) | rust_mcp_filesystem |
| **浏览器** | CDP 无头浏览器 (导航/截图/点击/JS) | obscura |
| **网页搜索** | DuckDuckGo + moka 缓存 | reqwest + scraper |
| **终端** | Shell 执行 + 超时控制 | hermes-tools |
| **进程管理** | 后台进程 spawn/kill/wait/status | tokio::process |
| **定时任务** | Cron 表达式调度 (30m/2h/0 9 * * *) | hermes-cron |
| **语音** | ASR + TTS + VAD (sherpa-onnx) | sherpa-rs |
| **MCP 协议** | JSON-RPC 2.0 over stdio + rmcp | rmcp |
| **记忆系统** | redb + tantivy 全文索引 | openhuman |
| **安全层** | 19种危险命令检测 + 30+密钥脱敏 | hermes-agent-ultra |
| **插件商店** | GitHub Releases 分发 + 安装/更新 | Operit_MCPS |
| **沙盒** | 进程隔离 + 超时 + 路径白名单 | wasmer |
| **代码分析** | 知识图谱构建 (scan→parse→graph→report) | UA Rust |
| **缓存** | moka 高性能缓存 (搜索结果/Token) | moka |
| **OAuth** | GitHub OAuth 登录 | oauth2 |

---

## 构建 / Build

### CLI 测试 (Windows/Linux/macOS)

```bash
git clone https://github.com/jinzechen/Hermes_Rust_Operit_App.git
cd Hermes_Rust_Operit_App

# Clone dependency
git clone https://github.com/jinzechen/Understand_Anything_Rust.git ../Understand_Anything_Rust

# Build & run
cargo build --release
./target/release/hermes_operit_app
```

### Android 构建 (需要 NDK)

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk

cargo ndk -t aarch64-linux-android -p 26 build --release
# → target/aarch64-linux-android/release/libhermes_operit_core.so
```

### 配置 / Config

```yaml
# ~/.hermes/config.yaml
model: "deepseek-v4-flash"
api_endpoint: "https://token-plan-sgp.xiaomimimo.com/v1/chat/completions"
api_key: ""
temperature: 0.7
max_tokens: 4096
```

---

## 项目结构 / Project Structure

```
src/
├── core/          # Agent引擎, Provider, 配置, 记忆, 安全
├── tools/         # 11 个 ToolHandler (filesystem/browser/web/vision/markdown/terminal/process/cronjob/speech/codebase_analyzer)
├── mcp/           # MCP JSON-RPC 客户端 + 管理器
├── store/         # 插件商店 (索引/下载/安装/管理)
├── environment/   # 沙盒 + Ubuntu proot
├── android/       # JNI 桥接 (纯 Rust, cfg-gated)
├── ui/            # Dioxus UI 组件 (Chat/Market/Toolbox/Memory/Settings)
tests/             # 集成测试
Learning/          # 63 份学习报告
```

**0 个 .kt 文件 · 0 个 .xml 文件 · 0 个 Gradle 文件**

---

## 测试 / Tests

```bash
cargo test --lib
# 149 passed, 0 failed, 6 ignored
```

---

## 许可证 / License

MIT
