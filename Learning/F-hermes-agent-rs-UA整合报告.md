# hermes-agent-rs — UA 深度分析与 Hermes_Rust 整合报告

> **UA Rust 分析数据**：700 nodes / 694 edges / 4 layers / 579 files  
> **仓库**：https://github.com/Lumio-Research/hermes-agent-rs (72⭐, Rust, MIT)  
> **关键发现**：Hermes_Rust_Operit_App 可以且应该直接引入其 crate

---

## 一、UA Rust 发现的完整结构

### 18 个 crate 逐一分析

| crate | 文件数 | 行数 | 核心文件 |
|-------|--------|------|----------|
| **hermes-agent** | 39 | **24,655** | `agent_loop.rs` (7,001行!) |
| **hermes-gateway** | 45 | **27,066** | 多平台网关 |
| **hermes-tools** | 69 | **23,031** | 工具注册/调度 |
| **hermes-cli** | 29 | **14,011** | CLI 界面 |
| **hermes-server** | 14 | 3,116 | HTTP 服务 |
| **hermes-mcp** | 6 | 3,413 | MCP 协议 |
| **hermes-config** | 16 | 4,457 | 配置系统 |
| **hermes-core** | 7 | 1,579 | 类型/trait |
| **hermes-acp** | 7 | npm | ACP 协议 |
| **hermes-skills** | 6 | 1,829 | 技能系统 |
| **hermes-bus** | — | — | 事件总线 |
| **hermes-transport** | — | — | 传输层 |
| **hermes-cron** | — | — | 定时任务 |
| **hermes-environments** | — | — | 环境管理 |
| **hermes-eval** | — | — | 评估系统 |
| **hermes-intelligence** | — | — | 智能层 |
| **hermes-telemetry** | — | — | 遥测 |
| **hermes-parity-tests** | — | — | 测试 |

### 3 个 App（非核心，但提供参考）

| App | 技术栈 | 用途 |
|-----|--------|------|
| **dashboard** | React + Vite + shadcn | 管理后台 |
| **web-app** | React + Electron | 桌面端 |
| **mobile-app** | React Native + Expo | 移动端 (iOS) |

---

## 二、agent_loop.rs (7,001 行) 关键设计

```rust
// AgentLoop 的核心循环:
// 1. 发送消息 + 工具给 LLM
// 2. 如果 LLM 返回工具调用 → 并行执行（tokio JoinSet）
// 3. 继续循环直到 LLM 返回文本
// 4. 记忆管理（8 种插件）
// 5. 子 Agent 编排
// 6. 智能模型路由
// 7. 预算控制
```

### 对比 Hermes_Rust_Operit_App

| 能力 | hermes-agent-rs | Hermes_Rust_Operit_App |
|------|----------------|----------------------|
| Agent 循环 | 7,001 行 | ~200 行 |
| 记忆插件 | 8 种 | 1 种（memory.rs） |
| 工具调度 | 并行 JoinSet | 串行 |
| 子 Agent | ✅ | ❌ |
| 智能路由 | ✅ | ❌ |
| 预算控制 | ✅ | ❌ |
| MCP 集成 | 独立 crate | 自建 14.4KB |
| 技能系统 | SKILL.md 扫描 | ❌ |

---

## 三、可复用的 crate

### 核心 crate（必须引入）

```toml
[dependencies]
hermes-core = { git = "https://github.com/Lumio-Research/hermes-agent-rs" }
hermes-agent = { git = "..." }
hermes-tools = { git = "...", features = ["filesystem", "browser"] }
hermes-mcp = { git = "..." }
hermes-config = { git = "..." }
```

### 可选 crate（按需）

```toml
hermes-skills = { git = "..." }     # SKILL.md 扫描
hermes-cron = { git = "..." }       # 定时任务
hermes-eval = { git = "..." }       # Agent 评估
```

### 不需要的

- `hermes-gateway` — 多平台网关，Android 不需要
- `hermes-cli` — CLI 界面，Android 用 Dioxus UI
- `hermes-server` — HTTP 服务，不需要
- `apps/` — web/dashboard/mobile 不需要

---

## 四、整合路线

```
Step 1: types 统一
  hermes-core → Hermes_Rust_Operit_App 的类型系统
  （MessageRole, ToolCall, AgentConfig 等）

Step 2: Agent Loop 替换
  agent_loop.rs (7K) → 替换当前 agent.rs (~200 行)
  获得: 子 Agent / 记忆插件 / 智能路由 / 预算控制

Step 3: Tools 注册中心
  registry.rs + dispatch.rs → 替换当前 tool_registry.rs
  获得: 并行执行 / 动态注册 / 权限控制

Step 4: MCP 客户端替换
  hermes-mcp → 替换自建 mcp/client.rs (14.4KB)
  获得: 标准实现 / 更小代码量

Step 5: Skills 系统
  skill_orchestrator.rs → 新增 SKILL.md 扫描能力
```

### 整合后的架构变化

```
当前: 自建 Agent (200行) + 自建 MCP (14.4KB) + 5个工具
     ↓
整合后: hermes-agent (24.6K) + hermes-mcp (3.4K) + hermes-tools (23K)
     ↓
获得: 8种记忆 / 并行工具 / 子Agent / 智能路由 / 预算控制
```
