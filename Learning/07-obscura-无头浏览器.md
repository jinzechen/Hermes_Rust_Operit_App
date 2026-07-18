# 07 — obscura：无头浏览器源码分析

> **仓库**：https://github.com/h4ckf0r0day/obscura (19,392⭐, Rust)  
> **核心能力**：AI Agent 用的无头浏览器  
> **Hermes_Rust_Operit_App 评分**：★★★★★（替代 Selenium/Puppeteer）

---

## 一、源码结构

```
obscura (Rust, 19K⭐)
├── src/
│   ├── browser/    → Chromium 控制
│   ├── mcp/        → MCP 协议接口
│   ├── api/        → HTTP REST API
│   └── main.rs
```

## 二、核心能力

| 能力 | 实现 | 对应 Hermes tool |
|------|------|-----------------|
| 页面导航 | Chrome DevTools Protocol | `browser.rs` navigate |
| 截图 | CDP Screenshot | snapshot |
| DOM 提取 | CDP DOM | 待实现 |
| JS 执行 | CDP Runtime.evaluate | 待实现 |
| 网络拦截 | CDP Network | 待实现 |
| MCP 模式 | `obscura mcp` 子命令 | `mcp/client.rs` |

## 三、已内置在 Operit_MCPS

```bash
# 通过 MCP 调用
obscura mcp  # 启动 MCP 服务器模式
```

## 四、对 Hermes_Rust_Operit_App 的作用

Hermes 已有 `browser.rs`，但 obscura 提供更完整的浏览器自动化能力（截图、JS 执行、网络拦截）。

### 评分：★★★★★

obscura 是 Hermes 浏览器能力的完美补充。已作为 Operit_MCPS 插件可用，可直接通过 MCP 调用。
