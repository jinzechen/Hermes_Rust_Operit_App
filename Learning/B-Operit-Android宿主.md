# Operit — Android MCP 宿主平台深度分析（三大源项目之二）

> **项目**：https://github.com/AAswordman/Operit (5,864⭐, Kotlin)  
> **定位**：**Hermes_Rust_Operit_App 的 Android 宿主平台**  
> **架构**：Kotlin + Compose + MNN/llama.cpp + MCP/Skill 插件生态

---

## 一、项目概况

Operit 是 Android 上功能最完备的 AI Agent 应用，核心客户端和模型配置均运行在设备本地。

### 核心亮点

| 能力 | 说明 | Hermes_Rust 对应 |
|------|------|-----------------|
| **Ubuntu 24 环境** | 内置完整 Linux 系统 | ❌ 需要 Termux |
| **MCP/Skill 市场** | 插件化的工具生态 | ✅ Operit_MCPS 就是为它做的 |
| **本地模型** | MNN + llama.cpp GGUF | ❌ 需集成 |
| **40+ 内置工具** | 文件/网络/系统/媒体 | ✅ Hermes 有 5 个 |
| **智能记忆** | 自动分类+时序查询 | ✅ 需增强 |
| **语音交互** | TTS + STT | ❌ 可加 sherpa-rs |
| **人设/角色卡** | 角色扮演+独立对话 | ❌ 未来 |
| **自动点击 Agent** | 屏幕操作 | ❌ 未来 |

---

## 二、技术架构

```
Operit (Kotlin, Android Compose)
│
├── app/                → Android 主模块
│   ├── build.gradle.kts (15.6KB)
│   ├── src/main/       → Kotlin 源码
│   └── ...
│
├── llama/              → llama.cpp 本地推理
├── mnn/                → MNN 本地推理
├── quickjs/            → JavaScript 引擎
├── terminal/           → 终端模拟器
│
├── web-chat/           → Web 聊天界面
├── showerclient/       → 投屏客户端
│
├── tools/              → 工具定义
├── docs/               → 文档
│
└── MCP/Skill 市场      → 插件生态 (→ Operit_MCPS)
```

---

## 三、Operit_MCPS 插件集成

Hermes_Rust_Operit_App 的 Operit_MCPS 项目（用户自建）为 Operit 提供 9 个 MCP 插件：

| 插件 | Hermes 也有？ |
|------|-------------|
| **obscura** (浏览器) | ✅ `browser.rs` |
| **agentic_vision** (视觉) | ✅ `vision.rs` |
| **rust_mcp_filesystem** (文件) | ✅ `filesystem.rs` |
| **typemill** (Markdown) | ✅ `markdown.rs` |
| **rust_mcp_server** (Rust 工具) | ❌ |
| **mcp_proxy** (代理) | ❌ |
| **sherpa** (语音) | ❌ |
| **m3ux** (音频) | ❌ |
| **rust_docs_mcp** (文档) | ❌ |

---

## 四、对 Hermes_Rust_Operit_App 的启发

| 维度 | Operit 做法 | Hermes 可以借鉴 |
|------|-----------|----------------|
| 工具调用 | 40+ 内置 + MCP/Skill | 从 5 个扩展到 40+ |
| 本地模型 | MNN + llama.cpp | 集成 candle 做嵌入 |
| 记忆系统 | 自动分类+时序 | tinycortex 方案 |
| Ubuntu 环境 | 内置 Termux | Termux 通道 |
| 语音 | TTS + STT | sherpa-rs 集成 |
| 角色卡 | 自定义性格 | 未来 |

---

## 五、三到五个关键复用点

| # | 复用点 | 说明 |
|---|--------|------|
| 1 | **Operit_MCPS 插件** | 9 个 MCP 插件是 Hermes 和 Operit 的桥梁 |
| 2 | **工具生态扩展** | 从 5 个工具扩展到 40+ 的路线图 |
| 3 | **本地模型** | MNN/llama.cpp 的 Android 部署经验 |
| 4 | **Ubuntu 环境** | Termux 作为 Agent 执行环境 |
