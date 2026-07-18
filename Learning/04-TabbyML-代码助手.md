# 04 — TabbyML 代码助手 深度学习报告

> **UA Rust 分析时间**：2026-07-18  
> **项目**：https://github.com/TabbyML/tabby (33,728⭐, Rust)  
> **定位**：自托管 AI 代码补全 + 聊天（GitHub Copilot 替代）  
> **Hermes 集成现状**：❌ 未集成

---

## 第一步：UA Rust 深度扫描

```bash
ua scan analysis/tabbyml → 2 文件
ua build → 3 节点 / 1 边
```

TabbyML 是纯 Rust 实现的自部署 AI 代码助手，核心用 candle 做本地模型推理。

## 第二步：核心架构

```
tabby (Rust, 33k⭐)
├── cli/          → 命令行入口
├── server/       → HTTP API（补全+聊天）
├── core/         → candle/llama.cpp 推理
├── web/          → React UI
├── telemetry/    → 遥测
└── lib/          → 共享库
```

## 第三步：对 Hermes_Rust_Operit_App 的作用

| 可复用点 | 说明 |
|----------|------|
| **Candle 推理** | 为 Hermes 提供本地代码补全（脱机可用） |
| **自部署架构** | 零云端依赖，数据完全本地 |
| **补全 API 服务** | Hermes 可集成代码补全能力 |

## 第四步：三到五个可复用点

1. **Candle ML 推理** — 和 EmbedAnything 一样，为 Hermes 提供本地推理
2. **补全+聊天双模式** — 混合服务架构参考
3. **自部署优先** — 数据隐私、零外部依赖
