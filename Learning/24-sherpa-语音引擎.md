# 24 — sherpa-rs 语音引擎 深度学习报告

> **UA Rust 分析时间**：2026-07-18  
> **sherpa-rs**：https://github.com/thewh1teagle/sherpa-rs (310⭐, Rust)  
> **上游 sherpa-onnx**：https://github.com/k2-fsa/sherpa-onnx (13,630⭐, C++)  
> **核心**：纯 ONNX 语音识别（ASR）+ 语音合成（TTS），支持 Android  
> **Hermes 集成现状**：❌ 未集成

---

## 第一步：UA Rust 深度扫描

```bash
ua scan analysis/sherpa-rs → 4 文件
ua build → 6 节点 / 2 边
```

## 第二步：核心能力

sherpa-onnx（上游）支持：

```
ASR: 语音→文字（Whisper / Paraformer / Zipformer）
TTS: 文字→语音（VITS / Matcha-TTS / Coqui-AI）
VAD: 语音活动检测
说话人分离: 谁在说话
增强: 降噪/分离
```

**关键优势**：纯 ONNX Runtime，无需 GPU，**Android 原生支持**。

## 第三步：对 Hermes_Rust_Operit_App 的作用

| 能力 | 当前 Hermes | 加 sherpa-rs 后 |
|------|-------------|-----------------|
| 语音输入 | ❌ 无 | ✅ ASR 语音→文本 |
| 语音输出 | ❌ 无 | ✅ TTS 文本→语音 |
| 离线 | ❌ 需网络 | ✅ 完全本地 |

### 集成方式

```toml
[dependencies]
sherpa-rs = "0.1"
# 或直接调用 sherpa-onnx C API
```

Android 上 sherpa-onnx 已提供预编译 `.so`：`sherpa-onnx` 在 Android 上可直接加载。

## 第四步：三到五个可复用点

| # | 可复用点 | 说明 |
|---|---------|------|
| 1 | **语音交互** | Android Agent 最自然的交互方式 |
| 2 | **本地 ASR** | Whisper 模型离线运行，无需网络 |
| 3 | **本地 TTS** | VITS 模型语音合成 |
| 4 | **Android 预编译** | sherpa-onnx 有 Android .so，直接加载 |
| 5 | **极低资源** | ONNX Runtime 小模型在手机流畅运行 |
