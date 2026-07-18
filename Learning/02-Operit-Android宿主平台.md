# 02 — Operit：Android MCP 宿主平台源码分析

> **仓库**：https://github.com/AAswordman/Operit (5.8K⭐, Kotlin)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（Android 壳的目标平台）

---

## 一、模块架构（从 settings.gradle.kts 提取）

```
Operit/
├── :app/             → 主应用（UI + 工具 + 权限）
├── :terminal/        → 终端模拟器
├── :mnn/             → MNN 本地推理引擎
├── :llama/           → llama.cpp 本地 GGUF 推理
├── :showerclient/    → 投屏客户端
├── :quickjs/         → JavaScript 引擎
├── :dragonbones/     → 骨骼动画（角色 Avatar）
├── :mmd/             → MMD 3D 模型
└── :fbx/             → FBX 3D 模型
```

---

## 二、核心功能模块源码级分析

### 2.1 主应用模块（:app）

通过 HermesApp 的 Kotlin 源码（200+ 文件）可以反推出 Operit 的架构：

```
com.ai.assistance.operit/
├── api/chat/
│   ├── EnhancedAIService.kt      — 核心 AI 服务（Agent Loop）
│   ├── ChatRuntimeHolder.kt      — 运行时持有者
│   ├── AIForegroundService.kt    — 前台通知服务
│   │
│   ├── enhance/
│   │   ├── ToolExecutionManager.kt  — ★工具执行引擎
│   │   ├── ConversationService.kt   — 对话服务
│   │   ├── FileBindingService.kt    — 文件绑定
│   │   ├── MultiServiceManager.kt   — 多服务编排
│   │   ├── InputProcessor.kt        — 输入处理
│   │   └── ReferenceManager.kt      — 引用管理
│   │
│   ├── llmprovider/
│   │   ├── AIServiceFactory.kt      — ★Provider 工厂
│   │   ├── OpenAIProvider.kt        — OpenAI
│   │   ├── ClaudeProvider.kt        — Claude
│   │   ├── DeepseekProvider.kt      — DeepSeek
│   │   ├── GeminiProvider.kt        — Gemini
│   │   ├── OllamaProvider.kt        — Ollama 本地
│   │   ├── KimiProvider.kt          — Kimi
│   │   ├── DoubaoAIProvider.kt      — 豆包
│   │   ├── MNNProvider.kt           — MNN 本地
│   │   ├── LlamaProvider.kt         — llama.cpp 本地
│   │   └── ... 共 13 个 Provider
│   │
│   └── library/
│       └── MemoryLibrary.kt         — ★记忆库
│
├── core/
│   ├── tools/
│   │   └── defaultTool/standard/
│   │       └── StandardBrowserSessionToolsBrowserSemantics.kt — 浏览器自动化
│   └── workflow/
│       └── WorkflowExecutor.kt      — ★工作流执行器
│
└── util/
    ├── PathMapper.kt                — 路径映射
    ├── CrashRecoveryState.kt        — 崩溃恢复
    └── stream/                      — 流式处理
```

### 2.2 EnhancedAIService.kt（核心 AI 服务）

这是 Operit/HermesApp 的 `agent_loop.rs` 等效实现，用 Kotlin 实现了：

```
输入处理 → Provider 路由 → LLM 调用
    → 工具执行 → 结果注入 → 下一轮
    → 记忆存储 → 状态持久化
```

### 2.3 ToolExecutionManager.kt（工具执行引擎）

对应 hermes-agent-rs 的 `ToolRegistry + dispatch.rs`，实现：

```kotlin
class ToolExecutionManager {
    fun executeTool(name: String, args: JsonObject): JsonObject
    fun listTools(): List<ToolSchema>
    // 浏览器 / 文件 / 代码执行 / 搜索 ...
}
```

### 2.4 MemoryLibrary.kt（记忆库）

对应 hermes-agent-rs 的 `memory_manager.rs`，但仅 1 种实现。

---

## 三、Operit 特色功能（HermesApp 中没有的）

| 功能 | 模块 | Rust 复刻方案 |
|------|------|--------------|
| **Ubuntu 24 环境** | Termux | Termux JNI 通道 |
| **本地 MNN 推理** | `:mnn` | candle (Rust) |
| **本地 llama.cpp** | `:llama` | llama-cpp-rs |
| **JS 引擎** | `:quickjs` | boa_engine (Rust) |
| **投屏** | `:showerclient` | 预留 |
| **角色 Avatar** | `:dragonbones` | Dioxus 动画替代 |
| **角色卡** | app 内 | Dioxus UI + redb |

---

## 四、对 Hermes_Rust_Operit_App 的整合

### 直接可用的能力

| Operit 功能 | Rust 替代 | 工作量 |
|------------|----------|--------|
| 13 个 LLM Provider | hermes-agent-rs provider.rs | 零 |
| 40+ 内置工具 | hermes-tools 35 个 | 零 |
| 记忆系统 | hermes-agent memory_manager | 零 |
| 浏览器 | hermes-tools browser.rs + obscura | 零 |
| 文件系统 | hermes-tools file.rs | 零 |
| 终端 | hermes-tools terminal.rs | 零 |
| 语音 TTS | hermes-tools tts.rs | 零 |
| 定时任务 | hermes-cron | 零 |

### 需要 Android 特定桥接的

| Operit 功能 | 方案 | 工作量 |
|------------|------|--------|
| Ubuntu 24 环境 | Termux JNI | ~500 行 |
| 无障碍服务 | Android JNI | ~300 行 |
| 前台通知 | Android JNI | ~200 行 |
| 本地模型 | candle (纯 Rust) | 零（但需下载模型） |

### 评分：★★★★★

Operit 是目标 Android 平台。Hermes_Rust_Operit_App 不需要重新实现 Operit 的功能，而是用 Rust 工具链替换其 Kotlin 实现。核心工作：将 hermes-agent-rs 的 Rust 能力通过 JNI 暴露给 Android。
