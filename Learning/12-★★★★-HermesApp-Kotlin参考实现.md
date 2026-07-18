# 03 — HermesApp：Kotlin 参考实现源码级分析

> **仓库**：https://github.com/SelectXn00b/HermesApp (194⭐, Kotlin)  
> **签名**："Hermes 内核(Kotlin) + Operit 壳(Android)"  
> **关键发现**：Hermes_Rust_Operit_App 不需要复制 Kotlin 代码，hermes-agent-rs 已是更完整的 Rust 版

---

## 一、实际 Kotlin 源码分析

从 HermesApp 的 GitHub 下载了 **6 个关键 Kotlin 文件，总 233KB**：

| 文件 | 大小 | 对应 Rust 方案 |
|------|------|--------------|
| `ToolExecutionManager.kt` | **28KB** | hermes-tools registry.rs + dispatch.rs |
| `ConversationService.kt` | **54KB** | hermes-agent agent_loop.rs |
| `FileBindingService.kt` | **31KB** | hermes-tools file.rs |
| `AIServiceFactory.kt` | **21KB** | hermes-agent provider.rs |
| `MemoryLibrary.kt` | **46KB** | hermes-agent memory_manager.rs |
| `WorkflowExecutor.kt` | **52KB** | hermes-agent skill_orchestrator.rs |

---

## 二、核心类的实现细节

### ToolExecutionManager.kt（28KB）

```kotlin
class ToolExecutionManager {
    // 工具目标解析
    private fun resolveToolTarget(tool: AITool): ResolvedToolTarget
    // 工具名显示
    private fun resolveDisplayToolName(tool: AITool): String
    // JS 包工具检测
    private fun isJsPackageTool(toolName: String, jsPackageNames: Set<String>): Boolean
    // 代理参数解析
    private fun resolveProxyParameters(tool: AITool): List<ToolParameter>
    // 角色卡权限控制
    private fun isInvocationAllowedForRoleCard(tool: AITool): Boolean
}
```

**对应 Rust**：hermes-tools 的 `ToolRegistry::register()` + `dispatch.rs` 的并行执行

### ConversationService.kt（54KB，最大的文件）

```kotlin
class ConversationService {
    suspend fun generateSummary()           // 对话摘要
    suspend fun generateSummaryFromPromptTurns()  // 从 prompt 轮次生成摘要
    suspend fun prepareConversationHistory() // 准备对话历史
    fun normalizeConversationHistoryForModel()    // 规范化为模型格式
    suspend fun processChatMessageWithTools()     // ★核心：处理带工具的聊天消息
    fun buildPreferencesText()                    // 构建偏好文本
    suspend fun buildPersistentInstructionsText()  // 构建持久指令
}
```

对应 Rust：hermes-agent 的 `agent_loop.rs` (7,001行) 的 `run()` + `run_stream()`

**Rust 复刻**：`cargo add hermes-agent` → 直接使用 agent_loop.run()

### AIServiceFactory.kt（21KB）

```kotlin
class AIServiceFactory {
    fun createService(config: ModelConfigData): AIService
    // 内部: parseCustomHeaders
    //        buildAndroidLlamaSessionConfig
    //        路由到 OpenAI/Claude/DeepSeek/Gemini/Ollama...
}
```

**对应 Rust**：hermes-agent 的 `provider.rs` + 10+ 内置 Provider

### MemoryLibrary.kt（46KB）

```kotlin
class MemoryLibrary {
    // 内存数据结构:
    data class ParsedLink(title, targetTitle, type, description, weight)
    data class ParsedEntity(title, content, tags, aliasFor, folderPath)
    data class ParsedUpdate(titleToUpdate, newContent, reason, ...)
    data class ParsedMerge(sourceTitles, newTitle, newContent, ...)
}
```

**对应 Rust**：hermes-agent 的 `memory_manager.rs` + 8 种记忆插件

---

## 三、HermesApp 源码 vs hermes-agent-rs 源码对照

| 功能 | HermesApp (Kotlin) 行数 | hermes-agent-rs (Rust) 行数 | 谁更完整 |
|------|----------------------|--------------------------|---------|
| Agent 循环 | `ConversationService.kt` 54KB | `agent_loop.rs` 7,001 行 | Rust ✅ |
| 工具执行 | `ToolExecutionManager.kt` 28KB | `registry.rs` 423 + `dispatch.rs` 200 | Rust ✅ |
| 记忆 | `MemoryLibrary.kt` 46KB | `memory_manager.rs` 657 + 8 插件 | Rust ✅ (1种vs8种) |
| Provider | `AIServiceFactory.kt` 21KB | `provider.rs` ~3K | Rust ✅ |
| 工作流 | `WorkflowExecutor.kt` 52KB | `skill_orchestrator.rs` 473 | Rust ✅ |
| Android 特定 | 全部 Kotlin 代码 | JNI 桥接 | Kotlin ✅ (需桥接) |

**核心结论**：hermes-agent-rs 在 Agent 引擎的每个维度上都比 HermesApp 更完整。

---

## 四、评分：★★★★

HermesApp 的价值不在于代码质量，而在于它**证明了 "Hermes 内核 + Android 壳" 架构的可行性**。Rust 版不需要复制其 Kotlin 代码，只需要参考其 Android 特定功能清单（无障碍、Shizuku、通知）。
