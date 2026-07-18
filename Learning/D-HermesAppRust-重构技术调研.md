# HermesApp Rust 重构 — 技术深度调研报告

> **基于文档**：HermesApp-Rust 重构思路参考.md  
> **核心公式**：hermes-agent-rs (Rust核心) + Operit (Android壳) + Dioxus (UI)  
> **目标**：纯 Rust 重构 Android AI 助手

---

## 一、技术选型深度验证

### 1.1 Dioxus — Rust UI 框架（36,804⭐）

| 维度 | 评估 |
|------|------|
| 定位 | Fullstack 应用框架（Web/Desktop/Mobile） |
| Android 支持 | ✅ `dx serve --platform android` 一行命令 |
| 架构 | 声明式（类似 React），虚拟 DOM |
| 渲染 | Blitz（自渲染）+ WebView 备选 |
| 二进制大小 | ~5MB（release） |
| 原生能力 | 通过 JNI 桥接 Android API |
| 成熟度 | 生产可用 |

**Dioxus 在 HermesApp Rust 中的角色**：
- 对话界面（Chat View）：消息列表、输入框、Markdown 渲染
- 商店界面：四 Tab（沙盒/Skills/MCP/我的）
- 设置界面：模型配置、皮肤、语言
- 登录界面：GitHub OAuth

### 1.2 hermes-agent-rs 作为内核（已验证）

已在文件 A 中深度分析（18 crates / 352 Rust 文件 / ~100K 行）。

**需要提取的核心 crate**：

| crate | 行数 | HermesApp Rust 用途 |
|-------|------|-------------------|
| `hermes-core` | 1.5K | 类型系统 / trait 定义 |
| `hermes-agent` | 24.6K | Agent 循环 / 记忆 / 工具调用 |
| `hermes-mcp` | 3.4K | MCP 协议客户端 |
| `hermes-config` | 4.4K | 配置管理 |
| `hermes-tools` (部分) | 23K | 工具注册 / dispatch |

### 1.3 MCP 聚合方案

| 项目 | 说明 | 适合度 |
|------|------|--------|
| **rmcp-mux** | MCP 服务器 stdio 多路复用器 | ★★★★★ 直接可用 |
| **mcpdock** | MCP 服务器管理和聚合网关 | ★★★★ 桌面端方案参考 |
| **Operit_MCPS (自建)** | 9 个 MCP 插件的打包方案 | ★★★★★ 最适合 |

### 1.4 沙盒方案

hermes-agent-rs 已有代码执行沙盒：
- `hermes-tools/src/tools/code_execution.rs` — 代码执行工具
- `hermes-eval/src/verifier.rs` — 输出验证

**Android 沙盒路线**：
```
方案 A: 复用 hermes-agent-rs code_execution
方案 B: Termux 内置 Linux 环境（Operit 已有）
方案 C: seccomp/landlock 原生限制（仅 Linux）
```

---

## 二、Store 四板块设计

基于 Operit 现有设计 + 用户需求：

### 2.1 架构

```
PluginManager (Rust 核心)
├── install_from_github(url)    — 从 GitHub 安装
├── list_skills()               — 列出已安装
├── update_skill(name)          — 更新
└── remove_skill(name)          — 卸载

数据模型
┌─────────────┐
│   Source    │  ← GitHub / 自定义 JSON 索引
├─────────────┤
│   Skill     │  ← 名称 / 描述 / 版本 / 仓库
├─────────────┤
│   MCPServer │  ← 协议 / 端点 / 工具列表
├─────────────┤
│   Sandbox   │  ← 镜像 / 资源限制 / 网络策略
└─────────────┘
```

### 2.2 四 Tab 设计

```
Tab 1: 沙盒 (Sandbox)
├── 内置 Ubuntu 24（通过 Termux）
├── 代码执行环境（Python/Node/Shell）
├── 资源限制（CPU/内存/网络）
└── 沙盒模板管理

Tab 2: Skills
├── 从 GitHub 发现 Skill
├── 已安装 Skill 列表
├── 一键安装/更新
└── Skill 配置

Tab 3: MCPs
├── MCP 服务器列表（rmcp-mux 管理）
├── 从 GitHub 发现 MCP 服务器
├── 连接状态/日志
└── 工具浏览

Tab 4: 我的 (Profile)
├── GitHub 登录状态
├── 已安装插件管理
├── 自定义源管理
└── 备份/恢复
```

---

## 三、三大痛点解决方案

### 痛点 1：GitHub 登录 404

**原因**：第三方 OAuth 回调地址失效  
**方案**：在 Rust 层自实现 OAuth 流程

```rust
// src/auth/github.rs
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn get_github_client() -> BasicClient {
    BasicClient::new(
        ClientId::new("YOUR_CLIENT_ID".to_string()),
        Some(ClientSecret::new("YOUR_CLIENT_SECRET".to_string())),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new("hermesapp://callback".to_string()).unwrap())
}
```

### 痛点 2：无法复制文字

**原因**：UI 组件的 user-select CSS 被禁用  
**方案**：Dioxus 默认文本节点可选中，避免使用不可选样式

### 痛点 3：商店老旧无源码

**原因**：闭源商店组件未更新  
**方案**：四 Tab 商店全部开源，GitHub 源管理

---

## 四、构建路线图

```
Phase 1: 脚手架搭建（1周）
├── cargo new hermes-app-rs --lib
├── 添加依赖: dioxus, hermes-agent-rs, tokio, reqwest, oauth2
├── dx serve --platform android → Hello Hermes!
└── JNI 胶水层打通

Phase 2: 内核集成（2周）
├── 引入 hermes-core + hermes-agent
├── Agent 循环跑通
├── 工具注册中心
└── MCP 客户端

Phase 3: UI 开发（3周）
├── ChatView（对话界面）
├── StoreView（四 Tab 商店）
├── SettingsView（设置）
└── AuthView（GitHub 登录）

Phase 4: 商店生态（2周）
├── PluginManager
├── GitHub 源发现
├── MCP 聚合（rmcp-mux）
└── 沙盒集成

Phase 5: 发布（1周）
├── Android APK 打包
├── Operit 壳集成
├── 测试 + 修复
└── Release v1.0
```
