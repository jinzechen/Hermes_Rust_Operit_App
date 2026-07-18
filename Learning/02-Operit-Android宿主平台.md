# 02 — Operit / HermesApp：Android Kotlin 壳源码级分析

> **Operit**：https://github.com/AAswordman/Operit (5.8K⭐, Kotlin)  
> **HermesApp**：https://github.com/SelectXn00b/HermesApp (194⭐, Kotlin)  
> **HermesApp = "Hermes 内核(Kotlin) + Operit 壳(Android)"**

---

## 一、HermesApp 的 Android 依赖（从 build.gradle 提取）

```
HermesApp (Android App)
├── Compose UI (Jetpack)
├── Shizuku API → 系统级权限（ADB 级别）
├── Xposed API → 框架钩子
├── ObjectBox → 本地数据库
├── Kotlin Serialization
└── Aliyun Maven → 国内镜像
```

### Shizuku 的作用（关键）

HermesApp 使用 Shizuku 获取**系统级权限**（无需 root）：

```kotlin
// 通过 Shizuku 执行系统命令
Shizuku.newProcess(arrayOf("input", "tap", "500", "500"), null, null)
// → 模拟屏幕点击（无障碍服务的高级替代）
```

这比无障碍服务更强大——可以执行 ADB 级别的操作。

---

## 二、Android 特定功能分析

| 功能 | 在 HermesApp 中的实现 | Rust 替代方案 |
|------|---------------------|--------------|
| **无障碍服务** | Android AccessibilityService | JNI 桥接 |
| **Shizuku 系统权限** | Shizuku API | JNI + Shizuku |
| **屏幕点击** | `input tap x y` via Shizuku | JNI 桥接 |
| **前台服务** | AIForegroundService | JNI 桥接 |
| **通知** | Android Notification | JNI 桥接 |
| **文件绑定** | FileBindingService | hermes-tools file.rs |
| **本地数据库** | ObjectBox | redb |

---

## 三、对 Hermes_Rust_Operit_App 的作用

| 能力 | 当前 Hermes_Rust | HermesApp 的参考 | 优先级 |
|------|-----------------|-----------------|--------|
| Shizuku 权限 | ❌ 无 | 系统级操作 | ★★★★★ |
| 无障碍 | ❌ 无 | 屏幕读取+点击 | ★★★★★ |
| 前台服务 | ❌ 无 | 后台运行 | ★★★★ |
| 通知 | ❌ 无 | 用户交互 | ★★★★ |
| 文件绑定 | ✅ filesystem.rs | 参考设计 | ★★★ |

### 评分：★★★★★

Operit/HermesApp 提供的 Android 特定功能（Shizuku、无障碍、前台服务）是 Hermes_Rust_Operit_App 需要实现的 Android 桥接层的完整功能清单。
