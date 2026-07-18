# 09 — Codex Mobile + iHermes 移动端 学习报告

> **Codex Mobile**：https://github.com/friuns2/codex-mobile (778⭐, TypeScript)  
> **iHermes**：https://github.com/2winter-dev/iHermes (27⭐, TypeScript)  
> **Hermes 集成现状**：同等定位（Hermes_Rust_Operit_App 就是 Android 上的 AI Agent）

---

## 核心发现

### Codex Mobile

在 Termux 上运行 OpenAI Codex CLI，展示了 Android 上运行 AI Agent 的完整工程方案。

### iHermes

Hermes Agent 的移动端 Web 界面。

## 对 Hermes_Rust_Operit_App 的作用

Hermes_Rust_Operit_App 是**纯 Rust 编译到 Android** 的方案，相比：
- **Codex Mobile**: 需要 Node.js + Termux（非原生）
- **iHermes**: 需要 Web 服务器 + 浏览器

Hermes 是**真正的原生 Android AI Agent**（编译为 .so 嵌入 Operit）。

## 三到五个可复用点

1. **Termux 部署模式** — Android 运行 AI CLI 的工程参考
2. **纯 Rust 优势** — 无需 Node.js，单二进制
3. **原生 Android** — Operit 插件形式，用户体验更好
