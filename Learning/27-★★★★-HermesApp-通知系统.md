# HermesApp 通知系统 — UA Rust 源码级学习

> **UA Rust 分析**：5 nodes / 2 layers / 2 源码文件  
> **文件**：OperitNotificationListenerService(4KB) + SkillRecorderNotification(4KB)  
> **Hermes_Rust_Operit_App 作用**：★★★★ 用户交互的必要能力

---

## 一、UA Rust 发现

```
Project: hermesapp-notification (5 nodes)
├── OperitNotificationListenerService.kt — 通知监听服务
└── SkillRecorderNotification.kt         — 技能录制通知
```

---

## 二、源码学习

### OperitNotificationListenerService.kt

```
OperitNotificationListenerService : NotificationListenerService
├── onListenerConnected()       → 监听器连接
├── onNotificationPosted()      → 新通知到达
├── onNotificationRemoved()     → 通知移除
├── upsert(StatusBarNotification) → 更新/插入通知
├── remove(StatusBarNotification) → 移除通知
├── snapshot(limit, ongoing)    → 获取当前通知快照
└── extractText(Notification)   → 提取通知文本
```

### SkillRecorderNotification.kt

```
SkillRecorderNotification
├── createChannel(context)          → 创建通知渠道
├── buildRecordingNotification()    → 构建录制中通知
├── buildSummarizingNotification()  → 构建摘要中通知
└── serviceIntent(context, action)  → 服务跳转 Intent
```

---

## 三、对 Hermes_Rust_Operit_App 的作用

| 能力 | 方法 | 说明 |
|------|------|------|
| 通知监听 | `NotificationListenerService` | 读取其他 App 通知（上下文） |
| 通知快照 | `snapshot()` | 获取当前通知列表 |
| 通知创建 | `createChannel()` | 创建通知渠道 |
| Agent 态通知 | `buildRecordingNotification()` | Agent 运行状态可见 |

### Rust 复刻总结

```rust
// 通过 jni-rs 调用 Android NotificationManager
fn send_notification(env: &mut JNIEnv, title: &str, text: &str) {
    let manager = env.find_class("android/app/NotificationManager")?;
    // NotificationCompat.Builder → build() → notify()
}

// 通知监听（读取其他 App 通知）
// NotificationListenerService → onNotificationPosted()
// → 获取通知文本 → 提供给 Agent 作上下文
```

### 评分：★★★★
