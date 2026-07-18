# Hermes_Rust_Operit_App — Technical Architecture & Design

**Version**: 1.0  
**Date**: 2026-07-19  
**Author**: deepseek-v4-pro (handoff from deepseek-v4-flash)  
**Repository**: [github.com/jinzechen/Hermes_Rust_Operit_App](https://github.com/jinzechen/Hermes_Rust_Operit_App)

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Research Foundation](#2-research-foundation)
3. [System Architecture](#3-system-architecture)
4. [Module Design](#4-module-design)
5. [Data Flow & Protocol](#5-data-flow--protocol)
6. [Implementation Roadmap](#6-implementation-roadmap)
7. [Build & Deployment](#7-build--deployment)
8. [Competitive Analysis](#8-competitive-analysis)
9. [Risk & Mitigation](#9-risk--mitigation)

---

## 1. Executive Summary

`Hermes_Rust_Operit_App` is a **pure-Rust AI agent operating system for Android**. It combines:

- An LLM agent engine (ReAct loop + 35 tools + sub-agent orchestration)
- Android system-level integration (Shizuku ADB privileges, AccessibilityService, foreground service anti-kill)
- A plugin marketplace (MCP protocol, artifact/skill/MCP three-tab store)
- Local inference (mistral.rs), voice (sherpa-onnx), sandboxed execution (wasmer)
- Code analysis (UA Rust knowledge graph engine, already integrated as path dependency)

The architecture is the synthesis of **63 learning reports** on Rust AI agent ecosystem projects, **74 UA knowledge graph analyses** (JSON + Markdown + HTML), and **24 existing Rust source files** (~3,200 lines of working code).

### Current Status

| Component | Files | Lines | Status |
|-----------|-------|-------|--------|
| `core/agent.rs` | 1 | 238 | Agent loop with tool execution, max 20 iterations |
| `core/provider.rs` | 1 | 386 | OpenAI + Anthropic providers, format conversion |
| `core/tool_registry.rs` | 1 | 77 | HashMap-based tool registry |
| `core/config.rs` | 1 | 75 | YAML/JSON config loading |
| `core/memory.rs` | 1 | — | In-memory HashMap storage |
| `tools/*.rs` | 5 | ~800 | filesystem(367L), markdown(321L), browser(109L), vision, codebase_analyzer |
| `ui/*.rs` | 4 | — | chat, login, settings, store_page stubs |
| `mcp/client.rs` | 1 | ~400 | Custom JSON-RPC MCP client |
| `environment/sandbox.rs` | 1 | 71 | Placeholder wrapper |
| **Total** | **24** | **~3,200** | |

---

## 2. Research Foundation

### 2.1 Project Analysis Inventory

This architecture is grounded in systematic analysis of the Rust AI agent ecosystem:

```
Data Source                           Count    Location
──────────────────────────────────────────────────────────────────────
Learning Reports (Markdown)           63       Learning/
  ★★★★★ (core)                        28       ─ core architecture reference
  ★★★★  (important)                   22       ─ ecosystem survey
  ★★★   (reference)                    9       ─ supplementary
  ★★    (awareness)                    4       ─ background

UA Rust Knowledge Graphs              74       Understand_Anything_Rust/Output/
  JSON (machine-readable)             74       Output/json/
  Markdown (human-readable)           74       Output/md/
  HTML (D3.js interactive)            23       Output/html/

Cloned Repositories                     3
  hermes-agent-rs (352 Rust files)            D:\...\hermes-agent-rs
  Operit_MCPS (659 Rust files)                D:\...\Operit_MCPS
  Understand_Anything_Rust (28 Rust files)    D:\...\Understand_Anything_Rust
```

### 2.2 Key Reference Projects

| Project | Stars | Language | Fidelity | Key Takeaways |
|---------|-------|----------|----------|---------------|
| **hermes-agent-rs** | 72 | Rust | ★★★★★ 1:1 | 18 crates, 700 nodes, 35 tools — direct dependency candidate |
| **thClaws** | 1,166 | Rust | ★★★★★ near-identical | 3-tab UI (Chat/Files/Terminal), MCP, skills, agent teams |
| **claude-code-rust** | 1,667 | Rust | ★★★★★ near-identical | engine/tools/mcp/skills/clipboard/git modules |
| **goose** | LF | Rust | ★★★★★ high overlap | 15+ Provider, Tauri desktop, 70+ MCP extensions |
| **ZeptoClaw** | 644 | Rust | ★★★★ modular ref | 25+ modules, 7-layer security, 4MB binary |
| **rtk** | 71,641 | Rust | ★★★★★ cost-critical | Token cache (60-90% reduction), model router |
| **headroom** | 59,000 | Rust | ★★★★★ cost-critical | JSON compression (60-95%), code compression (20%) |
| **obscura** | 19,392 | Rust | ★★★★★ browser | CDP-based headless browser, MCP mode |
| **wasmer** | 20,904 | Rust | ★★★★★ sandbox | Wasm runtime, Android support, 0ms cold start |
| **mistral.rs** | 7,492 | Rust | ★★★★★ inference | Pure Rust, ISQ quantization, OpenAI-compatible API |
| **sherpa-onnx** | 13,630 | C++/Rust | ★★★★★ voice | ASR/TTS/VAD, prebuilt Android .so |
| **openhuman** | 35,015 | Rust | ★★★★★ memory | tinycortex engine, L0→L1→L2 summary tree |
| **Dioxus** | 36,804 | Rust | ★★★★★ UI | Only Rust framework with Android native rendering |
| **rmcp** | 3,639 | Rust | ★★★★★ protocol | Official MCP Rust SDK, stdio+SSE transport |

### 2.3 UA Rust Topology Data

| Project | Files | Rust Files | Nodes | Edges | Layers |
|---------|-------|------------|-------|-------|--------|
| Operit_MCPS | 1,229 | 659 | 1,505 | 1,556 | 5 |
| Understand_Anything_Rust (self) | — | 28 | 1,641 | 1,623 | 3 |
| hermes-agent-rs | 579 | 352 | 700 | 694 | 4 |

Operit_MCPS is the largest codebase in our ecosystem — 1,505 nodes, 1,556 dependency edges across 5 architectural layers.

---

## 3. System Architecture

### 3.1 Layer Diagram

```
┌──────────────────────────────────────────────────────────────────────┐
│                        APPLICATION LAYER                              │
│  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐ ┌──────────────┐ │
│  │  Chat   │ │  Market  │ │ Toolbox  │ │ Memory │ │  Settings    │ │
│  │ (Dioxus)│ │ (3 tabs) │ │(9 tools) │ │ (cards)│ │ (25+ pages)  │ │
│  └────┬────┘ └────┬─────┘ └────┬─────┘ └───┬────┘ └──────┬───────┘ │
├───────┼───────────┼────────────┼───────────┼──────────────┼─────────┤
│                         SERVICE INTERFACE                             │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │              HermesService (gRPC / REST / IPC)                  │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────────────┐  │ │
│  │  │ ChatAPI  │ │StoreAPI  │ │ToolAPI   │ │ AgentOrchestrator │  │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └───────────────────┘  │ │
│  └────────────────────────────────────────────────────────────────┘ │
├──────────────────────────────────────────────────────────────────────┤
│                          CORE ENGINE                                  │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  AgentLoop (hermes-agent-rs / self-implemented)               │   │
│  │  ┌──────────┐ ┌──────────┐ ┌───────────┐ ┌───────────────┐  │   │
│  │  │Provider  │ │Tool      │ │Memory     │ │SubAgent       │  │   │
│  │  │Router    │ │Registry  │ │Manager    │ │Orchestrator   │  │   │
│  │  └──────────┘ └──────────┘ └───────────┘ └───────────────┘  │   │
│  └──────────────────────────────────────────────────────────────┘   │
├──────────────────────────────────────────────────────────────────────┤
│                       CAPABILITY LAYER                                 │
│  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ │
│  │ MCP    │ │ Skills │ │ Cron   │ │ OAuth  │ │ Voice  │ │ Code   │ │
│  │ Client │ │ Orch.  │ │ Sched. │ │ Pool   │ │ Engine │ │ Sandbx │ │
│  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ │
├──────────────────────────────────────────────────────────────────────┤
│                       PLATFORM LAYER                                   │
│  ┌────────┐ ┌────────┐ ┌───────────┐ ┌──────────┐ ┌──────────────┐ │
│  │Shizuku │ │Accessi-│ │Foreground │ │Notificat-│ │ Clipboard    │ │
│  │Bridge  │ │bility  │ │Service    │ │ion       │ │ Bridge       │ │
│  └────────┘ └────────┘ └───────────┘ └──────────┘ └──────────────┘ │
├──────────────────────────────────────────────────────────────────────┤
│                        STORAGE LAYER                                   │
│  ┌────────┐ ┌──────────┐ ┌───────────┐ ┌──────────┐                 │
│  │ redb   │ │ tantivy  │ │ moka      │ │ SQLite   │                 │
│  │ (KV)   │ │ (FTS)    │ │ (Cache)   │ │ (Memory) │                 │
│  └────────┘ └──────────┘ └───────────┘ └──────────┘                 │
└──────────────────────────────────────────────────────────────────────┘
```

### 3.2 Module Dependency Graph

```
main.rs
  └── hermes_operit_core (lib.rs)
        ├── core/
        │   ├── agent.rs        → provider.rs, tool_registry.rs, config.rs, memory.rs
        │   ├── provider.rs     → config.rs, reqwest
        │   ├── tool_registry.rs → (standalone trait)
        │   ├── config.rs       → serde_yaml, serde_json
        │   └── memory.rs       → redb (future: tinycortex)
        ├── tools/
        │   ├── filesystem.rs   → tool_registry, walkdir, regex, flate2
        │   ├── markdown.rs     → tool_registry, regex
        │   ├── browser.rs      → tool_registry (future: obscura CDP)
        │   ├── vision.rs       → tool_registry
        │   └── codebase_analyzer.rs → ua-core (path dep)
        ├── mcp/
        │   └── client.rs       → (future: rmcp crate)
        ├── environment/
        │   └── sandbox.rs      → (future: wasmer)
        ├── store/
        │   └── mod.rs
        └── ui/
            ├── chat.rs         → (future: dioxus)
            ├── login.rs
            ├── settings.rs
            └── store_page.rs
```

### 3.3 Key Data Types & Trait Contracts

```rust
// ── Tool System Trait (zero-overhead dispatch) ──────────────────

/// Every tool implements this trait. Called synchronously by AgentLoop
/// after the LLM returns a tool_call. No IPC, no serialization round-trip.
pub trait ToolHandler: Send + Sync {
    fn execute(&self, params: serde_json::Value) -> anyhow::Result<String>;
    fn schema(&self) -> ToolSchema;
}

pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,  // JSON Schema object
}

// ── LLM Provider Trait ──────────────────────────────────────────

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat_completion(
        &self,
        messages: Vec<serde_json::Value>,  // [{role, content}]
        tools: Vec<serde_json::Value>,      // tool schemas
        config: &AppConfig,
    ) -> anyhow::Result<LlmResponse>;
}

pub struct LlmResponse {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
}

// ── Agent Message Loop ──────────────────────────────────────────

pub struct AgentManager {
    config: AppConfig,
    provider: Box<dyn LlmProvider>,
    tool_registry: Arc<RwLock<ToolRegistry>>,
    sessions: Arc<Mutex<HashMap<String, Vec<Message>>>>,
    runtime: tokio::runtime::Runtime,
}
```

---

## 4. Module Design

### 4.1 Agent Engine (`core/agent.rs`)

**Current** (238 lines): Self-implemented ReAct loop with `AgentManager`.

**Target**: Replace with `hermes-agent-rs` as a direct cargo dependency OR extend current implementation to match its feature set.

**hermes-agent-rs reference architecture** (7,001-line AgentLoop):

| Component | Lines | Function |
|-----------|-------|----------|
| `agent_loop.rs` | 7,001 | Main loop: messages → LLM → tools → repeat |
| `agent_builder.rs` | — | Builder pattern for AgentLoop construction |
| `sub_agent_orchestrator.rs` | 617 | Spawn child agents for parallel work |
| `smart_model_routing.rs` | 419 | Route by task type (compile→pro, docs→flash) |
| `skill_orchestrator.rs` | — | Load SKILL.md, inject into system prompt |
| `memory_manager.rs` | 657 | 8 memory plugins (byterover, hindsight, holographic, honcho, mem0, openviking, retaindb, supermemory) |
| `interrupt.rs` | — | Interrupt & resume long-running agent loops |
| `compression.rs` | — | Context window compression |
| `budget.rs` | — | Token budget tracking & enforcement |
| `fallback.rs` | — | Provider failover chain |
| `rate_limit.rs` | — | Rate limit enforcement |
| `oauth.rs` | — | OAuth credential management |
| `credential_pool.rs` | — | Rotating API key pool |
| `session_persistence.rs` | — | Session save/restore |
| `plugins.rs` | — | Plugin lifecycle management |

**Decision**: Phase 1 keeps self-implemented `AgentLoop` (238 lines, working), add features incrementally. Phase 2 evaluates `cargo add hermes-agent` as direct dependency.

### 4.2 LLM Provider Layer (`core/provider.rs`)

**Current** (386 lines): `GenericProvider` (OpenAI-compatible) + `AnthropicProvider` with full format conversion.

**Provider Architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│                   SmartModelRouter                          │
│  ┌─────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ Task    │→│ Route to     │→│ Provider              │  │
│  │ Classify│  │ best model   │  │                      │  │
│  │         │  │              │  │ compile/code → pro   │  │
│  │ ─────── │  │ ──────────── │  │ docs/reports → flash │  │
│  │ compile │  │ deepseek-v4- │  │ chat/simple  → mini  │  │
│  │ report  │  │ pro          │  │ local/offline→mistral│  │
│  │ chat    │  │ deepseek-v4- │  │                      │  │
│  │ simple  │  │ flash        │  │                      │  │
│  └─────────┘  └──────────────┘  └──────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Token Optimization Layer                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │ rtk (HTTP    │  │ headroom     │  │ moka cache       │  │
│  │ proxy)       │  │ (JSON cmpr)  │  │ (L1 hot cache)   │  │
│  │ 60-90% save  │  │ 60-95% save  │  │ 100x faster than │  │
│  │ duplicate     │  │ tool output  │  │ API call         │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   API Dispatch                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │GenericProvider│  │AnthropicProv.│  │ MistralRsProvider│  │
│  │(OpenAI compat)│  │(Messages API)│  │ (local inference)│  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

**Supported endpoints**:
- Xiaomi MiMo Singapore (`token-plan-sgp.xiaomimimo.com`) — primary
- DeepSeek API (`api.deepseek.com`) — v4-pro, v4-flash
- OpenAI API (`api.openai.com`)
- Anthropic API (`api.anthropic.com`)
- Local (`127.0.0.1:8080` via mistral.rs)

### 4.3 Tool System (`tool_registry.rs` + `tools/`)

**ToolHandler trait** — the zero-overhead dispatch mechanism:

```
ToolHandler (trait, 0 IPC overhead)
│
├── Built-in Tools (compiled into binary, ToolHandler::execute)
│   ├── filesystem.rs    [367L]  — read/write/list/search/get_info/create/delete/move
│   ├── markdown.rs      [321L]  — render/extract_code/strip_formatting
│   ├── browser.rs       [109L]  — navigate/screenshot/get_html/click/type_text (placeholder)
│   ├── vision.rs        [~80L]  — image analysis (placeholder)
│   └── codebase_analyzer.rs     — UA Rust integration (via ua-core path dep)
│
├── To Be Built-in (extract core logic from Operit_MCPS MCP plugins)
│   ├── terminal.rs              — shell command execution
│   ├── web.rs                   — web search (DuckDuckGo/SearXNG + scraper)
│   ├── process.rs               — background process management
│   ├── session_search.rs        — FTS5 session search (tantivy)
│   ├── cronjob.rs               — scheduled task management
│   ├── delegation.rs            — sub-agent spawning
│   ├── memory.rs                — memory read/write (tinycortex)
│   ├── skills.rs                — SKILL.md scan & load
│   ├── tts.rs                   — text-to-speech (sherpa-onnx)
│   ├── transcription.rs         — speech-to-text (sherpa-onnx)
│   ├── clarify.rs               — user clarification prompts
│   └── todo.rs                  — task tracking
│
└── MCP Tools (external process, via rmcp client)
    ├── rust_mcp_server          — Rust toolchain (cargo check/build/test)
    ├── mcp_proxy                — proxy to external MCP servers
    ├── m3ux                     — audio processing (low frequency)
    └── rust_docs_mcp            — Rust documentation lookup
```

**Performance principle**: ToolHandler (0 overhead) > MCP via stdio (15ms) > Skills (50ms).

### 4.4 MCP Layer (`mcp/`)

**Current**: Custom 14.4KB `client.rs` implementing JSON-RPC 2.0 over stdio.

**Target**: Replace with `rmcp` crate (official MCP Rust SDK, 3,639⭐).

```
rmcp architecture (6 source files):
├── model/       — JSON-RPC 2.0 types (Request, Response, Notification)
├── service/     — RoleClient, RoleServer, Peer, Service, ServiceExt
├── handler/     — ClientHandler, ServerHandler traits
├── transport/   — StdioTransport, SseTransport
├── error/       — MCP error codes
└── task_manager/ — concurrent task management

Integration:
  [dependencies]
  rmcp = "0.8"

  use rmcp::{ServiceExt, Service};
  use rmcp::transport::stdio;

  let service = Service::new(transport).serve(handler).await?;
  let result = service.call_tool("filesystem_read", json!({"path": "/sdcard"}))?;
```

**Operit_MCPS plugin disposition**:

| Plugin | Rust Files | Action | Rationale |
|--------|-----------|--------|-----------|
| obscura | prebuilt binary | ✅ Build-in | High-frequency browser ops, 0ms ~ 15ms MCP |
| agentic_vision | 50+ | ✅ Build-in | Core capability, always needed |
| rust_mcp_filesystem | 25+ ops | ✅ Build-in | Filesystem ops are fundamental |
| typemill | self-written | ✅ Build-in | Markdown is ubiquitous |
| sherpa | self-written | ✅ Build-in | Voice is core Android differentiator |
| rust_mcp_server | many | ⚠️ MCP | Heavy, on-demand toolchain ops |
| mcp_proxy | few | ⚠️ MCP | External connection proxy |
| m3ux | few | ⚠️ MCP | Low-frequency audio tools |
| rust_docs_mcp | few | ⚠️ MCP | On-demand docs lookup |

### 4.5 Memory System (`core/memory.rs`)

**Current**: `HashMap<String, Vec<Message>>` — per-session in-memory message store.

**Target**: Replace with `tinycortex` (from openhuman, 35,015⭐).

```
tinycortex memory pipeline:

  User Message
      │
      ▼
  ┌──────────────────┐
  │ memory_queue      │  → Async worker pipeline (non-blocking)
  │ (tokio channel)  │
  └──────┬───────────┘
         │
         ▼
  ┌──────────────────┐
  │ memory_tree       │  → L0 (raw) → L1 (summary) → L2 (abstract)
  │ (summary engine) │     cascading summarization
  └──────┬───────────┘
         │
         ▼
  ┌──────────────────┐
  │ memory_store      │  → SQLite + vector embeddings + graph
  │ (persistence)    │
  └──────┬───────────┘
         │
         ▼
  ┌──────────────────┐
  │ memory_search     │  → Hybrid: vector (70%) + keyword (30%)
  │ (retrieval)      │     BM25 + cosine similarity fusion
  └──────────────────┘

Memory recall API:
  engine.recall("user preference for dark theme").await?
  → Returns ranked results with relevance scores
```

**Memory retention strategy**:

| Tier | Storage | Retention | Content |
|------|---------|-----------|---------|
| L0 (session) | HashMap | Current session | Full message history |
| L1 (recent) | SQLite | 30 days | Summarized sessions |
| L2 (durable) | SQLite + vectors | Indefinite | User preferences, facts, skills |
| Vector index | tantivy | Same as L2 | Semantic search over L2 |

### 4.6 Android Bridge Layer (new: `android/`)

This is the **core differentiator** — no other Rust AI agent runs on Android with system-level privileges.

```
android/
├── jni.rs                  — JNI initialization + function registration
│   #[no_mangle]
│   pub extern "C" fn Java_com_operit_hermes_HermesBridge_nativeInit(
│       env: JNIEnv, _class: JClass, config_json: JString) -> jlong
│
├── shizuku.rs              — Shizuku system privilege bridge
│   Capabilities gained:
│   - `input tap x y`        → AI-driven app interaction
│   - `input swipe x1 y1 x2 y2` → gesture control
│   - `input text "..."`     → automated text entry
│   - `input keyevent CODE`  → keyboard simulation
│   - `screencap`            → visual understanding
│   - `pm install/uninstall` → plugin auto-install
│
├── accessibility.rs        — AccessibilityService bridge
│   Reference: AccessibilityUITools.kt (33KB)
│   - Screen content reading (DOM-like tree)
│   - Element discovery (find by text/id/description)
│   - Gesture dispatch (click, swipe, scroll)
│   - Window state monitoring
│
├── foreground.rs           — ForegroundService anti-kill
│   Reference: AIForegroundService.kt (80KB)
│   - Persistent notification (required by Android)
│   - Wake lock management
│   - Microphone foreground service
│   - HTTP server keep-alive
│   - Voice wake lock
│
├── notification.rs         — Notification bridge
│   Reference: NotificationListener + SkillRecorderNotification (8KB)
│   - Notification reading (title, text, app, timestamp)
│   - Reply action dispatch
│   - Notification channel management
│
├── clipboard.rs            — Clipboard bridge
│   Reference: clipboard-rs (lib.rs 3.3KB + android.rs)
│   - Read/write system clipboard
│   - Content type detection (text, image, URL)
│   - Clipboard change monitoring
│
└── filesystem.rs           — Android SAF bridge
│   - Scoped Storage access
│   - DocumentProvider integration
│   - External storage (SD card) access
│
Reference source files (Kotlin, from HermesApp):
  HermesApp-Shizuku:     ShizukuInstaller.kt (12KB), ShizukuAuthorizer.kt (16KB)
  HermesApp-Accessibility: AccessibilityUITools.kt (33KB), Provider.kt (6KB)
  HermesApp-Foreground:  AIForegroundService.kt (80KB), ForegroundServiceCompat.kt (2KB)
  HermesApp-Notification:NotificationListener.kt (4KB), SkillRecorderNotification.kt (4KB)
  HermesApp-UI:          12 source files (~450KB), 50+ pages
```

### 4.7 UI Layer (`ui/`)

**Framework**: Dioxus 0.5 (36,804⭐, only Rust framework with Android native rendering).

**Layout** (reference: HermesApp OperitApp.kt + OperitScreens.kt, 50+ pages):

```
┌──────────────────────────────────────────────────────────────┐
│ [TopBar]  Title / Back / Actions                             │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─ Tab Navigation ──────────────────────────────────────┐  │
│  │ [AI Chat] [Market] [Toolbox] [Memory] [Settings]       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                              │
│  Tab: AI Chat                                                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ ┌──────────────────────────────────────────────────┐ │   │
│  │ │  [Message bubbles: user / assistant / tool]       │ │   │
│  │ │  (scrollable, infinite scroll, streaming)         │ │   │
│  │ └──────────────────────────────────────────────────┘ │   │
│  │ ┌──────────────────────────────────────────────────┐ │   │
│  │ │ [Voice input] [Text input_______________] [Send]   │ │   │
│  │ └──────────────────────────────────────────────────┘ │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  Tab: Market (3 sub-tabs)                                    │
│    [Artifacts] [Skills] [MCP]                                │
│    - Browse / Install / Manage / Publish                     │
│                                                              │
│  Tab: Toolbox (9 tool categories)                            │
│    File Manager / Terminal / Shell Executor / App Permissions│
│    UI Debugger / Logcat / SQL Viewer / Markdown / Speech     │
│                                                              │
│  Tab: Settings (25+ pages)                                   │
│    Provider / Model / Voice / Permissions / Account / ...    │
└──────────────────────────────────────────────────────────────┘
```

**Dioxus integration**:

```rust
// Cargo.toml (uncomment for Android build)
// dioxus = { version = "0.5", features = ["mobile"] }
// dioxus-mobile = "0.5"

fn HermesApp(cx: Scope) -> Element {
    let active_tab = use_state(cx, || Tab::Chat);
    let messages = use_state(cx, Vec::new);

    cx.render(rsx! {
        div { class: "app-container",
            TabBar { active: active_tab, on_change: move |t| active_tab.set(t) }
            match *active_tab.get() {
                Tab::Chat => rsx!(ChatView { messages: messages }),
                Tab::Market => rsx!(MarketView {}),
                Tab::Toolbox => rsx!(ToolboxView {}),
                Tab::Memory => rsx!(MemoryView {}),
                Tab::Settings => rsx!(SettingsView {}),
            }
        }
    })
}
```

### 4.8 Sandbox Execution (`environment/sandbox.rs`)

**Current**: 71-line placeholder wrapping `std::process::Command`.

**Target**: wasmer (20,904⭐) — WebAssembly sandbox runtime.

```
wasmer sandbox architecture:

  User Code (Python / Rust / JS / any→Wasm)
      │
      ▼
  wasmer::Module::from_file("user_code.wasm")
      │
      ▼
  wasmer::Instance::new(&module, &controlled_imports)
      │
      ├── Default: NO filesystem access
      ├── Default: NO network access
      ├── Memory limit: 256MB (configurable)
      ├── Execution timeout: 30s (configurable)
      └── Opcode counting: prevent infinite loops
      │
      ▼
  instance.exports.get_function("main")?.call(&[])?
      │
      ▼
  Return result → Agent

Advantages over alternatives:
  ┌──────────┬──────────┬───────────┬───────────┐
  │          │ wasmer   │ nsjail    │ Docker    │
  ├──────────┼──────────┼───────────┼───────────┤
  │ Start    │ <1ms     │ ~50ms     │ ~2s       │
  │ Android  │ ✅       │ ❌        │ ❌        │
  │ Default  │ No I/O   │ Config    │ Full      │
  │ isolation │          │ required  │ access    │
  │ Binary   │ ~5MB     │ ~500KB    │ ~50MB+    │
  │ size     │          │           │           │
  └──────────┴──────────┴───────────┴───────────┘
```

### 4.9 Local Inference (`hermes-local-inference`)

**Feature gate**: `local-inference`

```
[dependencies]
mistralrs = { version = "0.3", optional = true }

[features]
local-inference = ["mistralrs"]

// Provider:
impl LlmProvider for MistralRsProvider {
    async fn chat_completion(&self, messages, tools, config) -> Result<LlmResponse> {
        let request = ChatCompletionRequest { messages, tools, ... };
        let response = self.engine.send_chat_request(request).await?;
        Ok(convert_response(response))
    }
}
```

**Model selection for Android (constrained by RAM)**:

| Model | Size | RAM | Speed | Quality |
|-------|------|-----|-------|---------|
| Qwen2.5-1.5B-Instruct-GGUF (Q4) | ~1GB | ~2GB | Fast | Basic |
| Qwen2.5-3B-Instruct-GGUF (Q4) | ~2GB | ~3GB | Medium | Good |
| Llama-3.2-3B-Instruct-GGUF (Q4) | ~2GB | ~3GB | Medium | Good |

**Fallback strategy**: Local model for offline/no-network scenarios; cloud model (Xiaomi MiMo / DeepSeek) as primary.

### 4.10 Voice Engine

**Library**: sherpa-onnx (13,630⭐) + sherpa-rs (310⭐)

```
Android voice pipeline:

  ┌──────────────────────────────────────────────────┐
  │  sherpa-onnx Android prebuilt .so                │
  │  (aarch64-linux-android)                        │
  │                                                  │
  │  ASR (Speech → Text):                            │
  │    Models: Whisper-Tiny (~40MB), Zipformer (~30MB)│
  │    Input: 16kHz mono PCM audio buffer            │
  │    Output: UTF-8 text string                     │
  │                                                  │
  │  TTS (Text → Speech):                            │
  │    Models: VITS (~50MB), Matcha-TTS (~30MB)       │
  │    Input: UTF-8 text string                      │
  │    Output: 16kHz mono PCM audio buffer            │
  │                                                  │
  │  VAD (Voice Activity Detection):                 │
  │    Model: Silero VAD (~2MB)                       │
  │    Detects: speech start / speech end             │
  └──────────────────────────────────────────────────┘
```

### 4.11 Plugin Store (`store/`)

**Architecture**: GitHub Releases-based distribution (matching Operit_MCPS pattern).

```
Plugin lifecycle:

  1. Publish:  Developer pushes MCP plugin to GitHub Releases
               → binary (aarch64-musl) + index.js + package.json → ZIP

  2. Index:    Store index (JSON) tracks: name, version, sha256, size, description

  3. Discover: Hermes fetches store index → displays in Market tab
               Filters: category, popularity, compatibility

  4. Install:  Download ZIP → verify sha256 → extract to ~/hermes/plugins/
               → index.js auto-chmod 755 → register with MCP client

  5. Manage:   Enable/disable, update, uninstall, permission control
```

**CI pipeline** (existing in Operit_MCPS):

```yaml
# .github/workflows/build-mcp-plugins.yml
build:
  runs-on: ubuntu-latest
  strategy:
    fail-fast: false
    matrix:
      plugin: [obscura, agentic_vision, rust_mcp_filesystem, ...]
  steps:
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo install cross
    - run: cross build --target aarch64-unknown-linux-musl --release
    - run: python build/pack.py ${{ matrix.plugin }}
```

---

## 5. Data Flow & Protocol

### 5.1 Agent Loop (ReAct Pattern)

```
                    ┌─────────────────────────────┐
                    │     AgentLoop.run()          │
                    │                              │
    User Message ──▶│  1. Append to session        │
                    │  2. Build message array       │
                    │  3. Build tool schemas        │
                    │                              │
                    │        ┌──────────┐          │
                    │  4. ──▶│Provider  │          │
                    │        │.chat()   │          │
                    │        └────┬─────┘          │
                    │             │                │
                    │        LLM Response          │
                    │             │                │
                    │    ┌────────▼────────┐       │
                    │    │ Has tool_calls? │       │
                    │    └───┬────────┬────┘       │
                    │    YES │        │ NO         │
                    │        │        │            │
                    │   ┌────▼────┐   │            │
                    │   │Execute  │   │            │
                    │   │ToolCall │   │            │
                    │   │(0 ovhd) │   │            │
                    │   └────┬────┘   │            │
                    │        │        │            │
                    │   Append result │            │
                    │   to session    │            │
                    │        │        │            │
                    │   ┌────▼────┐   │            │
                    │   │Iter <  │   │            │
                    │   │MAX(20)?│   │            │
                    │   └───┬────┘   │            │
                    │   YES │        │            │
                    │   loop │   ┌────▼────┐       │
                    │   back  │   │ Return  │       │
                    │         │   │Response │       │
                    │         │   └─────────┘       │
                    └─────────┴─────────────────────┘
                              │
                    AgentResponse {
                      content: String,
                      tool_calls: Vec<ToolCall>,
                      session_id: String,
                    }
```

### 5.2 MCP Plugin Protocol Flow

```
┌─────────┐          ┌──────────┐          ┌─────────────┐
│ Agent   │          │ MCP      │          │ Plugin       │
│ Loop    │          │ Client   │          │ Process      │
│         │          │ (rmcp)   │          │ (aarch64)    │
└────┬────┘          └────┬─────┘          └──────┬──────┘
     │                    │                       │
     │  ToolHandler       │                       │
     │  call (0ms) ──────▶│                       │
     │                    │                       │
     │                    │  JSON-RPC over stdio  │
     │                    │  ──────────────────▶  │
     │                    │  {                    │
     │                    │    "jsonrpc":"2.0",   │
     │                    │    "method":          │
     │                    │      "tools/call",    │
     │                    │    "params":{         │
     │                    │      "name":"search", │
     │                    │      "arguments":{...}│
     │                    │    },                 │
     │                    │    "id":1             │
     │                    │  }                    │
     │                    │                       │
     │                    │              Execute  │
     │                    │              tool     │
     │                    │                       │
     │                    │  ◀──────────────────  │
     │                    │  {"jsonrpc":"2.0",    │
     │                    │   "result":{...},     │
     │                    │   "id":1}             │
     │                    │                       │
     │  Result ◀──────────│                       │
     │  (15ms total)      │                       │
     │                    │                       │

Latency breakdown:
  ToolHandler (built-in):  0ms   (direct function call)
  MCP stdio (process):     15ms  (IPC + serialization)
  MCP SSE (network):       50ms+ (HTTP + serialization)
  Skills (SKILL.md):       50ms+ (file I/O + LLM injection)
```

### 5.3 Configuration Flow

```
AppConfig {
    model: "deepseek-v4-pro",
    api_key: "sk-...",
    api_endpoint: "https://token-plan-sgp.xiaomimimo.com/v1/chat/completions",
    temperature: 0.7,
    max_tokens: 4096,
}
        │
        ├── Load priority:
        │   1. Command-line flag (--config path/to/config.yaml)
        │   2. Environment variable (HERMES_CONFIG)
        │   3. Default path (~/.hermes/config.yaml)
        │   4. Hardcoded defaults (AppConfig::default())
        │
        ▼
Provider selection logic:
    if api_endpoint contains "anthropic" → AnthropicProvider
    if model starts with "local/" → MistralRsProvider (feature gate)
    else → GenericProvider (OpenAI-compatible)
```

---

## 6. Implementation Roadmap

### Phase 1: Core Fusion (Weeks 1-3)

**Goal**: Replace self-built components with upstream crates. Keep the AgentLoop working throughout.

| Week | Task | Files Affected | Dependencies Added |
|------|------|---------------|-------------------|
| 1.1 | Replace `mcp/client.rs` with `rmcp` crate | `mcp/client.rs` → delete, `mcp/mod.rs` | `rmcp = "0.8"` |
| 1.2 | Integrate `tinycortex` memory engine | `core/memory.rs` → replace | `tinycortex = "0.1"` |
| 1.3 | Add SmartModelRouter to `provider.rs` | `core/provider.rs` (+~200L) | — |
| 1.4 | Integrate rtk HTTP proxy support | `core/config.rs` (+proxy_url field) | — (external process) |
| 2.1 | Build-in 5 Operit_MCPS plugins as ToolHandlers | `tools/browser.rs` rewrite, `tools/vision.rs` rewrite, `tools/filesystem.rs` extend | obscura CDP bindings |
| 2.2 | Implement `tools/web.rs` (reqwest + scraper) | `tools/web.rs` (new, ~200L) | `scraper = "0.19"` |
| 2.3 | Implement `tools/terminal.rs` (shell execution) | `tools/terminal.rs` (new, ~150L) | — |
| 2.4 | Implement `tools/process.rs` (background processes) | `tools/process.rs` (new, ~200L) | — |
| 3.1 | Add `moka` cache layer to provider | `core/provider.rs` (+cache module) | `moka = "0.12"` |
| 3.2 | Integrate headroom JSON compression | `core/provider.rs` (+compression step) | `headroom` (optional) |
| 3.3 | `tantivy` FTS index for sessions | `core/memory.rs` (+index module) | `tantivy = "0.22"` |

**Deliverable**: Agent with 12+ tools, rmcp MCP client, semantic memory, token optimization.

### Phase 2: Android Integration (Weeks 4-6)

**Goal**: JNI bridge layer for Android system services. Dioxus UI with 5-tab layout.

| Week | Task | New Files |
|------|------|-----------|
| 4.1 | JNI boilerplate + shizuku bridge | `android/jni.rs`, `android/shizuku.rs` |
| 4.2 | AccessibilityService bridge | `android/accessibility.rs` |
| 4.3 | ForegroundService anti-kill | `android/foreground.rs` |
| 4.4 | Notification + clipboard bridges | `android/notification.rs`, `android/clipboard.rs` |
| 5.1 | Dioxus UI shell + tab navigation | `ui/app.rs`, `ui/tab_bar.rs` |
| 5.2 | Chat view with message bubbles | `ui/chat.rs` rewrite |
| 5.3 | Market view (3 sub-tabs) | `ui/market.rs`, `ui/store_page.rs` rewrite |
| 5.4 | Toolbox view (9 tools) | `ui/toolbox.rs` (new) |
| 6.1 | Settings view (25+ pages) | `ui/settings.rs` rewrite |
| 6.2 | Memory view | `ui/memory.rs` (new) |
| 6.3 | Integration testing | `tests/android_integration.rs` |
| 6.4 | APK build configuration | `Cargo.toml` (dioxus-mobile), `build.rs` |

**Deliverable**: Working Android APK with chat, Shizuku system privileges, voice input, 5-tab UI.

### Phase 3: Capability Expansion (Weeks 7-9)

**Goal**: Sandbox, browser, voice, local inference.

| Week | Task |
|------|------|
| 7.1 | Replace sandbox placeholder with wasmer | `environment/sandbox.rs` → wasmer integration |
| 7.2 | obscura CDP real implementation | `tools/browser.rs` → CDP protocol |
| 7.3 | sherpa-onnx ASR integration | `tools/transcription.rs` (new) |
| 7.4 | sherpa-onnx TTS integration | `tools/tts.rs` (new) |
| 8.1 | mistral.rs feat flag + Android build | `Cargo.toml` feature, `provider/mistral.rs` |
| 8.2 | Voice wake word detection | `android/foreground.rs` (+wake module) |
| 8.3 | SubAgentOrchestrator implementation | `core/agent.rs` (+orchestrator module) |
| 9.1 | Plugin store download/install flow | `store/` implementation |
| 9.2 | GitHub OAuth login | `ui/login.rs` implementation |
| 9.3 | Performance profiling & optimization | `benches/`, `cargo flamegraph` |

### Phase 4: Ecosystem & Release (Weeks 10-12)

| Week | Task |
|------|------|
| 10.1 | Plugin store CI/CD (GitHub Actions) |
| 10.2 | Community MCP plugin compatibility testing |
| 11.1 | Documentation: ARCHITECTURE.md, API.md, CONTRIBUTING.md |
| 11.2 | APK signing + Google Play / GitHub Releases |
| 11.3 | Integration tests + CI pipeline |
| 12.1 | Performance benchmarks vs thClaws / goose |
| 12.2 | Blog post / announcement |

---

## 7. Build & Deployment

### 7.1 Development Build (Windows, current)

```bash
# Build (without UI, without sandbox)
cd D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App
cargo build --release

# Run CLI mode
./target/release/hermes_operit_app.exe
```

### 7.2 Android Build

```bash
# Prerequisites
rustup target add aarch64-linux-android
cargo install cargo-ndk

# Build APK
cargo ndk -t aarch64-linux-android -o ../app/src/main/jniLibs build --release

# Package with Gradle (requires Android Studio)
cd android/
./gradlew assembleRelease
```

### 7.3 MCP Plugin Build (for plugin store)

```bash
# Cross-compile for Android (aarch64-musl, fully static)
cargo install cross
cross build --target aarch64-unknown-linux-musl --release

# Package
python build/pack.py <plugin_name>
# → output/<plugin_name>.zip (binary + index.js + package.json)
```

### 7.4 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-linux-android
      - run: cargo build --target aarch64-linux-android --release

  build-plugins:
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        plugin: [obscura, agentic_vision, filesystem, typemill, sherpa]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cross
      - run: cross build --target aarch64-unknown-linux-musl --release -p ${{ matrix.plugin }}
```

### 7.5 Dependency Map

```
[dependencies]
# ── Async Runtime ──
tokio = { version = "1", features = ["full"] }

# ── Serialization ──
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"

# ── Code Analysis ──
ua-core = { path = "../Understand_Anything_Rust/crates/ua-core" }  # ✅ existing

# ── HTTP ──
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }

# ── Database ──
redb = "1"                 # ✅ existing (KV store)

# ── MCP Protocol ──
rmcp = "0.8"               # 🔜 phase 1 (replaces mcp/client.rs)

# ── Memory ──
tinycortex = "0.1"         # 🔜 phase 1 (replaces memory.rs)
tantivy = "0.22"           # 🔜 phase 1 (FTS index)

# ── Cache ──
moka = { version = "0.12", features = ["future"] }  # 🔜 phase 1

# ── Web Search ──
scraper = "0.19"           # 🔜 phase 1 (tools/web.rs)

# ── Android Bridge ──
jni = "0.21"               # 🔜 phase 2 (android/*)

# ── UI ──
dioxus = { version = "0.5", features = ["mobile"] }  # 🔜 phase 2 (uncomment)
dioxus-mobile = "0.5"      # 🔜 phase 2

# ── Sandbox ──
wasmer = "4"               # 🔜 phase 3

# ── Local Inference ──
mistralrs = { version = "0.3", optional = true }  # 🔜 phase 3 (feature gate)

# ── Voice ──
# sherpa-rs depends on prebuilt .so, loaded at runtime on Android

# ── Utility ──
parking_lot = "0.12"
once_cell = "1"
thiserror = "1"
anyhow = "1"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
env_logger = "0.11"
tracing = "0.1"
tracing-subscriber = "0.3"
walkdir = "2"
flate2 = "1"
regex = "1"
directories = "5"
oauth2 = { version = "4.4", features = ["reqwest"] }
```

---

## 8. Competitive Analysis

### Feature Matrix

| Feature | Hermes_Rust | thClaws | goose | claude-code-rust | ZeptoClaw |
|---------|-------------|---------|-------|------------------|-----------|
| **Language** | Rust | Rust | Rust | Rust + TS | Rust |
| **Android** | ✅ **Native** | ❌ | ❌ | ❌ | ❌ |
| **Desktop** | ⚪ Secondary | ✅ Tauri | ✅ Tauri | ✅ Tauri | ⚪ |
| **Agent Loop** | ✅ ReAct | ✅ Custom | ✅ | ✅ | ✅ |
| **Sub-agents** | ✅ Orchestrator | ✅ Teams | ❌ | ❌ | ❌ |
| **Tools** | ✅ 35 target | ✅ | ✅ | ✅ | ✅ |
| **MCP** | ✅ rmcp | ✅ | ✅ 70+ | ✅ | ✅ |
| **Skills** | ✅ SKILL.md | ✅ | ❌ | ✅ | ✅ |
| **System Privilege** | ✅ Shizuku | ❌ | ❌ | ❌ | ❌ |
| **Accessibility** | ✅ A11yService | ❌ | ❌ | ❌ | ❌ |
| **Voice** | ✅ sherpa-onnx | ❌ | ❌ | ❌ | ❌ |
| **Local Inference** | ✅ mistral.rs | ❌ | ❌ | ❌ | ❌ |
| **Sandbox** | ✅ wasmer | ❌ | ❌ | ❌ | ✅ 7-layer |
| **Token Optimization** | ✅ rtk+headroom | ❌ | ❌ | ❌ | ❌ |
| **Plugin Store** | ✅ 3-tab market | ❌ | ❌ | ❌ | ❌ |
| **OAuth** | ✅ GitHub | ❌ | ❌ | ❌ | ✅ |
| **Cron** | ✅ | ❌ | ❌ | ❌ | ✅ |
| **GitHub Stars** | — (new) | 1,166 | LF project | 1,667 | 644 |

### Differentiation Strategy

1. **Android-native is the core moat** — thClaws, goose, and claude-code-rust all target desktop. Hermes_Rust is the only Rust AI agent targeting Android with system-level integration.

2. **System privilege via Shizuku** — enables AI to directly interact with other apps (tap, swipe, type), read screens, install packages. Competitors can't do this on desktop either.

3. **Voice as first-class input** — sherpa-onnx for fully local ASR/TTS. Combined with Shizuku's microphone foreground service, this enables always-on voice agents.

4. **Plugin marketplace** — 3-tab store (Artifacts/Skills/MCP) creates an ecosystem moat. Operit_MCPS provides the initial plugin catalog.

5. **Cost optimization built-in** — rtk (60-90% token savings) + headroom (60-95% JSON compression) + moka cache. This matters for mobile users on metered connections.

---

## 9. Risk & Mitigation

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| hermes-agent-rs API instability | Medium | High | Phase 1 keeps self-implemented AgentLoop; only replace if API stabilizes |
| Dioxus Android rendering bugs | Medium | Medium | Phase 2 tests on real devices early; fallback to WebView if needed |
| Shizuku permission rejection | Low | High | Document Shizuku setup clearly; AccessibilityService as fallback |
| mistral.rs OOM on Android | Medium | Low | Use 1.5B models (Q4, ~1GB); aggressive context window limits |
| wasmer Android NDK issues | Medium | Medium | wasmer officially supports Android; test early in Phase 3 |
| rmcp API changes | Low | Low | Pin version; rmcp is official MCP SDK, semver-stable |
| Plugin store security | Medium | High | sha256 verification; permission model; sandboxed execution |

---

## Appendix A: Source File Inventory

```
Hermes_Rust_Operit_App/
├── Cargo.toml                    — crate manifest, dependencies, features
├── ARCHITECTURE.md               — this document
├── SESSION_HANDOFF.md            — flash→pro handoff log
├── Learning/                     — 63 learning reports (28 ★★★★★ + 22 ★★★★ + 13 others)
├── src/
│   ├── main.rs              [169L] — CLI entry point, REPL loop
│   ├── lib.rs                [10L] — module re-exports
│   ├── core/
│   │   ├── mod.rs            [14L] — module + type re-exports
│   │   ├── agent.rs         [238L] — AgentManager, agent_loop, tool execution
│   │   ├── provider.rs      [386L] — GenericProvider, AnthropicProvider, LlmProvider trait
│   │   ├── tool_registry.rs  [77L] — ToolHandler trait, ToolRegistry, ToolSchema
│   │   ├── config.rs         [75L] — AppConfig, YAML/JSON loading
│   │   └── memory.rs               — Session message storage (HashMap)
│   ├── tools/
│   │   ├── mod.rs                 — Module re-exports
│   │   ├── filesystem.rs    [367L] — File I/O (8 operations), path validation
│   │   ├── markdown.rs      [321L] — Markdown render, code extraction, formatting strip
│   │   ├── browser.rs       [109L] — Browser automation placeholder
│   │   ├── vision.rs              — Image analysis placeholder
│   │   └── codebase_analyzer.rs    — UA Rust integration
│   ├── mcp/
│   │   ├── mod.rs                 — Module definition
│   │   └── client.rs        [~400L] — Custom JSON-RPC MCP client
│   ├── environment/
│   │   ├── mod.rs                 — Module definition
│   │   └── sandbox.rs        [71L] — Sandbox placeholder
│   ├── store/
│   │   └── mod.rs                 — Plugin store module
│   └── ui/
│       ├── mod.rs                 — Module definition
│       ├── chat.rs                — Chat interface
│       ├── login.rs               — OAuth login
│       ├── settings.rs            — Settings manager
│       └── store_page.rs          — Store browser page
```

## Appendix B: Key Research Files Referenced

```
Learning/01-★★★★★-hermes-agent-rs-Rust Agent 引擎.md          → Agent loop architecture
Learning/03-★★★★★-rtk-LLM Token优化代理.md                      → Token optimization
Learning/10-★★★★★-mistral.rs-纯Rust本地LLM推理.md                → Local inference
Learning/11-★★★★★-obscura-无头浏览器.md                          → Browser tool
Learning/12-★★★★★-openhuman-记忆系统.md                           → Memory engine
Learning/13-★★★★★-Operit_MCPS-MCP插件集.md                       → Plugin catalogue
Learning/14-★★★★★-Operit-Android宿主平台.md                       → Android shell
Learning/15-★★★★★-RMCP-官方MCP-Rust-SDK.md                      → MCP protocol
Learning/16-★★★★★-Rust-网页搜索引擎.md                            → Web search
Learning/17-★★★★★-sherpa-onnx-语音引擎.md                         → Voice engine
Learning/18-★★★★★-Understand_Anything_Rust-代码分析引擎.md         → UA Rust
Learning/19-★★★★★-wasmer-沙盒执行.md                              → Sandbox
Learning/20-★★★★★-Dioxus-RustUI框架.md                            → UI framework
Learning/44-★★★★★-HermesApp-UI交互源码分析.md                      → UI layout
Learning/52-★★★★★-thClaws-功能重合RustAgent.md                     → Competitor ref
Learning/59-★★★★★-goose-RustAI-Agent重合分析.md                    → Competitor ref
Learning/60-★★★★★-claude-code-rust源码分析.md                      → Competitor ref
Learning/63-★★★★-ZeptoClaw-轻量RustAgent源码分析.md                → Module design ref
Learning/65-★★★★★-AIAgent全生态聚合仓库.md                         → Patterns reference
Learning/66-★★★★★-MCP生态-headroom源码分析.md                      → Token optimization

UA Output/md/01-hermes-agent-rs.md    → 700 nodes, 694 edges, 4 layers
UA Output/md/05-operit-mcps.md        → 1,505 nodes, 1,556 edges, 5 layers
UA Output/md/thclaws-full.md          → 10 nodes, 289-line Cargo.toml
UA Output/md/claude-code-rust.md      → 902-line main.rs, 54-line lib.rs
UA Output/md/zeptoclaw-lite.md        → 100-line lib.rs, 25+ modules
```

---

> **Next Action**: Phase 1, Week 1.1 — `cargo add rmcp` and replace `mcp/client.rs`.  
> **Status**: Architecture document complete. Ready for implementation.
