# HermesApp Shizuku 系统权限 — 源码学习

> **文件**：ShizukuInstaller.kt (12KB) + ShizukuAuthorizer.kt (16KB) + native-lib.cpp (7.8KB)  
> **Shizuku**：Android 系统级权限框架（无需 root，通过 ADB 级别授权）

---

## 一、Shizuku 的作用

Shizuku 让 Android 应用获取**系统级权限**（ADB 级别），无需 root：

| 能力 | 无障碍服务 | Shizuku |
|------|-----------|---------|
| 屏幕点击 | ✅ 有限制 | ✅ 任意位置 |
| 系统命令 | ❌ | ✅ `su` 级别 |
| SELinux 上下文 | ❌ | ✅ 可修改 |
| 截图 | ✅ 需用户确认 | ✅ 直接截取 |
| 输入法注入 | ✅ | ✅ 原生输入 |

---

## 二、HermesApp 的 Shizuku 实现

### ShizukuInstaller.kt

```kotlin
class ShizukuInstaller {
    companion object {
        // 从 assets 提取内置 Shizuku APK
        fun extractApkFromAssets(context: Context): File?
        
        // 检查是否已提取
        fun isApkExtracted(context: Context): Boolean
        
        // 安装内置 Shizuku
        fun installBundledShizuku(context: Context): Boolean
        
        // 获取内置和已安装版本号
        fun getBundledShizukuVersion(context: Context): String
        fun getInstalledShizukuVersion(context: Context): String?
        
        // 检查是否需要更新
        fun isShizukuUpdateNeeded(context: Context): Boolean
    }
}
```

### ShizukuAuthorizer.kt

```kotlin
class ShizukuAuthorizer {
    companion object {
        // 状态变更监听
        fun addStateChangeListener(listener: () -> Unit)
        fun removeStateChangeListener(listener: () -> Unit)
        
        // 检测 Sui 后端（另一种 root 方案）
        private fun isSuiBackendAvailable(): Boolean
        
        // 检查 Shizuku 是否已安装
        fun isShizukuInstalled(context: Context): Boolean
        
        // 获取服务/权限错误信息
        fun getServiceErrorMessage(): String
        fun getPermissionErrorMessage(): String
    }
}
```

### native-lib.cpp（C++ SELinux 桥接）

```cpp
// SELinux 上下文操作（需系统权限）
typedef int setcon_t(const char *ctx);       // 设置进程上下文
typedef int setfilecon_t(const char *path, const char *ctx); // 设置文件上下文
typedef int selinux_check_access_t(...);      // 检查 SELinux 权限
```

---

## 三、对 Hermes_Rust_Operit_App 的参考价值

Hermes_Rust_Operit_App 需要 Shizuku 来实现：

| 功能 | Shizuku 命令 | 用途 |
|------|-------------|------|
| 屏幕点击 | `input tap x y` | Agent 操作 App |
| 滑动 | `input swipe x1 y1 x2 y2` | 滚动/滑动 |
| 文本输入 | `input text "..."` | 填入表单 |
| 按键 | `input keyevent KEYCODE_ENTER` | 键盘操作 |
| 截图 | `screencap` | 视觉理解 |
| 获取当前 App | `dumpsys window` | 上下文感知 |

### 评分：★★★★★

Shizuku 是 Hermes_Rust_Operit_App 实现"操控手机"能力的核心。HermesApp 的 Kotlin 实现可直接作为 Rust JNI 桥接的参考。
