# 07 — rtk + cc-switch 优化与路由 深度学习报告

> **rtk**：https://github.com/rtk-ai/rtk (71,641⭐, Rust)  
> **cc-switch**：https://github.com/farion1231/cc-switch (118,513⭐, Rust)  
> **Hermes 集成现状**：❌ 未集成

---

## 第一步：UA Rust 深度扫描

```bash
ua scan analysis/rtk → 1 文件（README.md）
ua build → 3 节点 / 0 边
```

## 第二步：rtk — LLM Token 优化代理

Rust CLI 代理，将 LLM token 消耗降低 **60-90%**。

```
用户请求 → rtk 代理 → LLM API
         ↓
  缓存层（常见命令预缓存）
  压缩层（输出精简）
  路由层（按价格/速度选模型）
```

## 第三步：cc-switch — AI Agent 桌面中心

118k stars 的跨平台桌面中心，统一管理 Claude Code / Codex 等 AI Agent。

## 第四步：对 Hermes_Rust_Operit_App 的作用

| rtk 能力 | Hermes 对应 | 差距 |
|----------|-------------|------|
| Token 缓存 | 无 | ❌ |
| 模型路由 | provider.rs | 有基本路由 |
| 输出压缩 | 无 | ❌ |
| 成本追踪 | 无 | ❌ |

## 第五步：三到五个可复用点

1. **Token 缓存层** — 常见 prompt 预缓存节省 60-90% token
2. **模型路由** — 按任务智能选择便宜/快模型
3. **成本追踪** — 每次调用费用记录
4. **cc-switch 多 Agent 管理** — 桌面仪表盘模式
