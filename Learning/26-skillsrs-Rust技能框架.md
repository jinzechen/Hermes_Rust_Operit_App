# 26 — Skills-rs / SkillsRS 学习报告

> **搜索范围**：GitHub + crates.io  
> **Hermes 集成现状**：✅ Hermes 已有 `tool_registry.rs` + Operit_MCPS 插件系统

---

## 一、搜索结果

GitHub 和 crates.io 上未找到独立的 "skillsrs" 或 "skills-rs" Rust crate。

## 二、Hermes 的技能系统

Hermes_Rust_Operit_App 的技能系统由两部分组成：

### 1. ToolHandler（0 开销，内置）

```
tools/codebase_analyzer.rs  — UA Rust 知识图谱
tools/browser.rs            — 浏览器自动化
tools/vision.rs             — 视觉分析
tools/filesystem.rs         — 文件系统
tools/markdown.rs           — Markdown 处理
```

### 2. MCP 插件（15ms 开销，外置）

Operit_MCPS 的 9 个 MCP 插件通过 `mcp/client.rs` 通信。

## 三、对比 Understand-Anything 的 Skill 格式

```yaml
# SKILL.md 格式（Operit 兼容）
---
name: analyze-codebase
description: Analyze a codebase using UA Rust
argument-hint: ["[path]"]
---
```

## 四、三到五个可复用点

1. **ToolHandler > MCP** — 内置工具性能优于外部插件
2. **SKILL.md 格式** — 标准化元数据描述
3. **分层技能系统** — 内置 (0开销) + MCP (15ms) + Skills (50ms)
