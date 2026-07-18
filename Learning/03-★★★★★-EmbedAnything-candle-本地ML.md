# 11 — EmbedAnything + candle：本地嵌入/ML 推理

> **EmbedAnything**：https://github.com/StarlightSearch/EmbedAnything (1,283⭐, Rust)  
> **candle**：https://github.com/huggingface/candle (20,686⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（本地嵌入核心，赋予离线 AI 能力）

---

## 一、EmbedAnything 管线架构

```
输入文件(PDF/TXT/JPG/WAV)
    ↓ Processor（分块/提取）
    ↓ Inference（Candle/ONNX 嵌入）
    ↓ MPSC 通道 → Vector DB
```

### 支持模型

| 模型 | 大小 | 用途 |
|------|------|------|
| MiniLM-L6-v2 | 22M | 通用文本嵌入 |
| Model2Vec (potion) | **8M** | 🌟Android 最佳 |
| Jina Embeddings v2 | — | 多语言 |
| CLIP | — | 图文跨模态 |
| Whisper | — | 语音→文本 |

---

## 二、candle ML 框架

HuggingFace 的 Rust ML 框架，支持 CPU/GPU:

```toml
[dependencies]
candle-core = "0.11"
candle-nn = "0.11"
candle-transformers = "0.11"
```

## 三、对 Hermes_Rust_Operit_App 的作用

| 能力 | 方案 | 模型 | 内存 |
|------|------|------|------|
| 文本嵌入 | candle + Model2Vec | potion-base-8M | ~50MB |
| 图片分析 | candle + CLIP | openai/clip | ~200MB |
| 语音识别 | candle + Whisper | tiny.en | ~150MB |
| 重排序 | candle + BGE Reranker | bge-reranker | ~100MB |

### Rust 复刻总结

```rust
use candle_core::Device;
use candle_nn::Embedding;

// 本地嵌入（Model2Vec 8M 参数，Android 可用）
let device = Device::Cpu;
let model = Embedding::new(/* 加载 Model2Vec */);
let embedding = model.forward(&tokens)?;

// 嵌入后存入 qdrant 或 tinycortex 的向量存储
```

### 评分：★★★★★
