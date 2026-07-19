# Hermes Rust Operit App — 项目状态与交接文档

## 项目概述

纯 Rust Android AI 助手应用，将 Hermes Agent 移植到 Android。核心逻辑（Agent、工具链、MCP）用 Rust 实现，UI 用 WebView。

**仓库**: https://github.com/jinzechen/Hermes_Rust_Operit_App
**设备**: Xiaomi 2409BRN2CY, Android 16 (SDK 36), arm64-v8a

## CI/CD

✅ CI 可通过（test + android-apk 两个 job）
- 编译 Rust 库 → `libhermes_operit_core.so` (arm64)
- 打包 APK → `target/debug/apk/HermesOperit.apk`
- 手动 DEX 编译 + jar 注入（因为 cargo-apk 不原生支持 Java 编译）
- apksigner 重新签名

## 当前问题：黑屏/白屏

### 已尝试过的方案

| 方案 | 结果 |
|---|---|
| dioxus-mobile 0.5.x | WebView 构造函数崩溃（Looper 问题） |
| ndk_glue::main + Looper.prepare | WebView 创建成功但 `CalledFromWrongThreadException` |
| 手动 ANativeActivity_onCreate | WebView 创建成功，HTML 加载成功，但 SurfaceView 遮挡 |
| ANativeActivity 回调拦截 (onNativeWindowCreated) | **回调触发成功**，WebView 创建成功，HTML 加载，但面黑屏 |
| addContentView 叠加 + SurfaceView.setVisibility(GONE) | WebView 创建成功，但底层 SurfaceView 仍然显示黑色 |

### 关键发现

1. **WebView 可以正常创建** — JNI 调用 `new WebView(context)` + `loadDataWithBaseURL` 成功
2. **HTML 已加载** — 日志显示 "Created + HTML loaded (2xxx bytes)"
3. **回调机制正常工作** — `onNativeWindowCreated` 在主线程正确触发
4. **根因**: NativeActivity 的 `SurfaceView` 覆盖在 WebView 之上，无法通过 `setContentView` 或 `addContentView` 移除

### 最新尝试（未完成）

- 切换到 `Java MainActivity`（非 NativeActivity）: `com.operit.hermes.MainActivity`
- Java Activity 直接创建 WebView 并 setContentView
- 问题：DEX 注入到 APK 后，Android 无法找到 MainActivity 类
- 日志：`Activity class {rust.hermes_operit_core/com.operit.hermes.MainActivity} does not exist`
- 尽管 `unzip -l` 确认 APK 中包含 `classes.dex (3616 bytes)` 和 `MainActivity` 类

### DEX 问题分析

`classes.dex` 已正确注入 APK（`jar uf`），但 Android 的 `DexPathList` 只有 `[directory "."]`，没有加载 APK 中的 DEX。原因可能是：
- cargo-apk 生成的 AndroidManifest.xml 不支持 DEX 加载
- 或签名问题导致 APK 验证失败

## 现有代码结构

```
src/
├── main.rs              # 桌面 CLI 入口
├── lib.rs               # 库根
├── android_main.rs      # Android 入口（已简化，不再导出 ANativeActivity_onCreate）
├── android/
│   ├── mod.rs
│   ├── jni.rs           # JNI 桥接（Agent 生命周期）
│   ├── shizuku.rs       # Shizuku 系统级自动化
│   ├── accessibility.rs # 无障碍服务
│   ├── foreground.rs    # 前台服务
│   ├── notification.rs  # 通知管理
│   ├── clipboard.rs     # 剪贴板
│   ├── webview.rs       # WebView 创建（已弃用，改用 Java）
│   └── java/com/operit/hermes/
│       └── MainActivity.java  # Java Activity 入口
├── ui/                  # Dioxus UI 组件（桌面用）
├── core/                # Agent 核心
├── tools/               # 工具注册
├── mcp/                 # MCP 协议
├── store/               # 数据存储
└── environment/         # 沙箱/proot
```

## Cargo.toml 关键配置

- **Android deps**: `jni 0.21`, `android_logger 0.14`
- **已移除**: `dioxus-mobile`, `ndk-sys`, `ndk-glue`
- **activity_name**: `com.operit.hermes.MainActivity`
- **targetSdk**: 34（cargo-apk 实际生成 30，无法覆盖）

## CI Workflow (.github/workflows/ci.yml)

- test job: 本地 cargo check/test/clippy/fmt
- android-apk job: 安装 Android SDK + NDK → javac/d8 DEX → cargo-apk build → jar 注入 → apksigner 签名

## 建议下一步

1. **修 DEX 加载问题** — 让 Android 识别 APK 中的 `classes.dex`
   - 可能方向：用 `aapt2` 重新打包；或修改 cargo-apk 生成的 manifest
   - 或放弃 cargo-apk，改用 Gradle + `cargo-ndk` 构建
2. **验证 Java Activity 方案** — 一旦 DEX 能加载，MainActivity 的 WebView 应该能正常显示
3. **恢复暗色 UI 样式** — 当前 MainActivity.java 中是硬编码 HTML

## 提交历史（最新在前）

```
277a6f8 [FIX] cargo-apk java= 原生编译 Java
d3b3b95 [Clean] 移除 ndk-sys/ndk-glue
69ffa43 [Refactor] Java MainActivity 替代 NativeActivity
0562fba [FIX] 重新获取 window 引用
4d5f07d [FIX] 引用修复: w/d/c 借用
f5316db [FIX] 隐藏 SurfaceView (GONE)
01ce282 [FIX] JNI 类名: $ → $
39f3bfe [FIX] addContentView 叠加在 SurfaceView 上面
143dbac [TEST] 白色背景 — 测试 WebView 是否渲染
...
```

---

*文档生成时间: 2026-07-19*
*状态: APK 可构建，不崩溃，但不显示 UI（黑屏），等待 DEX 加载方案修复*
