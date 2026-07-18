# 06 — rtk：LLM Token 优化代理源码分析

> **仓库**：https://github.com/rtk-ai/rtk (71,641⭐, Rust)  
> **核心**：LLM API 代理，缓存+压缩+路由，降 token 消耗 60-90%  
> **Hermes_Rust_Operit_App 评分**：★★★★★

---

## 一、架构

```
用户请求 → rtk → LLM API
         ↓
   ├── 缓存层: 常见 prompt 预缓存
   ├── 压缩层: 输出精简
   └── 路由层: 按价格/速度选模型
```

## 二、对 Hermes 的作用

| 能力 | 当前 | 加 rtk 后 | 节省 |
|------|------|-----------|------|
| Token 缓存 | ❌ | 缓存重复请求 | 60-90% |
| 模型路由 | 固定 provider | 自动选择 | 40% |
| 输出压缩 | ❌ | 精简输出 | 20% |

## 三、集成

作为 HTTP 代理集成到 Hermes provider 层，零代码改动。只需设置：

```bash
export HERMES_LLM_BASE_URL=http://localhost:8080  # rtk 代理地址
```

## 评分：★★★★★

rtk 是最高性价比集成——零代码改动、直接节省 60-90% token 成本。
