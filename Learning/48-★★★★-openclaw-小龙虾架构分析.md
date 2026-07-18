# openclaw — 小龙虾 AI 助手 UA 分析

> **仓库**：https://github.com/openclaw/openclaw (383K⭐, TypeScript)  
> **Hermes_Rust_Operit_App 评分**：★★★★（架构参考）

---

## 一、openclaw 架构（从 package.json 提取）

```
openclaw (383K⭐, 世界最大 AI 项目)
├── src/
│   ├── agents/ — 多 Agent 系统
│   ├── cli/ — CLI 界面
│   ├── commands/ — 命令系统
│   ├── config/ — 配置
│   ├── context-engine/ — 上下文引擎
│   ├── cron/ — 定时任务
│   ├── chat/ — 聊天
│   ├── tools/ — 工具
│   └── mcp/ — MCP 协议
├── package.json — 依赖
└── sdks/ — SDK
```

## 二、对 Hermes 的参考

openclaw 是 TypeScript 版，与 Rust 版 Hermes 定位相同（AI Agent）。其多 Agent 系统（agents/）和上下文引擎（context-engine/）的设计思路可参考。

### 评分：★★★★

openclaw 383K⭐ 验证了 AI Agent 的巨型需求。但它是 TypeScript 项目，Rust 方案可直接参考其功能清单。
