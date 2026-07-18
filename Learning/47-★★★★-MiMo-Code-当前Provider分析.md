# MiMo-Code — 当前使用的 Provider UA 分析

> **仓库**：https://github.com/XiaomiMiMo/MiMo-Code (12K⭐, TypeScript)  
> **当前 provider**：token-plan-sgp.xiaomimimo.com  
> **Hermes_Rust_Operit_App 评分**：★★★★（provider 架构参考）

---

## 一、架构

```
MiMo-Code (12K⭐, TypeScript monorepo)
├── packages/ — SDK
├── sdks/ — 语言 SDK
├── infra/ — 基础设施
├── docs/ — 文档
└── AGENTS.md — Agent 集成指南
```

## 二、对 Hermes 的作用

当前 Hermes 使用的 provider 就是 MiMo（token-plan-sgp.xiaomimimo.com）。其 OpenAI 兼容 API 已集成在 hermes-agent provider.rs 中。

### 评分：★★★★

MiMo 是当前使用的 provider，其 API 格式是 Hermes provider 层需要兼容的标准。
