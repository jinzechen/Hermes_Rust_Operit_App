# Oxideterm — AI 终端集成 学习报告
> 项目：https://github.com/AldeanSoftware/oxideterm | Rust
> 学习日期：2026-07-18

## 核心概念
Oxideterm 是一个 AI-native 的工作区，零 Electron 零 OpenSSH，完全 Rust。

## 对 Hermes 的关键借鉴
1. **AI + Terminal 融合模式**：不是"给终端加 AI"，而是终端本身就是 AI 的交互界面
2. **本地 + 远程无差异**：同一套代码操作本地 Shell 和远程机器
3. **工具系统**：终端操作被包装为可被 AI 调用的工具

## 整合方案
- Hermes 的 Environment 模块可以参考其本地/远程统一抽象
- Alpine proot 环境 + SSH 远程统一接口
