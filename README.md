# Hermes Rust Operit App

[![Build & Test](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions/workflows/build.yml/badge.svg)](https://github.com/jinzechen/Hermes_Rust_Operit_App/actions/workflows/build.yml)

纯 Rust 重写的 Operit Android AI 助手应用 — 高性能、低延迟、零 JVM 开销。

A pure Rust rewrite of the Operit Android AI assistant app — high performance, low latency, zero JVM overhead.

## 架构 Architecture

```
┌─────────────────────────────────────────┐
│         Dioxus UI (Android APK)          │
│   Chat | Store | Settings | Login       │
└───────────────┬─────────────────────────┘
                │
┌───────────────▼─────────────────────────┐
│       Rust Core (libhermes_operit_core)   │
│                                          │
│  Agent Loop ←→ LLM Providers            │
│  Tool Registry (filesystem/markdown/...) │
│  Memory Store (redb)                    │
│  MCP Client (stdio JSON-RPC 2.0)        │
│  Plugin Store (skills + MCP plugins)     │
│  Alpine Linux Environment (proot)        │
│  Sandbox (seccomp/landlock)             │
│  GitHub OAuth                            │
└──────────────────────────────────────────┘
```

## 模块 Modules

| Module | Path | Description |
|--------|------|-------------|
| **core/agent** | `src/core/agent.rs` | AgentManager — LLM ↔ Tool 循环 |
| **core/provider** | `src/core/provider.rs` | OpenAI / Anthropic / Generic LLM 实现 |
| **core/memory** | `src/core/memory.rs` | redb 持久化会话与偏好 |
| **core/config** | `src/core/config.rs` | 模型/温度/token 配置 |
| **tools/filesystem** | `src/tools/filesystem.rs` | 8 个文件操作 (读写/搜索/列表...) |
| **tools/markdown** | `src/tools/markdown.rs` | Markdown 渲染与代码提取 |
| **tools/browser** | `src/tools/browser.rs` | 浏览器工具 (obscura 待集成) |
| **tools/vision** | `src/tools/vision.rs` | 视觉工具 (agentic-vision 待集成) |
| **mcp/client** | `src/mcp/client.rs` | MCP stdio JSON-RPC 2.0 客户端 |
| **store** | `src/store/mod.rs` | Skill/MCP 插件管理 |
| **environment** | `src/environment/mod.rs` | Alpine Linux (proot + apk) |
| **ui/chat** | `src/ui/chat.rs` | 聊天数据模型 |
| **ui/login** | `src/ui/login.rs` | GitHub OAuth + PKCE |
| **ui/settings** | `src/ui/settings.rs` | YAML 设置持久化 |
| **ui/store_page** | `src/ui/store_page.rs` | 商店浏览器 (GitHub API) |

## 构建 Build

### 本地 Native Build
```bash
cargo build --release
cargo test
```

### Android APK (待 Dioxus 启用后)
```bash
rustup target add aarch64-linux-android
cargo install cargo-apk
cargo apk build --release
```

### CI/CD
GitHub Actions 自动执行:
- `cargo fmt --check` + `cargo clippy` + `cargo test` (每次推送)
- Android APK 构建 (手动触发 / `[android]` 提交标记)

## 状态 Status

- [x] Core: Agent loop, LLM providers, Tool registry, Memory (redb)
- [x] Tools: Filesystem (8 ops), Markdown, Browser/vision stubs
- [x] MCP: Full JSON-RPC 2.0 stdio client
- [x] Store: Plugin manager, skill/MCP install
- [x] Environment: Alpine proot, sandbox stubs
- [x] UI: Chat model, store browser, settings, GitHub OAuth
- [x] CI/CD: Native build+test, Android APK workflow
- [ ] Dioxus Android rendering
- [ ] obscura / agentic-vision native integration
- [ ] APK release

## License

MIT
