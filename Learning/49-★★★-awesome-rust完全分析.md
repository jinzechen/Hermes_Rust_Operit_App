# awesome-rust 完全分析 — Hermes 系统工具全集

> **源**：rust-unofficial/awesome-rust (58K⭐, 347KB README)  
> **覆盖**：之前漏掉的系统工具、缓存、认证类项目

---

## 一、系统工具补完

| Rust 项目 | 用途 | Hermes 角色 | 评分 |
|----------|------|-----------|------|
| **clipboard-rs** | 📋 跨平台剪贴板 | Agent 读写剪贴板 | ★★★★★ |
| **microsoft/mxc** | 🏖️ 沙盒执行系统 | 替代代码执行沙盒 | ★★★★ |
| **moka-rs/moka** | ⚡ 高性能缓存 | Token/API 结果缓存 | ★★★★ |
| **oauth2-rs** | 🔑 OAuth2 客户端 | GitHub 登录 | ★★★★ |
| **jsonwebtoken** | 🔐 JWT | API 认证 | ★★★ |
| **procs** | 📊 进程列表 | Agent 系统感知 | ★★★★ |
| **dust** | 💾 磁盘使用 | Agent 系统感知 | ★★★ |
| **ripgrep** | 🔍 文件搜索 | Agent 搜索文件 | ★★★★ |

---

## 二、Hermes 最终技术栈

```
┌──────────────────────────────────────────┐
│            Dioxus UI (Android)             │
│  ChatView | StoreView(三Tab) | Settings    │
└──────────────────┬───────────────────────┘
                   │
┌──────────────────▼───────────────────────┐
│         hermes-agent-rs (Agent引擎)       │
│  agent_loop | ToolRegistry | MemoryManager │
│  sub_agent | smart_routing | budget       │
├──────────────────────────────────────────┤
│         hermes-tools (35工具)              │
│  browser | file | web | terminal | tts    │
│  + clipboard/cron/delegation/skills      │
├──────────────────────────────────────────┤
│         Android JNI 桥接                   │
│  Shizuku | Accessibility | Foreground    │
│  Notification | Termux | SAF             │
├──────────────────────────────────────────┤
│         本地 AI 能力                       │
│  mistral.rs (推理) | sherpa-onnx (语音)   │
│  candle+EmbedAnything (嵌入)              │
├──────────────────────────────────────────┤
│         安全与存储                         │
│  wasmer (沙盒) | redb (KV)               │
│  tinycortex (记忆) | moka (缓存)          │
│  oauth2 + aes-gcm (安全)                 │
└──────────────────────────────────────────┘
```

### 评分：XX 个项目全覆盖

Hermes_Rust_Operit_App 所需的所有系统级能力在 awesome-rust 中都已找到对应的 Rust 方案。
