# Hermes Agent Ultra — 深度学习报告（含 UA Rust 分析）

> 仓库：https://github.com/sheawinkler/hermes-agent-ultra  
> Stars：72 | License：— | 语言：Rust  
> 核心：22 crate workspace，多层安全策略，确定性执行

---

## 一、UA Rust 深度分析

| 指标 | 值 |
|------|----|
| 文件数 | 2（Cargo.toml 3.6KB = 22 crates 工作区） |
| 架构节点 | 6 |

### 工作区结构（Cargo.toml 解析）

```
hermes-agent-ultra (22 crates)
├── hermes-skills/          → Skill 内容扫描 + guard 检测
├── hermes-tool-planning/   → 工具策略控制
├── hermes-intelligence/    → 敏感数据脱敏
├── hermes-plugin/          → 插件中间件系统
├── hermes-observers/       → 观察者钩子
├── hermes-sandbox/         → 沙箱执行
└── 更多 crates...
```

---

## 二、安全策略架构（6 层）

| 层级 | 模块 | 作用 |
|------|------|------|
| L1: Skill 扫描 | `guard.rs` | 19种危险模式 + 9种 prompt injection 检测 |
| L2: 工具策略 | `tool-planning` | strict/balanced/dev/relaxed 预设 |
| L3: 插件中间件 | PreToolCall hook | 可 block 或修改工具请求/响应 |
| L4: 脱敏 | `redact.rs` | 30+ 密钥前缀 + PII（邮箱/电话/信用卡） |
| L5: 网络隔离 | Docker/Squid | 出口白名单 |
| L6: 预算控制 | 调速器 | max_turns + 成本追踪 + 模型切换 |

---

## 三、对 Hermes_Rust_Operit_App 的借鉴

| 功能 | 优先级 | 说明 |
|------|--------|------|
| 多层安全策略架构 | ★★★★★ | 当前 Hermes 无安全层 |
| Skill guard 模式检测 | ★★★★★ | 防止 prompt injection |
| 工具策略预设 | ★★★★ | strict/dev/relaxed 模式 |
| 脱敏引擎 | ★★★★ | 防止密钥泄露 |
| 检查点/回放 | ★★★ | 调试和恢复 |
| 观察者契约 | ★★★ | 可观测性 |

---

## 四、三到五个可复用点

1. **6 层安全架构** — 从 guard → 脱敏 → 网络隔离的完整设计
2. **PreToolCall hook** — 工具调用前拦截和审批
3. **客观守卫输出验证** — 结构化输出 + 重试
4. **检查点 + 回放** — 会话快照和全量录制
5. **Fail-open 设计** — 钩子异常不阻断 Agent
