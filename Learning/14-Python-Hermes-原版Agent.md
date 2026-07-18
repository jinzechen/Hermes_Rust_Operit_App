# Python Hermes Agent 原版 — 深度分析
> 项目：https://github.com/NousResearch/hermes-agent | Python | 216k stars
> 学习日期：2026-07-18

## Agent Loop 架构
- `run_conversation()` (~5680 行): 单次用户消息→完整工具调用循环
- 流程: build_turn_context → while(iterations < max): API call → tool dispatch → loop → finalize_turn
- 多 Provider 自动降级链
- Context 压缩: pre-flight + post-response
- 后台审查: post-turn fork agent 评估保存 skills/memories

## Skills 系统
- SKILL.md (YAML frontmatter + Markdown body)
- 按需加载: `/skill-name` → 展开为 model-facing message
- Self-improvement: Agent 用 `/learn` 或 `skill_manage` 创建/修补 skills
- Background review: 每轮后自动评估
- Skill Bundles: YAML 批量加载

## Memory 系统
- MemoryProvider ABC: prefetch/sync/shutdown
- 内置: MEMORY.md + USER.md
- 外部: mem0, openviking, hindsight, retaindb 等 8 种
- 每轮注入: prefetch → `<memory-context>` 标签 → API 调用

## Tool 系统
- ~100+ 工具文件
- 自注册模式: 每个模块 import 时调用 registry.register()
- ToolEntry: name, toolset, schema, handler, check_fn, is_async, emoji
- 并发执行: ThreadPoolExecutor (max 8, 420s timeout)

## Python vs Rust 差异
| 方面 | Python | Rust 移植 |
|------|--------|-----------|
| Skills | 深度集成 memory + background review | 子集 |
| Tools | ~100+ 自注册 | MCP-based |
| Context | 复杂压缩管道 | 简化 |
| Plugins | 丰富钩子系统 | 基本 |
| Memory | 8 种外部 provider | 少 |
