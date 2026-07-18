# goose — Linux Foundation Rust AI Agent UA 源码分析

> **UA Rust 分析**：Cargo.toml (7.8KB)  
> **仓库**：https://github.com/aaif-goose/goose (Linux Foundation)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（功能最重合的项目之一）

---

## 一、goose 与 Hermes 对比

| 维度 | goose | Hermes_Rust_Operit_App |
|------|-------|----------------------|
| **语言** | Rust ✅ | Rust |
| **桌面端** | ✅ 桌面 App | ❌ Android 优先 |
| **Android** | ❌ | ✅ **核心目标** |
| **15+ Provider** | ✅ | hermes-agent provider |
| **MCP** | ✅ 70+ 扩展 | RMCP |
| **Shell 集成** | ✅ CLI | terminal.rs |
| **Workflow** | ✅ | skill_orchestrator |

---

## 二、goose 的关键参考

goose 是 Linux Foundation 支持的项目（AAIF），证明了 Rust AI Agent 的市场需求：

- **Desktop app**: Tauri 2 + Rust
- **CLI**: 全终端集成
- **API**: HTTP 服务
- **Provider**: 15+ LLM 提供商
- **MCP**: Model Context Protocol 标准

### 对 Hermes 的启示

```
goose 证明: Rust AI Agent + MCP + 多 Provider = 已验证模式
Hermes 差异化: Android 原生（手机 Agent）
```

### 评分：★★★★★

goose 与 Hermes_Rust_Operit_App 的功能高度重合，但定位在桌面端。Hermes 的 Android 原生是核心差异。
