# awesome-rust 精选 — 新发现项目分析

> **来源**：rust-unofficial/awesome-rust (58,385⭐)  
> **从 347KB README 中提取的未分析但相关项目**  
> **Hermes_Rust_Operit_App 评分**：见各项目

---

## 一、burn (15,601⭐, Rust)

**用途**：下一代张量库和深度学习框架（candle 的替代）

```
特点:
- 纯 Rust 实现（无 C++ 绑定）
- GPU 支持（CUDA/Metal/Vulkan）
- WASM 支持
- 自动微分
```

**对 Hermes 的作用**：比 candle 更完整的深度学习框架，但在 Hermes 中 candle 已足够。

**Hermes 评分**：★★★（candle 已覆盖）

---

## 二、openinfer (547⭐, Rust)

**用途**：纯 Rust + CUDA LLM 推理引擎（无 PyTorch）

```
特点:
- OpenAI 兼容 API
- 分页 KV 缓存
- CUDA Graph
- 支持 Qwen3 → Kimi-K2
```

**对 Hermes 的作用**：类似 mistral.rs 但还不成熟（547⭐ vs 7.5K⭐）。

**Hermes 评分**：★★★（mistral.rs 已覆盖且更成熟）

---

## 三、axum (26,555⭐, Rust)

**用途**：Tokio 生态的 HTTP 路由和请求处理库

```rust
use axum::{Router, routing::get};

let app = Router::new()
    .route("/", get(handler))
    .route("/api/chat", post(chat_handler));

// Hermes 的 HTTP API 服务器
async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    let result = agent_loop.run().await;
    Json(result)
}
```

**对 Hermes 的作用**：如果需要为 Hermes 添加 HTTP API（如 OpenAI 兼容端点），axum 是最佳选择。

**Hermes 评分**：★★★★（Hermes HTTP API 服务器）

---

## 四、Hiqlite (463⭐, Rust)

**用途**：可嵌入的 SQLite + Raft 集群（高可用 + 缓存）

```
特点:
- 嵌入式 SQLite
- Raft 一致性
- 高可用
- 内置缓存
```

**对 Hermes 的作用**：Hermes 已有 redb（KV 存储），如果未来需要 SQL 查询或高可用，Hiqlite 是轻量方案。

**Hermes 评分**：★★★（redb 已覆盖，未来可选）

---

## 五、上次分析后还缺少的 Rust 生态对照

| awesome-rust 分类 | Hermes 已有 | 尚缺 |
|-----------------|------------|------|
| AI/ML | mistral.rs, candle, openhuman | — |
| Database | redb, qdrant | — |
| Audio | sherpa-onnx | — |
| Asynchronous | tokio | — |
| Web | — | **axum** (新增 ★★★★) |
| Security | keyring-rs | — |
| Text processing | scraper | — |
| Task scheduling | hermes-cron | — |
| Authentication | oauth2 crate | — |
| Caching | — | 可选 (moka) |
| Configuration | hermes-config | — |
| Cryptography | aes-gcm | — |

**主要缺口**：HTTP API 服务器（axum 可补），内存缓存（moka 可补）。

### 评分总览

| 项目 | Stars | 用途 | 评分 |
|------|-------|------|------|
| axum | 26.5K⭐ | HTTP 路由/API | ★★★★ |
| burn | 15.6K⭐ | 深度学习 | ★★★ |
| openinfer | 547⭐ | LLM 推理 | ★★★ |
| Hiqlite | 463⭐ | SQLite+Raft | ★★★ |
