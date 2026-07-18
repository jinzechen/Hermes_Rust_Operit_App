# 27 — shadowsocks-rust 网络模式 学习报告

> **UA Rust 分析时间**：2026-07-18  
> **项目**：https://github.com/shadowsocks/shadowsocks-rust (10,767⭐, Rust)  
> **版本**：v1.25.0 | License：MIT | 语言：Rust (edition 2024)  
> **Hermes 集成现状**：❌ 未集成

---

## 第一步：UA Rust 深度扫描

```bash
ua scan analysis/shadowsocks → 2 文件
ua build → 4 节点 / 1 边
```

## 第二步：架构分析

### 二进制入口

```
sslocal    — 本地客户端 (SOCKS5 代理)
ssserver   — 远程服务端
ssurl      — URL 解析工具
ssmanager  — 管理接口
ssservice  — 系统服务
```

### 核心依赖（Cargo.toml 7.3KB）

| 依赖 | 用途 |
|------|------|
| **tokio** | 异步 I/O 运行时 |
| **aes-gcm / chacha20** | 加密实现 |
| **trust-dns** | DNS 解析 |
| **sodiumoxide** | 加密库 |
| **socks** | SOCKS5 协议 |

### 网络模式

```
客户端 → 本地 SOCKS5 → 加密隧道 → 远程服务器 → 目标网站
         ↑                          ↑
     local cipher               server cipher
     (AEAD加密)                  (AEAD解密)
```

## 第三步：对 Hermes_Rust_Operit_App 的作用

| 可复用点 | 说明 |
|----------|------|
| **异步 I/O 架构** | shadowsocks 的 tokio 网络模式与 Hermes 相同 |
| **加密层设计** | AEAD 加密可复用于 Hermes 的网络通信 |
| **SOCKS5 代理** | Android 上可能需要代理支持 |
| **DNS 解析** | trust-dns 可用于 Hermes 的 DNS 查询 |

## 第四步：三到五个可复用点

1. **Tokio 异步网络** — shadowsocks 的 async 模式与 Hermes 一致
2. **AEAD 加密** — 可复用于 Hermes 的安全通信
3. **SOCKS5 代理** — Android 网络访问的前置代理
4. **DNS 解析库** — trust-dns 集成
