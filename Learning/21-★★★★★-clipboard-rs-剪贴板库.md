# clipboard-rs — Rust 剪贴板库 UA 源码分析

> **UA Rust 分析**：17 nodes / 2 源文件解析  
> **仓库**：https://github.com/ChurchTao/clipboard-rs  
> **Hermes_Rust_Operit_App 评分**：★★★★★

---

## 一、UA Rust 发现

```
clipboard-rs (17 nodes)
├── lib.rs (3.3KB) — 入口，重导出
├── common/        — 公共类型
├── platform/      — 平台实现（Windows/macOS/Linux）
└── image/         — 图片支持
```

## 二、核心 API

```rust
use clipboard_rs::{Clipboard, ClipboardContext};

pub trait Clipboard: Send {
    fn get_text(&self) -> Result<String>;        // 读剪贴板文本
    fn set_text(&self, text: &str) -> Result<()>; // 写剪贴板文本
    fn get_image(&self) -> Result<RustImageData>;  // 读图片
    fn set_image(&self, image: &RustImageData);    // 写图片
}
```

## 三、Android 端方案

clipboard-rs **不支持 Android**。需要通过 JNI 调用 Android API：

```rust
// JNI 桥接 Android ClipboardManager
fn android_get_clipboard(env: &mut JNIEnv) -> String {
    let clipboard = env.find_class("android/content/ClipboardManager")?;
    // clipboard.getPrimaryClip() → getItemAt(0) → getText()
}

fn android_set_clipboard(env: &mut JNIEnv, text: &str) {
    let clipboard = env.find_class("android/content/ClipboardManager")?;
    // clipboard.setPrimaryClip(ClipData.newPlainText(label, text))
}
```

## 四、Hermes 中的使用

| 场景 | 方法 | 优先级 |
|------|------|--------|
| Agent 读取用户复制的内容 | `clipboard.get_text()` | ★★★★★ |
| Agent 粘贴结果到其他 App | `clipboard.set_text(result)` | ★★★★★ |
| Agent 复制代码/文本 | `clipboard.set_text(code)` | ★★★★ |

### 评分：★★★★★

clipboard-rs 是 Agent 跨 App 交互的关键能力。桌面端直接使用，Android 端需 JNI 桥接。
