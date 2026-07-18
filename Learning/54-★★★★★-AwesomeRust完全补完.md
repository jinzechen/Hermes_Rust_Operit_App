# 全部遗漏关键概念补完 — MCP/SKill/沙盒/内嵌/Android

> **从 awesome-rust 58K⭐ 提取的最后一批关键项目**

---

## 一、记忆引擎 — VelesDB (75⭐)

**定位**：可解释的本地优先 AI Agent 记忆引擎。融合向量+图+列存三位一体。

```rust
// VelesQL 查询记忆
// 语义记忆："用户上次说的 Rust 项目"
// 情景记忆："昨天 3pm 发生了什么"  
// 程序记忆："重构 Hermes 的步骤"
```

**对 Hermes 的作用**：替代 tinycortex + qdrant 的组合，一个库搞定全部记忆。

**Hermes 评分**：★★★★（太新 75⭐，但架构吸引人）

---

## 二、沙盒执行 — microsandbox (6,962⭐)

**定位**：轻量微VM沙盒，毫秒级隔离代码执行。

| 沙盒方案 | 隔离级别 | 启动时间 | Android 支持 |
|---------|---------|---------|-------------|
| wasmer | Wasm 沙盒 | <1ms | ✅ |
| **microsandbox** | **微VM** | **~10ms** | ❌ Linux |
| youki (7.4K⭐) | OCI 容器 | ~100ms | ❌ |
| firecracker (35K⭐) | KVM 微VM | ~125ms | ❌ |

**Hermes 评分**：★★★（Linux only，Hermes 主要用 wasmer）

---

## 三、嵌入式脚本 — rhai (4,000+⭐)

**定位**：轻量嵌入式脚本语言（类似 JS+Rust 混合体）。

| 引擎 | 大小 | 语法 | Android |
|------|------|------|---------|
| boa_engine | ~7MB | JS | ✅ |
| **rhai** | **~200KB** | **Rust-like** | ✅ |
| rune | ~3MB | Rust-like | ✅ |

```rust
use rhai::Engine;

let engine = Engine::new();
let result: i64 = engine.eval("40 + 2")?;  // 42
```

**Hermes 评分**：★★★★（比 boa 轻量 35 倍，适合 Android）

---

## 四、Android 原生 UI — Blinc

**定位**：GPU 加速跨平台 UI 框架（Desktop + Android + iOS）。

| UI 框架 | Android | GPU | Stars |
|---------|---------|-----|-------|
| Dioxus | ✅ dioxus-native | ❌ | 36K⭐ |
| **Blinc** | **✅ 原生** | **✅ GPU** | — |
| Slint | ✅ | ✅ | 17K⭐ |
| egui | ❌ WebView | ✅ | 29K⭐ |

**Hermes 评分**：★★★（观察期，Dioxus 仍是主力）

---

## 五、嵌入式数据库 — native_db

**定位**：即插即用的嵌入式多平台数据库。Rust 类型直接映射。

```rust
use native_db::*;

#[derive(Serialize, Deserialize)]
struct AgentState { /* Hermes 状态 */ }
db.rw_transaction()?.insert(state)?;
```

**Hermes 评分**：★★★（redb 已够用）

---

## 六、功能重合项目最终列表

| 项目 | Stars | 与 Hermes 重合点 | 分析状态 |
|------|-------|-----------------|---------|
| **thClaws** | 1,166⭐ | 三Tab+MCP+Skills+Rust Agent | ✅ 已分析 |
| **Bamboo-agent** | 12⭐ | 嵌入 Agent 运行时 | ✅ 已分析 |
| **octomind** | — | 48 Agent+MCP | ✅ 已分析 |
| **aichat** | — | LLM CLI+RAG+Tools | ✅ 已分析 |
| **Tura** | 155⭐ | 本地编码 Agent | 🔍 新发现 |
| **bitrouter** | — | LLM 路由+MCP 网关 | 🔍 新发现 |
| **DeepSeek-TUI** | — | 编码 Agent+MCP | 🔍 新发现 |
| **hcom** | — | 跨终端 Agent 通信 | 🔍 新发现 |
| **flowcat** | — | 实时语音 AI Agent | 🔍 新发现 |
| **dora-rs** | 3,847⭐ | 多 AI 数据流编排 | 🔍 新发现 |

### 最终结论

awesome-rust 58K⭐ 共提取 **53 个项目**，覆盖了：
- ✅ 核心引擎（hermes-agent-rs/mistral.rs/candle）
- ✅ Android 桥接（Shizuku/无障碍/前台/通知）
- ✅ UI（Dioxus/Operit/HermesApp 50+页面）
- ✅ MCP（RMCP/Operit_MCPS）
- ✅ Sandbox（wasmer/microsandbox/youki）
- ✅ Skills（thClaws/Bamboo/fabric）
- ✅ 记忆（openhuman/VelesDB/tinycortex）
- ✅ 嵌入式脚本（boa/rhai）
- ✅ 系统工具（clipboard/moka/procs）
- ✅ 功能重合项目（thClaws/Bamboo/octomind）
