# TabbyML — 学习报告

> 项目：https://github.com/TabbyML/tabby
> 关键发现：Rich Segments 补全模型、客户端沙盒模式
> 学习日期：2026-07-18

## 项目概况

33.7k stars，Rust workspace 16 crates，自托管 AI 编程助手。

## 架构

```
crates/tabby             → 主二进制（serve, download）
crates/tabby-common      → 配置、API 类型、语言定义
crates/tabby-inference   → 推理抽象层
crates/tabby-index       → 代码索引（tantivy + embedding）
crates/tabby-download    → 模型下载管理
crates/tabby-git         → Git 集成
crates/llama-cpp-server  → llama.cpp 后端
crates/ollama-api-bindings → Ollama 后端
```

## 补全 API 设计（Rich Segments 模型）

```rust
Segments {
    prefix: String,       // 光标前文本
    suffix: String,       // 光标后文本（FIM）
    declarations: [],     // LSP 声明
    relevant_snippets: [],// RAG 检索片段
    clipboard: String,    // 剪贴板
    git_url: String,      // 仓库 URL
    filepath: String,     // 文件路径
    edit_history: [],     // 编辑历史（NES 模式）
}
```

**关键模式**：不是传原始 prompt，而是传结构化 Segments，服务端构建 prompt。

## 沙盒模式

**Tabby 不做服务端沙盒**。执行委托给 IDE 客户端。
- 服务端：推理 + 检索
- 客户端（IDE 插件）：代码执行

**结论**：客户端沙盒比服务端沙盒更安全。

## 对 Hermes_Rust_Operit_App 的借鉴

1. **Rich Segments 模型** — 优于原始 prompt 传递
2. **客户端执行模式** — 最安全的沙盒
3. **代码搜索 trait** — tantivy + embedding 抽象
4. **SSE 流式** — axum + KeepAlive
5. **配置驱动模型切换** — 本地/远程透明
