# Hermes_Rust_Operit_App — 会话进度交接文档

> 生成时间：2026-07-18  
> 当前模型：deepseek-v4-pro（已切换到 v4-flash 下次生效）  
> 目标：让 deepseek-v4-flash 无缝接手继续执行  

---

## 一、项目概述

**目标**：纯 Rust 重构 Operit Android AI 助手应用，整合 hermes-agent-rs 内核 + Operit 外壳理念。

**仓库**：https://github.com/jinzechen/Hermes_Rust_Operit_App  
**本地路径**：`D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App`

---

## 二、当前进度

### 2.1 已完成

| 阶段 | 内容 | 状态 |
|------|------|------|
| 项目骨架 | 23 个 Rust 文件，3661 行代码 | ✅ |
| 核心模块 | AgentManager, ToolRegistry, MemoryStore, Config, 3 LLM Providers | ✅ |
| 工具模块 | FileSystemTool(8 ops), MarkdownTool, BrowserTool(骨架), VisionTool(骨架) | ✅ |
| MCP 模块 | McpClient (stdio JSON-RPC 2.0 完整实现) | ✅ |
| 商店模块 | PluginStore (Skill/MCP 安装管理) | ✅ |
| 环境模块 | Alpine Linux + Sandbox (骨架) | ✅ |
| UI 数据层 | Chat, OAuth, Settings, StoreBrowser | ✅ |
| CI | GitHub Actions (cargo check 仍在修编译错误) | ⚠️ |
| 学习报告 | 17 份深度分析报告 | ✅ |

### 2.2 编译状态

CI 当前失败，最后已知错误（待修复）：
- CI 简化后只跑 `cargo check`，exit code 101
- 已知修过的问题：redb 生命周期、oauth2 API、anyhow import、FileSystemTool::new 参数
- 本地无法编译（Windows MSVC linker 被 MSYS link.exe 劫持）

### 2.3 还有 7 个未完成模块待补充

```
需要新建的模块：
  src/core/character.rs      ← 角色卡系统
  src/core/local_model.rs    ← 本地模型管理
  src/store/sources.rs       ← GitHub 源聚合
  src/ui/voice.rs            ← 语音模块
  src/core/sandbox.rs        ← 沙盒实现
```

---

## 三、关键文件速查

### 3.1 核心架构

```
src/lib.rs                    ← 模块导出 + re-export AgentManager/AppConfig
src/main.rs                   ← CLI 入口 (help/status/chat/tools/store/login)
src/core/mod.rs               ← 子模块声明
src/core/agent.rs             ← AgentManager (唯一对外接口，AgentLoop 实现)
src/core/tool_registry.rs     ← ToolHandler trait + ToolRegistry
src/core/memory.rs            ← MemoryStore (redb)
src/core/provider.rs          ← LlmProvider trait + Generic/Anthropic 实现
src/core/config.rs            ← AppConfig
src/tools/*.rs                ← 4 个工具（filesystem/markdown/browser/vision）
src/mcp/client.rs             ← McpClient (stdio JSON-RPC)
src/store/mod.rs              ← PluginStore
src/environment/mod.rs        ← Alpine Linux 环境
src/ui/*.rs                   ← Chat/OAuth/Settings/StoreBrowser 数据模型
```

### 3.2 依赖

```toml
tokio, serde, serde_json, serde_yaml
reqwest (json + blocking + rustls-tls)
oauth2 (reqwest feature)
redb, log, env_logger, tracing
anyhow, thiserror, async-trait
uuid, chrono, parking_lot, once_cell, directories
walkdir, flate2, regex
```

---

## 四、17 份学习报告索引

