# awesome-rust 系统工具 — Agent 需要的系统级能力

> **从 awesome-rust (58K⭐) Applications/System tools 提取**  
> **目标**：覆盖 Hermes_Rust_Operit_App 需要的所有系统级能力

---

## 一、Hermes Agent 的系统级能力需求

Agent 操控 Android 手机需要一系列系统工具。以下是 awesome-rust 中对应的 Rust 项目：

| Agent 需要的能力 | Rust 项目 | Stars | 评分 |
|-----------------|----------|-------|------|
| **剪贴板读/写** | clipboard-rs | — | ★★★★★ |
| **进程列表** | procs | 5K+⭐ | ★★★★ |
| **磁盘/存储** | dust | 9K+⭐ | ★★★★ |
| **网络监控** | bandwhich | 10K+⭐ | ★★★★ |
| **系统监控** | bottom | 10K+⭐ | ★★★★ |
| **Git 操作** | gitui | 18K+⭐ | ★★★★ |
| **多进程管理** | mprocs | 3K+⭐ | ★★★★ |
| **并行执行** | rust-parallel | 1K+⭐ | ★★★★ |
| **文件搜索** | ripgrep/rg | 50K+⭐ | ★★★★ |
| **压缩/解压** | crabz | — | ★★★ |

---

## 二、关键项目分析

### 2.1 clipboard-rs（★★★★★）— 剪贴板

Hermes_Rust_Operit_App 需要剪贴板能力来：
- 读取用户复制的内容
- 粘贴 AI 生成的结果
- 跨 App 共享数据

```rust
use clipboard_rs::{Clipboard, ClipboardContext};

let ctx = ClipboardContext::new().unwrap();
let text = ctx.get_text().unwrap();  // 读
ctx.set_text(result).unwrap();       // 写
```

**Android 端**：通过 JNI 调用 Android ClipboardManager。

### 2.2 procs（★★★★）— 进程列表

```rust
// Agent 查看当前运行进程
// 对应 HermesApp 的 Toolbox → ShellExecutor
let processes = std::process::Command::new("ps")
    .arg("-A").output()?;
```

### 2.3 gitui（★★★★）— Git 操作

Agent 需要管理 GitHub 仓库（Operit 的 Artifact/Skill 管理依赖 GitHub）：

```rust
// 通过 git2 crate 或 CLI
let repo = git2::Repository::open("/path/to/repo")?;
let statuses = repo.statuses(None)?;
```

### 2.4 mprocs + rust-parallel（★★★★）— 多进程

Agent 同时运行多个子任务：

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();
set.spawn(task1());
set.spawn(task2());
while let Some(result) = set.join_next().await {
    // 处理子任务结果
}
```

---

## 三、Hermes_Rust_Operit_App 系统工具集成清单

| 能力 | Rust 方案 | 实现位置 |
|------|----------|---------|
| 📋 剪贴板 | clipboard-rs / Android JNI | android-bridge/clipboard.rs |
| 📊 进程列表 | procs / ps CLI | tools/process.rs |
| 💾 存储 | dust / df CLI | tools/disk.rs |
| 🌐 网络 | bandwhich | tools/network.rs |
| 🖥️ 系统监控 | bottom / sysinfo crate | tools/system.rs |
| 📝 Git 操作 | git2 crate | tools/git.rs |
| 📦 压缩 | zip / tar crates | tools/compress.rs |
| 🔍 搜索 | ripgrep crate | tools/search.rs |
| ⚡ 并行 | tokio JoinSet | core/executor.rs |

### 评分总览

| 项目 | Stars | 用途 | 评分 |
|------|-------|------|------|
| clipboard-rs | — | 剪贴板读写 | ★★★★★ |
| procs | 5K⭐ | 进程列表 | ★★★★ |
| dust | 9K⭐ | 磁盘使用 | ★★★★ |
| bandwhich | 10K⭐ | 网络监控 | ★★★★ |
| bottom | 10K⭐ | 系统监控 | ★★★★ |
| gitui | 18K⭐ | Git TUI | ★★★★ |
| mprocs | 3K⭐ | 多进程 TUI | ★★★★ |
| rust-parallel | 1K⭐ | 并行命令 | ★★★★ |
| ripgrep | 50K⭐ | 文件搜索 | ★★★★ |
