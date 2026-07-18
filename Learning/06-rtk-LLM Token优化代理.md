# 06 — rtk：LLM Token 优化代理源码分析

> **仓库**：https://github.com/rtk-ai/rtk (71,641⭐, Rust)  
> **核心能力**：将 LLM token 消耗降低 60-90%  
> **Hermes_Rust_Operit_App 评分**：★★★★★（直接作为 provider 前置代理）

---

## 一、核心机制

```
用户请求 → rtk 代理 → LLM API
         ↓
   缓存层（常见命令预缓存）
   压缩层（LLM 输出精简）
   路由层（按价格/速度选模型）
```

## 二、对 Hermes_Rust_Operit_App 的作用

| 能力 | 当前 Hermes | 加 rtk 后 | 成本节省 |
|------|-------------|-----------|---------|
| Token 缓存 | 无 | 缓存常见 prompt | ~60-90% |
| 模型路由 | 固定 provider | 按价格/速度自动选 | ~40% |
| 输出压缩 | 无 | 精简非必要输出 | ~20% |

## 三、集成方式

```toml
[dependencies]
# 直接调用 rtk 的 Rust API（如果提供），或通过 HTTP 代理
```

作为 HTTP 代理集成到 Hermes provider 层即可，零代码改动。

### 评分：★★★★★

rtk 是最高性价比的集成——加一个 HTTP 代理就能节省 60-90% token 成本，对使用付费 API 的 Android 用户尤其重要。
