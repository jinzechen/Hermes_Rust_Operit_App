# rtk Token 优化 + cc-switch 多供应商路由 — 学习报告
> rtk: https://github.com/rtk-ai/rtk | 71.6k stars
> cc-switch: https://github.com/farion1231/cc-switch | Rust
> 学习日期：2026-07-18

## rtk: Token 优化 (60-90% 节省)

**12 种过滤策略**:
- Stats Extraction (90-99%): git status/log/diff
- Failure Focus (94-99%): 测试只显示失败
- Grouping by Pattern (80-90%): lint 错误分组
- Deduplication (70-85%): 重复日志行合并
- Code Filtering (0-90%): 8 语言注释/代码分离

**整合方案**: AgentLoop 内置 TokenOptimizer 中间件
- 过滤所有 tool output 再发给 LLM
- 预估节省：~78% token/会话

## cc-switch: 多供应商路由

**核心模式**:
- Priority-queue failover (P1→P2→P3)
- Circuit breaker per provider (Closed/Open/HalfOpen)
- Hot-switch 事件通知

**整合方案**: AgentLoop 内置 ProviderRouter
- 优先级队列 + 断路器
- 每个 provider 独立状态追踪
