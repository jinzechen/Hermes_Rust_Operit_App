# Android Rust 生态 — 关键词驱动学习报告

> **从报告中提取的关键词**：Shizuku 权限 / 无障碍服务 / 前台服务 / 通知  
> **搜索策略**：GitHub 搜 "rust android <keyword>"

---

## 一、Rust Android 基础设施

### 1. jni-rs（标准 Rust JNI crate）

```
crates.io: jni
用途: Rust ↔ Java/Kotlin 互操作
版本: 0.21
这是 Hermes_Rust_Operit_App 访问 Android API 的唯一通道
```

### 2. willir/cargo-ndk-android-gradle (70⭐)

```
用途: Android Gradle 插件，自动调用 cargo-ndk 编译 Rust
集成: build.gradle.kts 中配置即可
Hermes 可用: ★★★★ 简化 Android 构建配置
```

### 3. rustdesk (118K⭐) — Android 构建参考

```
cargo ndk --platform 21 --target aarch64-linux-android build --release
→ liblibrustdesk.so → jniLibs/arm64-v8a/
→ Flutter/Gradle 打包 APK
```

---

## 二、能力→Rust 项目映射

| 能力 | Rust 方案 | 状态 | 优先级 |
|------|----------|------|--------|
| **Shizuku 系统权限** | jni-rs 调用 Shizuku SDK | 需自行桥接 | ★★★★★ |
| **无障碍服务** | `AccessibilityService` via JNI | 需自行实现 | ★★★★★ |
| **前台服务** | `ForegroundService` via JNI | 需自行实现 | ★★★★ |
| **通知** | `NotificationManager` via JNI | 需自行实现 | ★★★★ |
| **Termux 通道** | Termux:Terminal via JNI | 参考 Operit | ★★★★ |
| **文件访问** | hermes-tools file.rs | ✅ 已有 | ★★★ |
| **语音 TTS** | sherpa-onnx ✅ | ✅ 已分析 | ★★★★★ |
| **向量搜索** | qdrant-client ✅ | ✅ 已分析 | ★★★★ |

---

## 三、关键结论

**Hermes_Rust_Operit_App 的 Android 桥接层需要自己实现以下 JNI 桥接：**

```
android-bridge/
├── jni.rs           → jni-rs 初始化
├── accessibility.rs → 无障碍服务（屏幕读取+点击）
├── shizuku.rs       → Shizuku 系统权限
├── foreground.rs    → 前台服务
├── notification.rs  → 通知
├── termux.rs        → Termux 通道
└── filesystem.rs    → Android 文件系统（SAF）
```

每个桥接约 200-500 行 Rust 代码 + 对应的 Kotlin 胶水层。

### Rust 复刻总结

完整的 Android 桥接层目录结构：

```
android-bridge/
├── Cargo.toml            ← jni = "0.21"
├── src/
│   ├── lib.rs            ← JNI 函数导出宏
│   ├── accessibility.rs  ← 屏幕读取+点击
│   ├── shizuku.rs        ← 系统权限
│   ├── foreground.rs     ← 后台保活
│   ├── notification.rs   ← 通知
│   ├── termux.rs         ← Ubuntu 通道
│   └── filesystem.rs     ← SAF 文件系统
└── kotlin/
    └── Bridge.kt         ← Kotlin 侧 JNI 声明
```
