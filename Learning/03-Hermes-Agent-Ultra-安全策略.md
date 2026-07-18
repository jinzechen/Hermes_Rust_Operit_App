# 03 — Hermes Agent Ultra 安全策略 深度学习报告

> **UA Rust 分析时间**：2026-07-18  
> **项目**：https://github.com/sheawinkler/hermes-agent-ultra (72⭐, Rust)  
> **定位**：NousResearch/hermes-agent 的 Rust 重写版 + 安全增强层  
> **Hermes 集成现状**：❌ 尚未集成，但安全策略架构可直接借鉴

---

## 第一步：UA Rust 深度扫描

```bash
# 扫描项目
ua scan /d/.../hermes-agent-ultra
# → 3 文件（README.md + Cargo.toml + meta.json）

# 构建知识图谱
ua build /d/.../hermes-agent-ultra output/hermes-agent-ultra.json
ua build /d/.../hermes-agent-ultra output/hermes-agent-ultra.md --format=md
```

### UA Rust 发现的项目结构

```
项目概览：
  文件数: 3 (仅初始下载)
  知识图谱: 4 节点 / 1 边
  复杂度: Simple（局部样本）

真实规模: Cargo.toml (3.6KB) 定义 22 crate 工作区
├── hermes-skills/          → Skill guard 检测
├── hermes-tool-planning/   → 工具策略控制
├── hermes-intelligence/    → 数据脱敏
├── hermes-plugin/          → 插件中间件
├── hermes-observers/       → 观察者钩子
├── hermes-sandbox/         → 沙箱执行
└── 更多...
```

---

## 第二步：6 层安全架构详解

| 层级 | 模块 | 作用 | Hermes 状态 |
|------|------|------|-------------|
| L1: Skill 扫描 | `guard.rs` | 19种危险模式 + 9种 prompt injection | ❌ 无 |
| L2: 工具策略 | `tool-planning` | strict/balanced/dev/relaxed 预设 | ✅ 有 tool_registry 但无策略 |
| L3: 插件中间件 | PreToolCall hook | 可 block/修改请求和响应 | ❌ 无 hook 系统 |
| L4: 脱敏 | `redact.rs` | 30+ 密钥前缀 + PII | ❌ 无 |
| L5: 网络隔离 | Docker/Squid | 出口白名单 | ❌ 无 |
| L6: 预算控制 | 调速器 | max_turns + 成本 + 模型切换 | ❌ 无 |

### Ultra 特有能力

- **会话分支 + 时间旅行** — TUI 中 checkpoint/rollback/replay
- **工具调用模拟器** — 预览策略 allow/deny 结果
- **记忆融合 (ContextLattice)** — 多来源记忆评分/融合
- **Provider QoS 路由** — 自动选择健康/便宜的模型
- **语义仓库图** — 依赖枢纽/边界的 Mermaid 预览

---

## 第三步：对 Hermes_Rust_Operit_App 的作用分析

### 当前 Hermes 的安全状况

Hermes_Rust_Operit_App 目前**没有安全层**——Agent 可随意调用所有工具，无敏感数据脱敏，无策略控制。

### 差距分析

| 安全维度 | Hermes_Rust_Operit_App | Hermes Agent Ultra |
|----------|----------------------|-------------------|
| Prompt injection 防护 | ❌ | ✅ 9 种检测 |
| 工具策略控制 | ❌ | ✅ 4 种预设 |
| 敏感数据脱敏 | ❌ | ✅ 30+ 密钥前缀 |
| 会话回放 | ❌ | ✅ ReplayRecorder |
| 预算控制 | ❌ | ✅ max_turns + 成本 |

### 集成优先级

1. **最高**: L1 Skill guard（prompt injection 防护）
2. **最高**: L4 脱敏引擎（防止密钥泄露）
3. **高**: L2 工具策略预设
4. **中**: L3 PreToolCall hook
5. **低**: L5 网络隔离（Android 上不同）

---

## 第四步：三到五个 Hermes 可复用点

| # | 可复用点 | 具体做法 |
|---|---------|----------|
| 1 | **Prompt injection 检测** | copy `guard.rs` 的 9 种检测模式到 Hermes tools 层 |
| 2 | **脱敏引擎** | 在 tool_registry 中插入 redact 过滤器 |
| 3 | **工具策略预设** | strict/balanced/dev 三种模式，用户可切换 |
| 4 | **检查点 + 回放** | Agent 每步保存会话快照，支持调试 |
| 5 | **Fail-open 设计** | 钩子异常不阻断 Agent（Ultra 的关键设计原则） |
