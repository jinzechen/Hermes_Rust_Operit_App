# rhai — 嵌入式脚本引擎 UA 源码分析

> **UA Rust 分析**：lib.rs (16KB) / 1.6K 行  
> **仓库**：https://github.com/rhaiscript/rhai (4K+⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★（轻量嵌入式脚本）

---

## 一、核心 API

```rust
use rhai::{Engine, Scope};

let engine = Engine::new();
engine.register_fn("add", |a: i64, b: i64| a + b);

let result: i64 = engine.eval("add(40, 2)")?;
println!("{result}"); // 42

// 自定义类型注册
engine.register_type_with_name::<AgentState>("AgentState")
      .register_get("name", |s: &mut AgentState| s.name.clone());
```

---

## 二、对比其他脚本引擎

| 引擎 | 大小 | 语法 | 嵌入式 | Android |
|------|------|------|--------|---------|
| **rhai** | **~200KB** | Rust 风格 | ✅ | ✅ |
| boa_engine | ~7MB | JavaScript | ✅ | ⚠️ 大 |
| piccolo | ~1MB | Lua | ✅ | ✅ |
| rune | ~3MB | Rust 风格 | ✅ | ✅ |

**Hermes 作用**：用户自定义工具/Workflow 的脚本引擎（比 JS 更轻量，比 Lua 更 Rust）。

### 评分：★★★★

rhai 是 Hermes 嵌入式脚本的最佳平衡点：轻量（200KB）、Rust 原生语法、支持 Android。
