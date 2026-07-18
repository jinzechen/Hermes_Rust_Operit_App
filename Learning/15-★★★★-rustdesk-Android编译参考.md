# 14 — rustdesk：Rust Android 应用参考

> **仓库**：https://github.com/rustdesk/rustdesk (118,426⭐, Rust)  
> **核心价值**：最大的 Rust Android 应用，Rust→Android 编译流程参考  
> **Hermes_Rust_Operit_App 评分**：★★★★（Android 交叉编译参考）

---

## 一、Android 构建流程

```
cargo-ndk --platform 21 --target aarch64-linux-android build --release
→ liblibrustdesk.so → jniLibs/arm64-v8a/
→ Flutter build apk
```

## 二、对 Hermes_Rust_Operit_App 的作用

| 可复用点 | 说明 |
|----------|------|
| **cargo-ndk 流程** | Hermes 的 Android 编译命令完全一致 |
| **NDK r28c** | 已验证的 NDK 版本 |
| **Flutter+Rust** | 如果未来需要 Flutter UI 的参考 |

### 评分：★★★★

rustdesk 的 Android 交叉编译流程是 Hermes 可以直接复制的。118K⭐ 的成功项目验证了 Rust→Android 的可行性。
