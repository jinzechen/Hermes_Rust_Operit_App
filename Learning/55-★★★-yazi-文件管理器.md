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

Hermes 已有 `filesystem.rs` (12.5KB)。

### Rust 复刻总结

yazi 的异步 I/O、Lua 插件系统设计可参考但不直接使用。Hermes 的 file.rs 已满足需求：

```rust
// Hermes 现有文件操作（已覆盖 yazi 核心能力）
tools/file.rs — 读/写/搜索/复制/移动/删除
```

### 评分：★★★
