# 01 — hermes-agent-rs：Rust Agent 引擎源码深度分析

> **仓库**：https://github.com/Lumio-Research/hermes-agent-rs  
> **UA Rust 分析数据**：700 nodes / 694 edges / 4 layers / 18 crates / 352 Rust 文件  
> **Hermes_Rust_Operit_App 评分**：★★★★★（最高优先级，直接作为核心依赖）

---

## 一、源码结构（UA Rust 扫描发现）

### 18 个 crate 的文件级结构

```
hermes-agent-rs/
├── crates/
│   ├── hermes-acp/       (7文件) — ACP 协议实现
│   ├── hermes-agent/     (39文件, 24.6K行) — ★Agent 引擎核心
│   ├── hermes-bus/       (5文件) — 事件总线
│   ├── hermes-cli/       (20+文件, 14K行) — CLI 界面
│   ├── hermes-config/    (16文件, 4.4K行) — 配置系统
│   ├── hermes-core/      (7文件, 1.5K行) — 核心类型系统
│   ├── hermes-cron/      — 定时任务
│   ├── hermes-environments/ — 环境管理
│   ├── hermes-eval/      — Agent 评估
│   ├── hermes-gateway/   (45文件, 27K行) — 多平台网关
│   ├── hermes-intelligence/ — 智能层
│   ├── hermes-mcp/       (6文件, 3.4K行) — ★MCP 协议客户端
│   ├── hermes-parity-tests/ — 一致性测试
│   ├── hermes-server/    (14文件, 3.1K行) — HTTP 服务
│   ├── hermes-skills/    (6文件, 1.8K行) — ★技能系统
│   ├── hermes-telemetry/ — 遥测
│   ├── hermes-tools/     (69文件, 23K行) — ★工具系统
│   └── hermes-transport/ — 传输层
├── apps/
│   ├── dashboard/        — React 管理后台
│   ├── web-app/          — Electron 桌面端
│   └── mobile-app/       — React Native 移动端
```

---

## 二、AgentLoop 源码级实现（agent_loop.rs, 7,001 行）

### AgentLoop 结构体（第 731 行）

```rust
pub struct AgentLoop {
    pub config: AgentConfig,                          // Agent 配置（30+ 参数）
    pub tool_registry: Arc<ToolRegistry>,              // 工具注册中心
    pub llm_provider: Arc<dyn LlmProvider>,            // LLM 提供者
    pub interrupt: InterruptController,                // 中断控制
    pub memory_manager: Option<Arc<Mutex<MemoryManager>>>, // 可选记忆管理
    pub plugin_manager: Option<Arc<Mutex<PluginManager>>>, // 可选插件管理
    pub callbacks: Arc<AgentCallbacks>,                // 回调
    pub delegate_depth: u32,                           // 子 Agent 深度
    pub primary_credential_pool: Option<Arc<CredentialPool>>, // 凭据池
    pub evolution_counters: Arc<Mutex<EvolutionCounters>>,   // 进化计数器
    sub_agent_orchestrator: Option<Arc<SubAgentOrchestrator>>, // 子 Agent 编排
    pending_steer: Arc<Mutex<Option<String>>>,         // 运行中 steer
    oauth_refresh_backoff: Arc<Mutex<HashMap<String, Instant>>>,
}
```

### Builder 模式（第 904 行起）

```rust
// 使用方式：
let agent = AgentLoop::new(config)
    .with_interrupt(interrupt)
    .with_memory(memory_manager)
    .with_plugins(plugin_manager)
    .with_callbacks(callbacks)
    .with_sub_agent_orchestrator(orchestrator)
    .with_primary_credential_pool(pool);

// 两种运行方式：
agent.run().await;        // 阻塞式，返回 AgentResult
agent.run_stream().await; // 流式，返回 SSE 事件流
```

### AgentConfig（第 250 行，30+ 配置项）

关键配置项包括：

| 配置 | 默认值 | 作用 |
|------|--------|------|
| `max_turns` | 50 | 最大对话轮次 |
| `max_concurrent_delegates` | 3 | 最大并发子 Agent |
| `memory_flush_interval` | 10 | 记忆刷新间隔（轮次） |
| `cost_guard_degrade_at_ratio` | 0.85 | 成本守卫降级阈值 |
| `checkpoint_interval_turns` | 5 | 检查点间隔 |
| `rollback_on_tool_error_threshold` | 3 | 工具错误回滚阈值 |
| `budget_caution_threshold` | 0.7 | 预算警告阈值 |
| `budget_warning_threshold` | 0.9 | 预算严重阈值 |
| `empty_content_max_retries` | 2 | 空内容重试 |
| `invalid_tool_call_max_retries` | 2 | 无效工具调用重试 |
| `stream_activity_stall_secs` | 120 | 流超时（秒） |

### 核心运行流程（run 方法，第 2695 行）

