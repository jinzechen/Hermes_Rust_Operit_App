# 17 — yazi：终端文件管理器

> **仓库**：https://github.com/sxyazi/yazi (40,442⭐, Rust)  
> **核心价值**：异步 I/O 文件操作引擎  
> **Hermes_Rust_Operit_App 评分**：★★★（文件操作性能参考）

---

## 架构

```
yazi (Rust, 40K⭐)
├── core/       → 异步文件操作引擎
├── fns/        → Lua 插件系统
├── ui/         → ratatui TUI
└── adapters/   → 预览器/打开器
```

### 对 Hermes 的作用

Hermes 已有 `filesystem.rs` (12.5KB)，yazi 的异步 I/O 设计可参考优化性能。
