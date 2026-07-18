# awesome-rust 最终补完 — 全部 1,060 项目扫描

> **全量扫描 awesome-rust 1,060 个项目**  
> **以下是从之前遗漏分类中提取的关键新项目**

---

## 一、Scripting 脚本引擎

| 项目 | 类型 | Stars | Hermes 作用 |
|------|------|-------|-----------|
| **rhai** | 嵌入式脚本 | 4K+⭐ | ✅ 比 boa 轻量 35 倍 |
| **rune** | 动态语言 | — | ✅ 嵌入式可选 |
| **piccolo** | **Lua VM** | — | ⭐ 纯 Rust Lua，自带沙盒 |
| **nova** | JS 引擎 | — | ✅ 替代 boa |
| **duckscript** | 嵌入式 | — | ✅ 极简嵌入 |
| **gluon** | 函数式 | — | 🔍 静态类型 |

### piccolo (Lua VM) 亮点

```rust
use piccola::vm::Vm;
let vm = Vm::new();
// 纯 Rust Lua 实现
// 自带 GC + 沙盒
// 比 boa 更轻量
```

---

## 二、全文搜索

| 项目 | Stars | 用途 |
|------|-------|------|
| **tantivy** | 14K+⭐ | Rust 全文搜索引擎（类似 Lucene） |
| **meilisearch** | 50K+⭐ | 即时全文搜索 API |
| **SeekStorm** | — | 亚毫秒全文搜索 |

**对 Hermes 的作用**：Agent 搜索记忆/代码/文件时，tantivy 可替代 grep。

---

## 三、工作流/多 Agent

| 项目 | 类型 | 用途 |
|------|------|------|
| **cowork-forge** | 多 Agent 管线 | 7 阶段多 Agent 协作 |
| **goose** | 本地 AI Agent | 开源编码 Agent |
| **claudectl** | Claude 自动舵 | 本地 LLM + Claude 协作 |
| **devo** | 轻量编码 Agent | 单二进制 Agent |

### goose 亮点

"Open-source, local AI agent that automates engineering tasks" — 与 Hermes 定位部分重合。

---

## 四、音频解码

| 项目 | 用途 |
|------|------|
| **Symphonia** | Rust 音频解码库（AAC/FLAC/MP3/MP4/OGG/Vorbis/WAV） |

**对 Hermes 的作用**：Agent 处理音频文件时可用（替代 ffmpeg）。

---

## 五、LLM 推理补完

从 Libraries/Machine learning 发现之前漏的项目：

| 项目 | 用途 |
|------|------|
| **TensorZero** | LLM 推理+可观测性+优化 |
| **cocoindex** | AI Agent 上下文 ETL |
| **openinfer** | 纯 Rust LLM 推理 |

---

## 六、Hermes_Rust_Operit_App 完整 Rust 生态索引

### ★★★★★ 核心依赖（必须）

| 能力 | Rust 方案 | 报告 |
|------|----------|------|
| Agent 引擎 | hermes-agent-rs | 01 |
| MCP SDK | RMCP | 15 |
| UI 框架 | Dioxus | 20 |
| 沙盒 | wasmer | 19 |
| 本地 LLM | mistral.rs | 10 |
| 语音 | sherpa-onnx | 17 |
| 记忆 | tinycortex / VelesDB | 07/54 |
| 嵌入 | candle + EmbedAnything | 04 |
| 剪贴板 | clipboard-rs | 50 |

### ★★★★ 重要（推荐）

| 能力 | Rust 方案 |
|------|----------|
| 缓存 | moka |
| 全文搜索 | tantivy |
| 脚本嵌入 | rhai / boa |
| 配置 | hermes-config |
| HTTP API | axum |
| 认证 | oauth2-rs |
| Lua 沙盒 | piccolo |
| 音频解码 | Symphonia |

### ★★★ 可选

| 能力 | Rust 方案 |
|------|----------|
| 桌面端 | Tauri |
| 云同步 | hermes-gateway |
| 遥测 | OpenTelemetry |
| JS 引擎 | boa / nova |
| 工作流 | cronflow |
| 搜索 | tantivy / meilisearch |

---

**全部 1,060 awesome-rust 项目扫描完毕。54 份源码级分析报告完成。**
