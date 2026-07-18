# HermesApp Shizuku 系统权限 — UA Rust 源码级学习

> **UA Rust 分析**：5 nodes / 1 edge / 2 layers / 3 源文件  
> **文件**：ShizukuInstaller.kt(12KB) + ShizukuAuthorizer.kt(16KB) + native-lib.cpp(7.8KB)  
> **Hermes_Rust_Operit_App 作用**：★★★★★ Android 系统级操作的核心权限层

---

## 一、UA Rust 分析结果

UA Rust 扫描了 3 个源文件，识别出项目结构：

```
Project: hermesapp-shizuku (5 nodes, 2 layers)
├── Layer 1 — Core Code (4 nodes)
│   ├── ShizukuInstaller.kt      — Shizuku 安装/更新管理
│   ├── ShizukuAuthorizer.kt     — Shizuku 授权/状态管理
│   └── native-lib.cpp           — SELinux 上下文操作
└── Layer 2 — Configuration (1 node)
    └── meta.json
```

UA Rust 不解析 Kotlin/C++，所以没有函数级节点，但文件结构清晰。

---

## 二、源码学习

从实际源码阅读（基于 UA Rust 发现文件后的手动分析）：

### 2.1 ShizukuInstaller.kt

```
ShizukuInstaller
├── extractApkFromAssets()      → 从 assets 提取 Shizuku APK
├── isApkExtracted()            → 检查是否已提取
├── installBundledShizuku()     → 安装内置 Shizuku
├── getBundledShizukuVersion()  → 获取内置版本号
├── getInstalledShizukuVersion() → 获取已安装版本号
└── isShizukuUpdateNeeded()     → 检查是否需要更新
```

### 2.2 ShizukuAuthorizer.kt

```
ShizukuAuthorizer
├── addStateChangeListener()    → 状态变更监听
├── removeStateChangeListener()
├── isSuiBackendAvailable()     → 检测 Sui（另一 root 方案）
├── isShizukuInstalled()        → 检查 Shizuku 是否已安装
├── getServiceErrorMessage()    → 获取服务错误信息
└── getPermissionErrorMessage() → 获取权限错误信息
```

### 2.3 native-lib.cpp

```
SELinux 上下文操作:
├── getcon()   → 获取当前 SELinux 上下文
├── setcon()   → 设置进程 SELinux 上下文
└── setfilecon() → 设置文件 SELinux 上下文
```

---

## 三、对 Hermes_Rust_Operit_App 的作用

Hermes_Rust_Operit_App 通过 Shizuku 获得**系统级权限**后，可实现：

| 能力 | 实现方式 | 用途 |
|------|---------|------|
| 屏幕点击 | `input tap x y` | AI 操控 App |
| 滑动 | `input swipe x1 y1 x2 y2` | 滚动页面 |
| 文本输入 | `input text "..."` | 自动填表 |
| 按键 | `input keyevent KEYCODE_ENTER` | 键盘操作 |
| 截图 | `screencap` | 视觉理解 |
| 包管理 | `pm install/uninstall` | 自动安装插件 |

### Rust 复刻总结

通过 jni-rs crate 调用 Shizuku API：

```rust
use jni::JNIEnv;

// 通过 Shizuku 执行 Shell 命令
fn shizuku_exec(env: &mut JNIEnv, command: &str) -> Result<String> {
    let shizuku = env.find_class("moe/shizuku/api/Shizuku")?;
    // 调用 Shizuku.newProcess(cmd)
    // → 获得系统级 Shell 权限
}

// 常用操作
shizuku_exec("input tap 500 500")?;         // 点击
shizuku_exec("input text "hello"")?;      // 输入
shizuku_exec("screencap /sdcard/screen.png")?; // 截图
shizuku_exec("dumpsys window")?;            // 获取当前窗口
```

### 评分：★★★★★
