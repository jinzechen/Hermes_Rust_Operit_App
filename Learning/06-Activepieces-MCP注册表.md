# 06 — Activepieces MCP 注册表 学习报告

> **项目**：https://github.com/activepieces/activepieces (23,312⭐, TypeScript)  
> **核心**：MCP 服务器注册表 + 自动化工作流平台  
> **Hermes 集成现状**：参照 MCP 注册表模式为 Operit_MCPS 设计

---

## 核心发现

Activepieces 的 MCP 注册表管理功能：
- 发现和安装 MCP 服务器
- 统一管理界面
- 工具列表浏览

## 对 Hermes_Rust_Operit_App 的作用

| 可复用点 | 说明 |
|----------|------|
| MCP 注册表模式 | Operit_MCPS 的 9 个插件需要类似的发现/安装机制 |
| MCP Server 生命周期 | 启动/停止/配置管理 |
| 统一工具列表 | 所有 MCP 插件暴露的工具统一展示 |

## 三到五个可复用点

1. **MCP 注册表架构** — Operit_MCPS 插件目录
2. **工具列表聚合** — 跨 MCP 插件的工具发现
3. **配置模板** — 每个插件独立的配置界面模式
