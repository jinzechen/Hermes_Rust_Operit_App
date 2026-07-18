# HermesApp 前台服务 — UA Rust 源码级学习

> **UA Rust 分析**：2 个关键文件  
> **文件**：AIForegroundService.kt (80KB) + ForegroundServiceCompat.kt (2KB)  
> **Hermes_Rust_Operit_App 作用**：★★★★ Android 后台运行保障

---

## 一、UA Rust 发现

```
Project: hermesapp-foreground
├── AIForegroundService.kt (80KB) — ★前台服务主实现
└── ForegroundServiceCompat.kt (2KB) — 兼容层
```

---

## 二、源码学习

### AIForegroundService.kt (80KB)

```
class AIForegroundService : Service()
├── 通知管理:
│   ├── buildReplyNotificationTag()        → 回复通知标签
│   ├── createMainActivityPendingIntent()  → 点击通知打开主界面
│   ├── ensureReplyNotificationChannel()   → 创建通知渠道
│   └── notifyReplyCompleted()             → 回复完成通知
│
├── 前台运行:
│   ├── ensureMicrophoneForeground()       → 麦克风前台服务
│   ├── ensureRunningForExternalHttp()     → HTTP 服务保活
│   └── hasPersistentForegroundResponsibilityConfigured() → 持久前台检查
│
├── 语音唤醒:
│   ├── setWakeListeningSuspendedForIme()  → 输入法时暂停唤醒
│   ├── setWakeListeningSuspendedForFloatingFullscreen() → 全屏时暂停
│   ├── startRecordingStateMonitoring()    → 录音状态监控
│   ├── ensureWakeSpeechProvider()         → 语音唤醒提供者
│   └── applyWakeListeningState()          → 应用唤醒状态
│
└── 兼容层:
    ├── ForegroundServiceCompat.buildTypes()  → 构建前台类型
    ├── ForegroundServiceCompat.startForeground() → 启动前台(兼容)
    └── ForegroundServiceCompat.startForegroundWithFallback() → 带回退的启动
```

---

## 三、对 Hermes_Rust_Operit_App 的作用

| 能力 | 方法 | 说明 |
|------|------|------|
| 后台保活 | `startForeground()` | 防止系统杀死 Agent |
| 通知交互 | `ensureReplyNotificationChannel()` | 用户可回复/控制 |
| 语音唤醒 | `ensureWakeSpeechProvider()` | 语音关键词唤醒 |
| 录音监控 | `startRecordingStateMonitoring()` | 监听录音状态 |
| HTTP 服务 | `ensureRunningForExternalHttp()` | 外部 MCP 连接 |

### 评分：★★★★

前台服务是 Android Agent 后台运行的保障。HermesApp 的80KB实现涵盖了通知管理、语音唤醒、录音监控等完整功能。Rust 版只需核心功能（后台保活+通知）即可。
