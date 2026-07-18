# 15 — shadowsocks-rust：异步网络模式参考

> **仓库**：https://github.com/shadowsocks/shadowsocks-rust (10,767⭐, Rust)  
> **核心价值**：tokio 异步网络编程模式  
> **Hermes_Rust_Operit_App 评分**：★★★（网络层参考）

---

## 一、核心依赖

| 依赖 | 用途 | Hermes 也有 |
|------|------|-------------|
| **tokio** | 异步运行时 | ✅ |
| **aes-gcm** | 加密 | ❌ 按需 |
| **trust-dns** | DNS | ❌ 按需 |
| **socks** | SOCKS5 | ❌ 按需 |

## 二、对 Hermes_Rust_Operit_App 的作用

| 可复用点 | 说明 |
|----------|------|
| tokio 异步架构 | Hermes 同样使用，可参考其事件循环 |
| AEAD 加密 | 如果需要加密通信 |
| SOCKS5 代理 | Android 网络代理 |

### Rust 复刻总结

Hermes 的 Agent 通信主要通过 HTTP API 和 stdio MCP，不涉及代理/加密隧道。shadowsocks 的 tokio 异步模式已有参考。

### 评分：★★★
