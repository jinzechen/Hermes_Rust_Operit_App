# 05 — Operit_MCPS：9 个 MCP 插件源码分析

> **仓库**：https://github.com/jinzechen/Operit_MCPS (用户自建, Rust)  
> **UA Rust 分析数据**：1,505 nodes / 1,556 edges / 5 layers / 659 Rust 文件  
> **Hermes_Rust_Operit_App 评分**：★★★★★（直接使用，按需决定内置/MCP）

---

## 一、9 个插件详解

| # | 插件 | Rust 重写 | 功能 | Hermes 对应 | 决策 |
|---|------|----------|------|-----------|------|
| 1 | **obscura** | ❌ 预编译 binary | 无头浏览器 | `browser.rs` | ✅ 内置 |
| 2 | **agentic_vision** | ✅ 自写 | 视觉分析/OCR | `vision.rs` | ✅ 内置 |
| 3 | **rust_mcp_filesystem** | ✅ 自写 | 25+ 文件操作 | `filesystem.rs` | ✅ 内置 |
| 4 | **typemill** | ✅ 自写 | Markdown 处理 | `markdown.rs` | ✅ 内置 |
| 5 | **sherpa** | ✅ 自写 | 语音 ASR+TTS | 新功能 | ✅ 内置 |
| 6 | **m3ux** | ✅ 自写 | 音频处理 | 新功能 | ⚠️ 保留 MCP |
| 7 | **rust_mcp_server** | ✅ 自写 | Rust 开发工具 | 新功能 | ⚠️ 保留 MCP |
| 8 | **mcp_proxy** | ✅ 自写 | MCP 代理 | 新功能 | ⚠️ 保留 MCP |
| 9 | **rust_docs_mcp** | ✅ 自写 | Rust 文档查询 | 新功能 | ⚠️ 保留 MCP |

### 内置化原则（来自 hermes-agent-rs 的集成铁律）

```
ToolHandler (0 开销) > MCP Client (15ms) > Skills (50ms)
```

高频 + 低延迟 → 内置 ToolHandler（browser/vision/filesystem/markdown/voice）
低频 + 重量 → 保留 MCP（rust tools/docs/audio/proxy）

---

## 二、ZIP 打包规范（build 目录分析）

每个插件的发布产物：

```
plugin-v1.0.0.zip
├── binary                → aarch64-linux-musl 静态链接
├── index.js              → auto-chmod 755 + spawn(bin, args)
└── package.json          → name / version / description / tools
```

### index.js 模板

```javascript
#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const bin = path.join(__dirname, 'binary');
// Android 上 chmod 755
try { fs.chmodSync(bin, 0o755); } catch(e) {}

const proc = spawn(bin, process.argv.slice(2), {
    stdio: ['pipe', 'pipe', 'pipe'],
    env: { ...process.env }
});

process.stdin.pipe(proc.stdin);
proc.stdout.pipe(process.stdout);
proc.stderr.pipe(process.stderr);
proc.on('exit', (code) => process.exit(code));
```

---

## 三、对 Hermes_Rust_Operit_App 的作用

### 直接可用的 MCP 插件（保留）

通过 `mcp/client.rs` 直接连接：

```rust
// 使用 MCP 模式调用 obscura
let client = McpClient::connect_stdio("obscura", &["mcp"])
let tools = client.list_tools().await?;
let result = client.call_tool("browser_navigate", json!({"url": "..."})).await?;
```

### 可内置的 ToolHandler（性能更优）

```rust
// 使用 ToolHandler 模式（0 开销）
let handler = BrowserNavigateHandler::new(backend);
let result = handler.handle(tool_call).await?;
```

### 评分：★★★★★

Operit_MCPS 是连接 Hermes_Rust_Operit_App 和 Operit Android 壳的桥梁。5 个插件可内置化提升性能，4 个保留 MCP 提供扩展性。
