# ZeptoClaw — 轻量 Rust AI Agent UA 源码分析

> **UA Rust 分析**：lib.rs (3.5KB) — 25+ 模块  
> **仓库**：https://github.com/qhkm/zeptoclaw (644⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★（轻量架构参考）

---

## 一、UA Rust 发现的模块

```
zeptoclaw (~4MB 二进制, 7 层安全)
├── agent — Agent 核心
├── api — API 层
├── audit — 审计
├── auth — 认证 (7 层安全)
├── batch — 批处理
├── bus — 事件总线
├── cache — 缓存
├── channels — 多通道
├── config — 配置
├── cron — 定时任务
├── deps — 依赖
├── devices — 设备管理
├── error — 错误处理
├── gateway — 网关
├── hands — 操作
├── memory — 记忆
├── mcp — MCP 协议
├── network — 网络
├── oauth — OAuth
├── providers — LLM 提供商
├── sandbox — 沙盒
├── shell — Shell 执行
├── skills — 技能
├── tools — 工具
├── ui — UI
└── workspace — 工作区
```

---

## 二、与 Hermes 对照

| 模块 | ZeptoClaw | Hermes |
|------|-----------|--------|
| agent | ✅ | hermes-agent |
| mcp | ✅ | RMCP |
| sandbox | ✅ 7 层安全 | wasmer |
| memory | ✅ | tinycortex |
| skills | ✅ | hermes-skills |
| tools | ✅ | hermes-tools |
| providers | ✅ | provider.rs |
| cron | ✅ | hermes-cron |
| shell | ✅ | terminal.rs |
| oauth | ✅ | oauth2-rs |
| cache | ✅ | moka |

**关键发现**：ZeptoClaw 的模块划分与 Hermes_Rust_Operit_App 几乎完全一致，可作为架构参考。

### 评分：★★★★

ZeptoClaw 是轻量级 Rust AI Agent 的优秀参考（4MB 二进制，7 层安全）。其模块划分可验证 Hermes 的架构设计。
