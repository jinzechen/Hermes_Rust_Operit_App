# 07 — obscura：无头浏览器源码分析

> **仓库**：https://github.com/h4ckf0r0day/obscura (19,392⭐, Rust)  
> **核心**：AI Agent 用无头浏览器（替代 Selenium/Puppeteer）  
> **Hermes_Rust_Operit_App 评分**：★★★★★

---

## 一、架构

```
obscura (Rust, 19K⭐)
├── src/
│   ├── browser/    → Chrome DevTools Protocol (CDP) 控制
│   ├── mcp/        → MCP 协议接口（obscura mcp）
│   └── api/        → HTTP REST API
```

## 二、对 Hermes 的作用

| 能力 | hermes-tools browser.rs | obscura 增强 |
|------|-----------------------|-------------|
| 页面导航 | ✅ | ✅ |
| 截图 | ❌ | ✅ CDP Screenshot |
| DOM 提取 | ❌ | ✅ CDP DOM |
| JS 执行 | ❌ | ✅ CDP Runtime.evaluate |
| 网络拦截 | ❌ | ✅ CDP Network |
| MCP 模式 | ❌ | ✅ obscura mcp |

## 三、集成

已在 Operit_MCPS 中，通过 MCP 调用：

```bash
obscura mcp  # 启动 MCP 服务器模式
```

## 评分：★★★★★

obscura 已打包为 Operit_MCPS 插件，可直接通过 MCP 调用。如果需要更高性能，可以将其 browser 能力内化为 ToolHandler。
