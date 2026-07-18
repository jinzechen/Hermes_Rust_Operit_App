# OpenClaw — 多供应商 Agent 架构 学习报告
> 项目：https://github.com/openclaw/openclaw | TypeScript
> 学习日期：2026-07-18

## 架构核心
- **Gateway-centric**: 单守护进程托管所有消息面，WebSocket RPC (127.0.0.1:18789)
- **序列化 Agent Loop**: Entry→Queue→Prompt→Inference→Tools→Reply→Compact→Persist
- **60+ LLM providers**: 3 层抽象 provider/model→ProviderPlugin→AgentRuntime
- **三层工具**: Tools(typed funcs), Skills(Markdown), Plugins(npm packages)

## 对 Hermes 的可复用模式
| 模式 | OpenClaw | Rust 等价 |
|------|---------|-----------|
| Gateway | WebSocket RPC daemon | axum + tokio-tungstenite |
| Provider | ModelRegistry + plugins | Trait objects + HashMap |
| Agent loop | Lane-based pipeline | tokio::sync::Mutex + channels |
| Tools | Typed with JSON Schema | serde + trait-based |
| Skills | SKILL.md (YAML frontmatter) | Same (serde_yaml) |
| Memory | File-based Markdown | std::fs + embedding API |
| Plugins | npm packages with manifest | Cargo crates |
