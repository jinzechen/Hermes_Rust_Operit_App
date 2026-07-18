# 05 — Operit_MCPS：9 个 MCP 插件源码级分析

> **仓库**：https://github.com/jinzechen/Operit_MCPS (用户自建, Rust)  
> **UA Rust 分析**：1229 文件 / 659 Rust 文件 / **1505 节点** / **1556 边** / 5 层架构  
> **Hermes_Rust_Operit_App 评分**：★★★★★

---

## 一、UA Rust 分析发现

| 指标 | 值 |
|------|----|
| 总文件数 | 1,229（14 种语言） |
| Rust 源文件 | **659**（最大代码库） |
| 解析的文件 | **624** |
| 知识图谱节点 | **1,505** |
| 关系连线 | **1,556** |
| 架构层数 | **5** |
| 引导步数 | **8** |
| 复杂度 | Complex |

这是 Hermes_Rust_Operit_App 生态中**规模最大的代码库**，甚至超过 hermes-agent-rs（352 Rust 文件）。

---

## 二、9 个插件 + 构建系统

```
Operit_MCPS/
├── build.sh             → 构建脚本（cross + Docker）
├── build/
│   ├── src-obscura/        → 无头浏览器 ★
│   ├── src-agentic_vision/ → 视觉分析 ★
│   └── src-rust-mcp-*/     → Rust MCP 工具
├── .github/workflows/
│   └── build-mcp-plugins.yml → CI 自动构建
└── 发布 ZIP 包
    ├── binary            → aarch64-linux-musl 静态链接
    ├── index.js          → auto-chmod + spawn
    └── package.json      → 元数据
```

### 构建流程

```bash
# build.sh
for plugin in obscura agentic_vision rust_mcp_server mcp_proxy \
              rust_mcp_filesystem rust_docs_mcp typemill; do
    cross build --target aarch64-unknown-linux-musl --release
    # → 产物: target/aarch64-linux-musl/release/<binary>
    # → 打包: binary + index.js + package.json → ZIP
done
```

---

## 三、插件内置化评估

| 插件 | Rust 文件数 | 功能 | 决策 | 理由 |
|------|-----------|------|------|------|
| obscura | 预编译 | 无头浏览器 | ✅ 内置 | hermes-tools browser.rs |
| agentic_vision | 大量 | 视觉/OCR | ✅ 内置 | hermes-tools vision.rs |
| rust_mcp_filesystem | 大量 | 25+ 文件操作 | ✅ 内置 | hermes-tools file.rs |
| typemill | Rust 自写 | Markdown | ✅ 内置 | hermes-tools markdown |
| sherpa | Rust 自写 | 语音 | ✅ 内置 | sherpa-onnx 直接调用 |
| rust_mcp_server | 大量 | Rust 工具链 | ⚠️ MCP | 太重，按需 |
| mcp_proxy | 少量 | MCP 代理 | ⚠️ MCP | 外部连接 |
| m3ux | 少量 | 音频 | ⚠️ MCP | 低频 |
| rust_docs_mcp | 少量 | 文档查询 | ⚠️ MCP | 按需 |

---

### Rust 复刻总结

5 个可内置化到 ToolHandler（零开销）的插件直接移到 hermes-tools：

```rust
// 内置插件路径
src/tools/browser.rs       ← 从 obscura MCP 提取核心逻辑
src/tools/vision.rs        ← 从 agentic_vision MCP 提取
src/tools/filesystem.rs    ← 从 rust_mcp_filesystem 提取
src/tools/markdown.rs      ← 从 typemill 提取
src/tools/speech.rs        ← 从 sherpa MCP 提取
```

4 个保留 MCP 的插件通过 hermes-mcp 客户端调用：

```rust
// MCP 插件路径
let client = McpClient::connect_stdio("rust-mcp-server", &[]);
let tools = client.list_tools().await?;
```

### 评分：★★★★★

Operit_MCPS 是 Hermes_Rust_Operit_App 和 Operit Android 壳之间的桥梁。9 个插件中 5 个可内置化提升性能，4 个保留 MCP 提供扩展性。
