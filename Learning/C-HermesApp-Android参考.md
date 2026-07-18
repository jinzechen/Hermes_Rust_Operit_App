# HermesApp — Android Hermes 参考实现（三大源项目之三）

> **项目**：https://github.com/SelectXn00b/HermesApp (194⭐, Kotlin)  
> **架构口号**：**"Hermes 内核 + Operit 壳"**  
> **Hermes_Rust_Operit_App 的关系**：**Kotlin 版的等效 Rust 重写**

---

## 一、项目定位

HermesApp 就是将 [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) (Python, 216k⭐) 的 Agent Loop **1:1 翻译成 Kotlin**，作为内核，跑在 Operit 的 Android 应用壳里。

**Hermes_Rust_Operit_App 做的事情完全一样，只是用 Rust 而不是 Kotlin。**

```
          HermesApp (Kotlin)               Hermes_Rust_Operit_App (Rust)
          ────────────────                 ─────────────────────────
Agent     Kotlin 1:1 翻译 Python Agent     Rust 重写 Agent 引擎
壳        Operit Android 壳                Operit Android 壳 (MCP 插件)
Agent 源  NousResearch/hermes-agent        Lumio-Research/hermes-agent-rs
```

---

## 二、技术架构

```
HermesApp (Kotlin)
│
├── app/                  → Android 主应用
├── hermes-android/       → Hermes 内核 Kotlin 翻译
├── llama/                → llama.cpp 本地推理
├── mnn/                  → MNN 本地推理
├── quickjs/              → JavaScript 引擎
├── terminal/             → 终端模拟器
├── web-chat/             → Web 聊天
├── showerclient/         → 投屏
├── tools/                → 工具定义
└── docs/                 → 文档
```

### HermesApp 核心能力

| 能力 | 描述 |
|------|------|
| **屏幕操控** | 无障碍服务→看 App→点 App |
| **跨应用联动** | 微信收地址→高德导航 |
| **代码执行** | Termux SSH + 内置 Shell |
| **搜索+抓取** | Brave 搜索 + 全文抓取 |
| **多平台消息** | 飞书、Discord 远程控制 |
| **MCP Server** | 将手机无障碍暴露给外部 Agent |
| **技能扩展** | ClawHub 市场安装新能力 |

---

## 三、三项目关系总览

```
hermes-agent-rs (Rust 引擎) ←→ Hermes_Rust_Operit_App (Rust 整合) ←→ Operit (Android 壳)
        ↑                                                           ↑
        └── 提供 18 crates / 352 Rust 文件                          └── 提供 9 MCP 插件 / 宿主环境
                                                                    ↑
                                                              HermesApp (Kotlin 参考)
                                                              └── 提供功能清单和实现参考
```

### 代码量对比

| 项目 | 语言 | 代码量 | Hermes_Rust 覆盖率 |
|------|------|--------|-------------------|
| **hermes-agent-rs** | Rust | **~100K 行** | ~5% (仅 24 Rust 文件) |
| **Operit** | Kotlin | 大型 | ~10% (仅 9 个 MCP 插件) |
| **HermesApp** | Kotlin | 大型 | ~10% (基本功能对标) |
| **Hermes_Rust_Operit_App** | Rust | **~5K 行** | 100% (基准) |

---

## 四、Hermes_Rust_Operit_App 的整合路线

```
Phase 1 (Current):    自建 Rust 引擎 + Operit_MCPS 插件
Phase 2 (Next):       引入 hermes-agent-rs crate 替换核心
Phase 3 (Target):     Rust 引擎 → MCP 插件 → Operit Android 壳
                      (HermesApp 的 Kotlin→Rust 等价)
```

### 下一步行动优先级

1. **最高** — 引入 `hermes-core` (核心类型/trait) 统一类型系统
2. **最高** — 引入 `hermes-agent` (agent_loop) 替换简单 `agent.rs`
3. **高** — 引入 `hermes-mcp` 替换自建 MCP 客户端
4. **中** — 利用 `hermes-tools` 的工具注册和 dispatch
5. **中** — 基于 Operit_MCPS 插件增强 Hermes 工具集
