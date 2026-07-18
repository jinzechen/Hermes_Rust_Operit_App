# boa_engine — Rust 嵌入式 JavaScript 引擎源码分析

> **仓库**：https://github.com/boa-dev/boa (7,402⭐, Rust)  
> **对应**：Operit 的 `:quickjs` 模块  
> **Hermes_Rust_Operit_App 评分**：★★★★（替换 quickjs，执行用户 JS 脚本）

---

## 一、源码结构

```
boa (workspace)
├── core/        — JS 引擎核心（解析器+执行器+GC）
├── parser/      — JavaScript 解析器
├── gc/          — 垃圾回收
├── builtins/    — 内置对象（Array/Object/String/...）
├── boa_engine/  — 统一入口 crate
└── examples/    — 使用示例
```

---

## 二、使用方式

```rust
use boa_engine::{Context, Source};

let mut context = Context::default();

// 执行 JS 代码
let result = context.eval(Source::from_bytes(
    r#"
    function add(a, b) { return a + b; }
    add(1, 2)
    "#
)).unwrap();

println!("Result: {}", result.to_string(&mut context).unwrap());
// → "3"

// 传值给 Rust
let val = result.to_number(&mut context).unwrap();
assert_eq!(val, 3.0);
```

---

## 三、对 Hermes_Rust_Operit_App 的作用

| 场景 | JS 代码 | boa 的用途 |
|------|---------|-----------|
| 用户自定义工具 | `function tool(args) { return result }` | Agent 调用 |
| 数据转换 | `json.map(x => x.name)` | 管道处理 |
| 自定义 Workflow | `steps.forEach(step => execute(step))` | 工作流 |

### 评分：★★★★

boa_engine 允许用户用 JS 编写自定义工具和 Workflow，类似 Fabric 的 pattern。不是核心依赖，但提供灵活性。