`D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App\Learning\`

| # | 文件 | 核心发现 | 整合优先级 |
|---|------|----------|------------|
| 01 | Understand-Anything | 知识图谱可视化 + 多语言解析器 | P3 |
| 02 | OpenHuman | **tinycortex crate 可直接加依赖** | **P0** |
| 03 | Hermes-Agent-Ultra | 6 层安全策略 + 确定性执行 | P1 |
| 04 | TabbyML | Rich Segments 补全模型 | P2 |
| 05 | Qdrant | Rust 原生向量搜索 | P2 |
| 06 | Activepieces | ~400 MCP 数据源 | P2 |
| 07 | rtk+cc-switch | Token 优化 + 供应商路由 | P1 |
| 08 | oxideterm | AI + Terminal 融合 | P3 |
| 09 | codex-mobile+iHermes | Android 移动端模式 | P3 |
| 10 | yazi+servo | 异步 FS Engine + 浏览器引擎 | P2 |
| 11 | openclaw | 60+ 供应商 Agent 架构 | P2 |
| 12 | nushell | 结构化 Shell 管道模式 | P1 |
| 13 | Operit-插件格式 | Skill/ToolPkg/MCP 规范 | P1 |
| 14 | Python-Hermes | 原版 Agent Loop (~5680行) | P1 |
| 15 | Fabric+AgentS+LLM | 255 patterns + 382 免费模型 | P2 |
| 16 | Skills-MCP-生态 | 5400+ skills + 中文 MCP | P2 |
| 17 | 深度代码库分析 | 完整类型系统 + 依赖图 + 整合路线图 | P0 |

---

## 五、下一步行动（按优先级）

### P0 — 立即执行

1. **修复 CI 编译错误**
   - 跑 `cargo check` 看最新错误
   - 已知问题：redb 生命周期、oauth2 API 不匹配
   - 目标：CI 绿色

2. **添加 tinycortex 依赖**
   - `cargo add tinycortex --features git-diff,persona`
   - 替换当前简陋的 MemoryStore
   - 实现 EmbeddingBackend/ChatProvider adapter

### P1 — 核心增强

3. **Agent Loop 增强**
   - 参考 Python Hermes 的 context compression
   - 添加 skill nudge 机制
   - 添加 background review

4. **安全层**
   - 参考 hermes-agent-ultra 的 skill guard (19 危险模式)
   - 添加 ToolHandler 中间件

5. **TokenOptimizer**
   - 参考 rtk 的 12 种过滤策略
   - AgentLoop 内置过滤中间件

6. **Skill 系统完善**
   - 按 Operit 格式实现 skill_manage
   - 实现 skill 扫描/加载/调用生命周期

### P2 — 功能扩展

7. **Qdrant 向量搜索集成**
8. **ProviderRouter (priority + circuit breaker)**
9. **Engine trait (yazi 可插拔 FS)**
10. **Plugin Store 多源聚合**

### P3 — 远期

11. **CodebaseAnalyzer (Understand-Anything)**
12. **Dioxus UI 层**
13. **Android APK 构建**

---

## 六、Understand-Anything 已安装状态

```
安装路径: D:\Hermes_Agent_Desktop\Hermes_Download\Understand-Anything\
Dashboard: http://127.0.0.1:8765/ (Python HTTP，可能已停)
知识图谱: 37 节点 / 46 边 (已生成)
扫描命令: node understand-anything-plugin/skills/understand/scan-project.mjs <项目路径> <输出json>
```

---

## 七、当前模型配置

```yaml
# D:\Hermes_Agent_Desktop\config.yaml
model: deepseek-v4-flash  # 已切换
provider: custom
```

---

## 八、给接手 AI 的指令

1. 阅读本文件了解全局
2. 阅读 `Learning/17-深度代码库分析.md` 了解代码现状
3. 阅读 `Learning/02-OpenHuman-记忆系统.md` 了解 tinycortex 整合方案
4. 先修 CI，再按 P0→P1→P2→P3 顺序推进
5. 不要重新学习已学过的项目，直接看 Learning/ 中的报告
6. 每次提交前确认 `cargo check` 通过
7. 用户偏好：中文回复、穷尽式分析、不弹确认窗

---

*本文件由 deepseek-v4-pro 在会话结束时自动生成*
