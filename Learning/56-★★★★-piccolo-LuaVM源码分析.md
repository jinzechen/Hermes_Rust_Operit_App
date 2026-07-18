# piccolo — 纯 Rust Lua VM UA 源码分析

> **UA Rust 分析**：17 nodes / 4 Rust 源文件解析  
> **仓库**：https://github.com/kyren/piccolo  
> **Hermes_Rust_Operit_App 评分**：★★★★（嵌入式脚本沙盒）

---

## 一、UA Rust 发现的模块

```
lib.rs 导出模块（25+）:
├── any / callback / closure — Lua 类型系统
├── compiler / opcode — 编译+字节码
├── gc — 增量 GC
├── vm / stack / fuel — 虚拟机+栈+执行配额
├── stdlib / io — 标准库+IO
├── types / value / conversion — 类型+值+转换
├── table / string — 表+字符串
├── thread / registry — 协程+注册表
├── async_callback — 异步回调
├── error / stash / finalizers — 错误/隐藏值/终结器
├── meta_ops / userdata — 元操作/用户数据
└── function / constant — 函数+常量
```

---

## 二、关键特性

```rust
use piccolo::{Lua, LuaResult};

// 执行 Lua（自带沙盒）
let mut lua = Lua::new();
lua.enter(|ctx| {
    let result: i64 = ctx.exec::<i64>("return 1 + 2")?;
    assert_eq!(result, 3);
    Ok(())
});

// 内置: 增量 GC、循环检测、燃料限制(DoS 防护)
```

| 特性 | piccolo | boa_engine |
|------|---------|-----------|
| 语言 | Lua | JavaScript |
| 沙盒 | ✅ 内置 | ❌ |
| 增量 GC | ✅ | ✅ |
| 大小 | 更小 | ~7MB |
| 燃料限制 | ✅ | ❌ |

**Hermes 作用**：嵌入式脚本执行沙盒（用户自定义工具、Workflow 脚本）。

### 评分：★★★★

piccolo 的 Lua 沙盒天然适合 Agent 执行用户脚本（比 JS 更安全）。继续分析 tantivy。
