# hermes-agent-ultra：安全层增强源码分析

> **仓库**：https://github.com/sheawinkler/hermes-agent-ultra (72⭐, Rust)  
> **关系**：基于 hermes-agent-rs 核心，增加安全+可观测性层  
> **Hermes_Rust_Operit_App 评分**：★★★★（安全设计参考）

---

## 一、安全层源码（从实际文件读到的）

```
hermes-skills/src/
├── guard.rs    → 19种危险模式+9种prompt injection检测
├── hub.rs      → 技能中心
├── skill.rs    → 技能定义
├── store.rs    → 技能存储
└── lib.rs

hermes-intelligence/src/
├── redact.rs   → 30+密钥前缀+PII脱敏
├── router.rs   → 智能路由
├── engine.rs   → 推理引擎
└── ...

hermes-tools/src/
├── approval.rs → ApprovalDecision(Approved/Denied/RequiresConfirmation)
├── credential_guard.rs → 凭据保护
└── ...
```

### approval.rs 核心

```rust
pub enum ApprovalDecision {
    Approved,              // 安全，直接执行
    Denied,                // 拒绝
    RequiresConfirmation,  // 需用户确认
}

// 危险命令模式（DENIED_PATTERNS）
static DENIED_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i)rm\s+-rf\s+/").unwrap(),  // 删除根目录
        Regex::new(r"(?i)mkfs|format").unwrap(),     // 格式化磁盘
        // ... 更多
    ]
});
```

### redact.rs 脱敏

```rust
// 30+ 密钥前缀检测（AWS_ACCESS_KEY, GITHUB_TOKEN, OPENAI_API_KEY...）
// PII 检测（邮箱/电话/信用卡/身份证）
// 自动脱敏后输出
```

---

## 二、对 Hermes_Rust_Operit_App 的作用

| 安全层 | hermes-agent-rs | hermes-agent-ultra | Hermes 应有 |
|--------|----------------|-------------------|------------|
| Prompt injection 检测 | ❌ | ✅ guard.rs | ★★★★★ |
| 命令审批 | ✅ approval.rs | ✅ | ★★★★★ |
| 凭据保护 | ✅ credential_guard.rs | ✅ | ★★★★ |
| PII 脱敏 | ❌ | ✅ redact.rs | ★★★★ |
| 工具策略预设 | ❌ | ✅ | ★★★ |

### Rust 复刻总结

将 ultra 的 6 层安全设计分步集成到 Hermes_Rust_Operit_App：

```rust
// L1: Prompt injection 检测 → tools/guard.rs
// L2: 工具策略预设 → tool_registry.rs 
// L4: PII 脱敏 → core/redact.rs（30+ 密钥前缀检测）
```

### 评分：★★★★
