# cc-switch — AI Agent 桌面中心 UA 源码分析

> **UA Rust 分析**：lib.rs (95KB) / 2,500+ 行  
> **仓库**：https://github.com/farion1231/cc-switch (118K⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★（UI/UX 参考）

---

## 一、UA Rust 发现的源码

```
cc-switch (Tauri 桌面应用)
├── src-tauri/src/main.rs (2KB) — 入口
├── src-tauri/src/lib.rs (95KB) — ★核心
│   ├── run() — 应用启动
│   ├── handle_deeplink_url() — 深链接
│   ├── set_windows_app_user_model_id() — Windows 标识
│   ├── redact_url_for_log() — 日志脱敏
│   ├── macos_tray_icon() — Mac 托盘图标
│   ├── initialize_common_config_snippets() — 配置初始化
│   ├── show_migration_error_dialog() — 迁移错误对话框
│   ├── save_window_state_before_exit() — 窗口状态保存
│   ├── destroy_single_instance_lock() — 单实例锁
│   └── restart_process() — 进程重启
└── frontend/ — Web 前端 (React/Vue)
```

---

## 二、cc-switch 的核心能力

118K⭐ 验证了 AI Agent 桌面中心的巨大需求：

| 能力 | 实现 | Hermes 对应 |
|------|------|-----------|
| 多 Agent 管理 | 配置文件切换 | Dioxus 四 Tab |
| CLI 启动器 | Tauri command | Dioxus UI |
| 深链接 | deeplink_url | — |
| 窗口状态 | save/restore | Dioxus 路由 |
| 配置 | config snippets | hermes-config |
| 日志脱敏 | redact_url | core/redact.rs |

**UI 参考**：cc-switch 的三 Tab 布局（聊天+文件+终端）与 Operit/HermesApp 设计一致。

### 评分：★★★★

cc-switch 是 UI/UX 参考，其 118K⭐ 验证了 Agent 桌面中心的市场需求。但 Hermes 定位在 Android，UI 层面可借鉴其设计。
