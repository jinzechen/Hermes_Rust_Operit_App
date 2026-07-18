# awesome-claws — Rust AI Agent 生态全发现

> **源**：machinae/awesome-claws (478⭐) — OpenClaw 启发的 AI Agent 列表  
> **Hermes_Rust_Operit_App 对比分析**

---

## 一、awesome-claws Rust AI Agent 对比

| 项目 | Stars | 特色 | 与 Hermes 的重合度 |
|------|-------|------|-------------------|
| **ZeptoClaw** | 644⭐ | ~4MB, 7层安全, 本地优先 | ★★★★★ |
| **Moltis** | — | 个人 AI 网关, 单二进制, 沙盒+语音+MCP | ★★★★★ |
| **IronClaw** | — | 隐私安全, 本地加密 | ★★★★ |
| **OpenFang** | — | Agent OS, 137K LOC, 14 crates | ★★★★★ |
| **ZeroClaw** | — | Trait 驱动, 零开销 AI 基础设施 | ★★★★ |
| **moxxy** | — | 自托管多 Agent 框架 | ★★★★ |
| **shrew** | — | 紧凑自主助手 | ★★★ |
| **Microclaw** | — | 聊天表面 AI 助手 | ★★★ |
| **OpenCrabs** | — | 自改进 AI Agent | ★★★ |
| **thClaws** | 1,166⭐ | 已分析 (52) | ★★★★★ |
| **goose** | — | 已分析 (59) | ★★★★★ |
| **claude-code-rust** | 1,667⭐ | 已分析 (60) | ★★★★★ |

---

## 二、Hermes_Rust_Operit_App 与生态共性

所有 Rust AI Agent 的共同模块：

```
agent / mcp / tools / skills / memory / providers / shell / cron / auth / config
```

Hermes_Rust_Operit_App 已覆盖上述全部模块。生态验证了 Hermes 架构的正确性。

### 差异化优势

| 维度 | 其他 Rust Agent | Hermes_Rust_Operit_App |
|------|----------------|----------------------|
| Android | ❌ 桌面/CLI | ✅ **手机 Agent** |
| 沙盒 | wasmer/bwrap | wasmer + JNI |
| UI | Tauri 桌面 | Dioxus Android |
| 规模 | 4MB-137K LOC | hermes-agent-rs 100K LOC |

### 总评

Rust AI Agent 生态正在快速增长（ZeptoClaw/IronClaw/Moltis/OpenFang等）。
Hermes_Rust_Operit_App 的 Android 原生定位是核心差异化优势。
