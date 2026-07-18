# HermesApp 无障碍服务 — UA Rust 源码级学习

> **UA Rust 分析**：7 nodes / 2 layers / 4 源码文件  
> **文件**：AccessibilityProviderInstaller(6KB) + ShellExecutor(6KB) + UITools(33KB) + config.xml  
> **Hermes_Rust_Operit_App 作用**：★★★★★ AI 操控 Android 手机的核心管道

---

## 一、UA Rust 分析结果

UA Rust 扫描了 4 个源码文件 + 配置文件：

```
Project: hermesapp-access (7 nodes, 2 layers)
├── Core Code (5 nodes)
│   ├── AccessibilityProviderInstaller.kt  (6KB, 160行) — 无障碍服务安装
│   ├── AccessibilityShellExecutor.kt      (6KB, 169行) — Shell 执行器
│   ├── AccessibilityUITools.kt            (33KB)       — ★核心：UI 交互工具
│   └── OperitAccessibilityService.kt      — 无障碍服务定义
└── Configuration (2 nodes)
    └── accessibility_service_config.xml   — 服务配置
```

---

## 二、源码学习（基于实际函数签名）

### 2.1 AccessibilityUITools.kt (33KB) — 核心 UI 操控

```
AccessibilityUITools extends StandardUITools
├── isAccessibilityServiceEnabled()       → 检查服务是否启用
├── getUIHierarchyWithRetry()             → ★获取当前屏幕 XML 层级
├── getPageInfo()                         → 获取页面信息
├── extractFocusInfoFromAccessibility()   → 提取焦点元素信息
├── simplifyLayout()                      → ★简化 XML 布局为结构化 UI 树
├── clickElement()                        → ★点击元素（按文本/ID/坐标）
├── setInputText()                        → ★设置输入框文本
├── tap()                                 → 点击坐标
├── longPress()                           → 长按
├── swipe()                               → 滑动
├── pressKey()                            → 按键
├── captureScreenshot()                   → 截图（无障碍方式）
└── findNodesInXml()                      → 在 XML 中搜索节点
```

**工作流程**：

```
1. getUIHierarchyWithRetry() → 获取屏幕 XML
2. simplifyLayout() → 简化为结构化 UI 树
3. 传给 LLM 分析 → 决定操作
4. clickElement() / setInputText() / tap() → 执行
5. 循环到 1（观察新状态）
```

### 2.2 AccessibilityShellExecutor.kt (6KB)

```
AccessibilityShellExecutor implements ShellExecutor
├── setAccessibilityService()              → 注册服务实例
├── isAvailable()                          → 检查可用
├── hasPermission()                        → 检查权限
├── requestPermission()                    → 请求权限
├── startProcess()                         → 启动 Shell 进程
├── executeCommand()                       → ★执行 Shell 命令
└── isAccessibilityServiceEnabled()        → 检查服务启用状态
```

### 2.3 AccessibilityProviderInstaller.kt (6KB)

```
AccessibilityProviderInstaller
├── getBundledVersion()                    → 获取内置版本
├── getInstalledVersion()                  → 获取已安装版本
├── isUpdateNeeded()                       → 是否需要更新
├── launchInstall()                        → 启动安装
└── clearCache()                           → 清除缓存
```

---

## 三、对 Hermes_Rust_Operit_App 的作用

HermesApp 的无障碍服务是 AI 操控手机的完整实现：

| 能力 | 方法 | Rust 桥接方案 |
|------|------|-------------|
| 读取屏幕 | getUIHierarchyWithRetry() → XML | JNI 调用 AccessibilityService |
| 简化布局 | simplifyLayout() → UI 树 | JNI 转换 XML→JSON |
| 点击元素 | clickElement() | JNI 调用 performAction |
| 输入文本 | setInputText() | JNI 调用 setText |
| 滑动 | swipe() | JNI 调用 dispatchGesture |
| 截图 | captureScreenshot() | JNI 调用 takeScreenshot |
| Shell 命令 | executeCommand() | JNI 调用 Runtime.exec |

### 评分：★★★★★

无障碍服务是 Hermes_Rust_Operit_App 操控 Android 手机的**核心管道**。HermesApp 的 33KB Kotlin 实现是 Rust JNI 桥接的直接参考。