```
run() →
  1. 从 SQLite 恢复 system_prompt（hydrate_stored_system_prompt）
  2. 构建 ContextManager
  3. 循环：
     a. 将消息 + 工具 schema 发给 LLM
     b. LLM 返回文本 → 结束循环
     c. LLM 返回工具调用 → 并行执行（tokio JoinSet）
     d. 工具结果注入下一轮
     e. 检查 max_turns / 预算 / 中断
  4. 记忆存储 + 技能进化提示
  5. 返回 AgentResult
```

---

## 三、工具系统源码（hermes-tools, 69 文件, 23K 行）

### 工具注册机制（registry.rs）

```rust
pub struct ToolEntry {
    pub name: String,
    pub description: String,
    pub handler: Arc<dyn Fn(ToolCall) -> ToolResult + Send + Sync>,
    pub schema: ToolSchema,
}

pub struct ToolRegistry {
    inner: Arc<RwLock<ToolRegistryInner>>,
}

impl ToolRegistry {
    pub fn new() -> Self;
    pub fn register(&self, entry: ToolEntry);
    pub fn get(&self, name: &str) -> Option<&ToolEntry>;
    pub fn schemas(&self) -> Vec<ToolSchema>;  // 给 LLM 的 tools 参数
    pub fn names(&self) -> Vec<String>;
}
```

### 35 个工具的 Handler 模式

每个工具都是 **Handler 结构体 + Backend trait** 的双层设计：

```rust
// 浏览器工具示例（browser.rs）
pub struct BrowserNavigateHandler {
    backend: Arc<dyn BrowserBackend>,
}
impl BrowserNavigateHandler {
    pub fn new(backend: Arc<dyn BrowserBackend>) -> Self { ... }
}

// 后端 trait（可切换实现）
pub trait BrowserBackend: Send + Sync {
    async fn navigate(&self, url: &str) -> Result<(), Error>;
    async fn snapshot(&self) -> Result<String, Error>;
    async fn click(&self, selector: &str) -> Result<(), Error>;
    // ...
}

// 工具清单（每个工具一个文件）
tools/
├── browser.rs       — 导航/截图/点击/输入/滚动/JS执行
├── code_execution.rs — 代码执行（Python/Rust/Shell）
├── vision.rs        — 图片分析
├── file.rs          — 文件读写
├── web.rs           — 网页搜索+抓取
├── terminal.rs      — 终端命令
├── memory.rs        — 记忆读写
├── skills.rs        — 技能管理
├── cronjob.rs       — 定时任务
├── delegation.rs    — 子 Agent 委派
├── tts.rs           — 语音合成
├── transcription.rs — 语音识别
├── todo.rs          — 待办事项
├── session_search.rs— 会话搜索
├── clarify.rs       — 追问澄清
├── messaging.rs     — 消息平台
├── image_gen.rs     — 图片生成
├── video_gen.rs     — 视频生成
├── audio_gen.rs     — 音频生成
├── homeassistant.rs — 智能家居
├── ...共 35 个工具
```

---

## 四、记忆系统（memory_manager.rs + 8 种插件）

```rust
pub trait MemoryProviderPlugin: Send + Sync {
    async fn store(&self, content: &str) -> Result<(), Error>;
    async fn recall(&self, query: &str) -> Result<Vec<String>, Error>;
    async fn forget(&self, id: &str) -> Result<(), Error>;
    fn name(&self) -> &'static str;
}
```

8 种内置记忆插件：

| 插件 | 后端 | 特点 |
|------|------|------|
| `byterover` | 本地 | 基础记忆 |
| `hindsight` | 本地 | 后见之明总结 |
| `holographic` | 本地 | 全息压缩 |
| `honcho` | 云端 | Honcho API |
| `mem0` | 本地/云端 | Mem0 记忆 |
| `openviking` | 本地 | 维京记忆 |
| `retaindb` | 本地 | RetainDB |
| `supermemory` | 云端 | SuperMemory API |

---

## 五、对 Hermes_Rust_Operit_App 的整合方式

### 不需要编写的代码

hermes-agent-rs 可以直接作为 cargo dependency 引入：

```toml
[dependencies]
# 核心依赖（必须）
hermes-core = { git = "https://github.com/Lumio-Research/hermes-agent-rs" }
hermes-agent = { git = "..." }
hermes-tools = { git = "...", features = ["default"] }
hermes-mcp = { git = "..." }
hermes-config = { git = "..." }

# 可选
hermes-skills = { git = "..." }
hermes-cron = { git = "..." }
```

### 需要编写的

| 组件 | 行数预估 | 说明 |
|------|---------|------|
| Dioxus UI 界面 | ~3,000 | 对话/商店/设置 |
| Android JNI 桥接 | ~1,000 | 无障碍/Termux/通知 |
| MCP 插件管理 | ~500 | 四 Tab 商店 |
| 配置适配 | ~300 | Android 存储适配 |

### 评分：★★★★★

hermes-agent-rs 是整个 Hermes_Rust_Operit_App 的**心脏**。不引入它就意味着从零重新实现 Agent 循环、35 个工具、8 种记忆、MCP 客户端、技能系统——这是至少 100K 行代码的工作量。
