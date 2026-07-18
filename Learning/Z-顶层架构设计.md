# Hermes_Rust_Operit_App — 顶层架构设计

> **定位**：纯 Rust 复刻 Operit + HermesApp 全部功能，并增强  
> **公式**：`Hermes内核(hermes-agent-rs) + Operit壳(Android) + Dioxus UI(纯Rust)`  
> **三不原则**：不重复造轮子、性能为王、生态优先

---

## 一、系统分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Dioxus UI Layer                          │
│  ChatView | StoreView | Settings | Auth | 4-Tab Dashboard   │
│  (纯 Rust 声明式 UI, dx serve --platform android)            │
├─────────────────────────────────────────────────────────────┤
│                    Android 桥接层 (JNI)                      │
│  无障碍服务 | Termux通道 | 通知 | 文件系统 | 权限管理          │
├─────────────────────────────────────────────────────────────┤
│                    Service Layer                             │
│  PluginManager | AuthService | SandboxEngine | SearchEngine  │
├─────────────────────────────────────────────────────────────┤
│                    Core Layer                                │
│  ┌─────────────┐  ┌──────────┐  ┌───────────────────────┐  │
│  │ hermes-agent │  │hermes-mcp│  │   hermes-tools        │  │
│  │ Agent Loop   │  │ MCP协议  │  │   69 files / 23K 行   │  │
│  │ 24.6K/39文件 │  │ 客户端   │  │   tools + dispatch    │  │
│  └─────────────┘  └──────────┘  └───────────────────────┘  │
│  ┌─────────────┐  ┌──────────┐  ┌───────────────────────┐  │
│  │ hermes-core  │  │hermes-   │  │   hermes-skills       │  │
│  │ 类型/trait   │  │config    │  │   技能系统             │  │
│  └─────────────┘  └──────────┘  └───────────────────────┘  │
├─────────────────────────────────────────────────────────────┤
│                    Storage Layer                              │
│  redb (KV) | SQLite (结构化) | qdrant-client (向量)          │
└─────────────────────────────────────────────────────────────┘
```

---

## 二、四 Tab 商店架构

源自 Operit 现有四板块 + 用户增强需求：

### Tab 1: 沙盒 (Sandbox)
```
安全代码执行环境
├── 内置: hermes-agent-rs code_execution.rs  ✅ (已在)
├── 扩展: Termux Ubuntu 24 通道 (Operit已有)
├── 限制: seccomp/landlock 系统调用过滤
└── 来源: 复用 Operit 已有方案
```

### Tab 2: Skills
```
技能市场
├── 标准: SKILL.md 格式 (Operit兼容)
├── 源: GitHub 聚合 (Awesome-MCP-ZH 等)
├── 内置: hermes-agent-rs skill_orchestrator
└── 安装: PluginManager::install_from_github()
```

### Tab 3: MCPs
```
MCP 服务器市场
├── 标准: MCP JSON-RPC 2.0 协议
├── 聚合: rmcp-mux (多路复用器)
├── 预装: Operit_MCPS 9 插件
└── 内置化评估: 见文件 E
```

### Tab 4: 我的 (Profile)
```
用户中心
├── GitHub OAuth (自实现, 解决404)
├── 已安装插件管理
├── 自定义源管理
└── 备份/恢复
```

---

## 三、Operit_MCPS 9 插件内置化路径

| MCP | 决定 | 技术方案 |
|-----|------|----------|
| **obscura** (浏览器) | ✅ 内置 ToolHandler | 复用 Hermes `browser.rs` |
| **agentic_vision** (视觉) | ✅ 内置 ToolHandler | 复用 Hermes `vision.rs` |
| **rust_mcp_filesystem** (文件) | ✅ 内置 ToolHandler | 复用 Hermes `filesystem.rs` |
| **typemill** (Markdown) | ✅ 内置 ToolHandler | 复用 Hermes `markdown.rs` |
| **sherpa** (语音) | ✅ 内置 ToolHandler | 集成 sherpa-onnx |
| **rust_mcp_server** | ⚠️ 保留 MCP | 太重，按需加载 |
| **mcp_proxy** | ⚠️ 保留 MCP | 外部连接用 |
| **m3ux** (音频) | ⚠️ 保留 MCP | 低频使用 |
| **rust_docs_mcp** | ⚠️ 保留 MCP | 按需查询 |

**内置化原则**：
- ToolHandler (0 开销) > MCP Client (15ms) > Skills (50ms)
- 高频/低延迟→内置；低频/重量→保留 MCP

---

## 四、三大痛点解决方案

| 痛点 | 原因 | 方案 |
|------|------|------|
| **GitHub 登录 404** | 第三方 OAuth 回调失效 | Rust 自实现 oauth2 crate |
| **文字无法复制** | CSS user-select 禁用 | Dioxus 默认支持文本选择 |
| **商店老旧无源码** | 闭源组件不更新 | 四 Tab 全开源 + GitHub 源 |

---

## 五、依赖清单（零重复造轮子）

| 类别 | crate/项目 | 来源 |
|------|-----------|------|
| UI | dioxus (36k⭐) | github.com/DioxusLabs/dioxus |
| 内核 | hermes-agent-rs (18 crates) | Lumio-Research/hermes-agent-rs |
| MCP | hermes-mcp | 同上 |
| 浏览器 | obscura (19k⭐) | h4ckf0r0day/obscura |
| 视觉 | agentic-vision | agentralabs/agentic-vision |
| 语音 | sherpa-onnx (13k⭐) | k2-fsa/sherpa-onnx |
| 向量 | qdrant-client (33k⭐) | qdrant/qdrant |
| 本地模型 | candle (20k⭐) | huggingface/candle |
| OAuth | oauth2 crate | crates.io |
| 文件系统 | rust-mcp-filesystem | rust-mcp-stack/rust-mcp-filesystem |
| Markdown | comrak / typemill | crates.io / jinzechen/Operit_MCPS |

---

## 六、学习任务继续

基于顶层设计，接下来需深入学习的知识点：

```
继续 SESSION_HANDOFF.md 任务 24-27（已完成）
下一步：
  F-Android 无障碍服务 Rust 桥接方案
  G-Termux Ubuntu 环境集成方案
  H-Dioxus 移动端实战指南
  I-Sandbox 安全执行环境设计
```

是否需要我继续写 F-I 的学习报告？
