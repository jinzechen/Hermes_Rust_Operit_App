# 12 — sherpa-onnx：语音引擎源码分析

> **仓库**：https://github.com/k2-fsa/sherpa-onnx (13,630⭐, C++)  
> **Rust 绑定**：https://github.com/thewh1teagle/sherpa-rs (310⭐, Rust)  
> **Hermes_Rust_Operit_App 评分**：★★★★★（Android 语音交互核心）

---

## 一、核心能力

| 能力 | 描述 | Android 支持 |
|------|------|-------------|
| **ASR** 语音→文字 | Whisper / Paraformer / Zipformer | ✅ 预编译 .so |
| **TTS** 文字→语音 | VITS / Matcha-TTS / Coqui-AI | ✅ |
| **VAD** 语音检测 | Silero VAD | ✅ |
| **说话人分离** | 谁在说话 | ✅ |

## 二、对 Hermes_Rust_Operit_App 的作用

| 当前 | 加 sherpa 后 |
|------|-------------|
| 仅文字交互 | 语音输入+语音输出 |
| 需网络 | 完全本地 |
| 无唤醒 | 语音唤醒 |

### Rust 复刻总结

```rust
// 直接加载 sherpa-onnx Android .so
// sherpa-onnx 已提供预编译 aarch64-linux-android .so

// ASR：语音→文字
let recognizer = sherpa_rs::Recognizer::new("whisper-tiny.onnx");
let text = recognizer.recognize(audio_data)?;

// TTS：文字→语音
let synthesizer = sherpa_rs::Tts::new("vits-model.onnx");
let audio = synthesizer.synthesize("你好，我是 Hermes")?;
```

### 评分：★★★★★
