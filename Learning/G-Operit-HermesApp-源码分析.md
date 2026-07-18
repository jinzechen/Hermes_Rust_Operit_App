# Operit + HermesApp — 源码级深度分析与 Rust 复刻方案

> **Operit 仓库**：https://github.com/AAswordman/Operit (5.8K⭐, Kotlin)  
> **HermesApp 仓库**：https://github.com/SelectXn00b/HermesApp (194⭐, Kotlin)  
> **核心发现**：HermesApp = "Hermes Kotlin 内核 + Operit Android 壳"

---

## 一、Operit Android 壳完整分析

### 模块清单（从 settings.gradle.kts 提取）

| 模块 | 用途 | Rust 复刻方案 |
|------|------|--------------|
| `:app` | 主应用（UI + 工具 + 权限） | Dioxus UI |
| `:terminal` | 终端模拟器 | Termux JNI |
| `:mnn` | MNN 本地推理引擎 | candle / ort |
| `:llama` | llama.cpp 本地推理 | llama-cpp-rs |
| `:showerclient` | 投屏客户端 | 预留 |
| `:quickjs` | JS 引擎 | boa_engine (Rust) |
| `:dragonbones` | 骨骼动画 | ❌ 非必需 |
| `:mmd` | MMD 3D 模型 | ❌ 非必需 |
| `:fbx` | FBX 3D 模型 | ❌ 非必需 |

### Operit 40+ 内置工具的 Rust 替代

| 工具类别 | Operit 实现 | Rust 替代 |
|---------|------------|----------|
| 文件操作 | Kotlin | hermes-tools file.rs |
| 网络请求 | Kotlin | hermes-tools web.rs |
| 浏览器 | Kotlin WebView | obscura MCP |
| 截图 | Kotlin + MediaProjection | Android JNI |
| 代码执行 | Ubuntu 24 Termux | code_execution.rs |
| 语音 TTS | Android TTS | hermes-tools tts.rs |
| 语音 STT | Android Speech | sherpa-onnx |
| 图片分析 | 本地模型 | hermes-tools vision.rs |
| 搜索 | 网页 API | hermes-tools web.rs |
| 定时任务 | Kotlin AlarmManager | hermes-cron |
| 剪贴板 | Kotlin Clipboard | Android JNI |
| 通知 | Kotlin Notification | Android JNI |
| 无障碍 | Kotlin AccessibilityService | Android JNI |
| 角色卡 | Kotlin 本地存储 | Dioxus UI + redb |
| 记忆系统 | Kotlin SQLite | hermes-agent memory |

---

## 二、HermesApp 的 Kotlin Hermes 内核

HermesApp 的核心是：**把 NousResearch/hermes-agent (Python) 的 Agent Loop 1:1 翻译成 Kotlin**。这意味着：

### Hermes Kotlin 内核的结构（推测）

```
hermes-android/src/
├── AgentLoop.kt        — Agent 主循环 (Hermes Python 翻译)
├── ProviderManager.kt  — LLM 提供者管理
├── ToolRegistry.kt     — 工具注册
├── MemoryManager.kt    — 记忆管理
├── SkillManager.kt     — 技能管理
└── McpClient.kt        — MCP 协议客户端
```

### Hermes_Rust 的对比优势

| 维度 | HermesApp (Kotlin) | Hermes_Rust_Operit_App (Rust) |
|------|-------------------|------------------------------|
| 内核来源 | 手工翻译 Python → Kotlin | ✅ 直接引入 hermes-agent-rs |
| 代码量 | 需维护完整翻译 | ✅ 零翻译成本 |
| 性能 | JVM 运行时 | ✅ 原生二进制 |
| 工具生态 | 需自建 | ✅ 35 个现成工具 |
| MCP | 需自建 | ✅ hermes-mcp (3.4K) |
| 记忆 | 需自建 | ✅ 8 种记忆插件 |
| 子 Agent | 需自建 | ✅ sub_agent_orchestrator |

**结论**：Rust 版本不需要像 HermesApp 那样从零翻译 Python 代码，因为 **hermes-agent-rs 已经完成了这个工作**。

---

## 三、三大项目源码级关系图

```
NousResearch/hermes-agent (Python, 216K⭐)
    │ Python Agent Loop
    ├──────────────────────────────┐
    │                              │
    ▼                              ▼
hermes-agent-rs (Rust)         HermesApp (Kotlin)
    │ Rust 翻译 Python              │ Kotlin 翻译 Python
    │ 18 crates / 352 文件          │ 1 app + hermes-android 模块
    │ agent_loop.rs (7K 行)        │ AgentLoop.kt
    │                              │
    └──────────────┬───────────────┘
                   │
                   ▼
         Hermes_Rust_Operit_App (Rust)
              │ 直接引入 hermes-agent-rs crate
              │ Dioxus UI (取代 Operit Kotlin UI)
              │ MCP 插件 (Operit_MCPS)
              │ 四 Tab 商店 (开源)
```

---

## 四、整合后的 Rust 代码结构

```
hermes-rust-app/
├── Cargo.toml
│   └── dependencies:
│       ├── hermes-agent      → Rust Hermes 内核
│       ├── hermes-tools      → 35 个工具
│       ├── hermes-mcp        → MCP 协议
│       ├── dioxus            → UI 框架
│       ├── oauth2            → GitHub 登录
│       └── sherpa-onnx       → 语音
│
├── src/
│   ├── main.rs               → 入口
│   ├── ui/
│   │   ├── chat.rs           → 对话界面
│   │   ├── store.rs          → 四 Tab 商店
│   │   ├── settings.rs       → 设置
│   │   └── auth.rs           → GitHub 登录
│   ├── android/
│   │   ├── jni_bridge.rs     → JNI 桥接
│   │   ├── accessibility.rs  → 无障碍服务
│   │   └── termux.rs         → Termux 通道
│   └── plugins/
│       ├── manager.rs        → 插件管理器
│       └── sandbox.rs        → 沙盒引擎
```

**结论**：不需要大量编码，核心工作只是把 hermes-agent-rs 的 crate 作为依赖引入，然后写 Dioxus UI 和 Android JNI 桥接。
