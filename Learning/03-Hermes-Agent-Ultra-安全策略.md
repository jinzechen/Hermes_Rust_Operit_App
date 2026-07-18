# Hermes Agent Ultra — 学习报告

> 项目：https://github.com/sheawinkler/hermes-agent-ultra
> 关键发现：多层安全策略、确定性执行、观察者钩子
> 学习日期：2026-07-18

## 项目概况

22 crate workspace，1339 commits，完全 Rust 实现。

## 安全策略架构（6 层）

```
Layer 1: Skill 内容扫描 (hermes-skills/src/guard.rs)
         - 19 种危险模式检测
         - 9 种 prompt injection 检测
         - URL/IP 黑名单

Layer 2: 工具策略控制 (hermes-tool-planning)
         - strict/balanced/dev/relaxed 预设
         - 平台级工具集解析

Layer 3: 插件中间件
         - PreToolCall hook（可 block）
         - ToolRequestMiddleware（修改参数）
         - ToolExecutionMiddleware（拦截结果）

Layer 4: 敏感数据脱敏 (hermes-intelligence/redact.rs)
         - 30+ 已知密钥前缀
         - PII 检测（邮箱/电话/信用卡）

Layer 5: 网络出口隔离
         - Docker 级防御
         - Squid 代理白名单

Layer 6: 预算/调速器控制
         - max_turns, 成本追踪
         - 可靠性降级 + 模型切换
```

## 确定性执行

- **检查点**：每 N 轮保存会话快照
- **回放**：ReplayRecorder 记录完整会话
- **客观守卫**：结构化输出验证 + 重试
- **工具去重 + 修复**：大小写不敏感匹配、参数修复

## 观察者钩子契约

- 会话生命周期、每轮范围、API 请求、工具生命周期
- 审批生命周期、子代理生命周期
- 关联 ID（session_id, task_id, turn_id, api_request_id, tool_call_id）
- **Fail-open**：钩子异常不阻断 Agent
- 负载脱敏后导出

## 对 Hermes_Rust_Operit_App 的借鉴

| 功能 | 优先级 |
|------|--------|
| 多层安全策略架构 | 高 |
| Skill guard 模式检测 | 高 |
| 工具策略预设 | 高 |
| 客观守卫输出验证 | 高 |
| 脱敏引擎 | 中 |
| 检查点/回放 | 中 |
| 观察者契约 | 中 |
