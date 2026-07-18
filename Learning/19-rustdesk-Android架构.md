# RustDesk Android 架构 — 学习报告

> 仓库：https://github.com/rustdesk/rustdesk  
> Stars：118,423 | License：AGPL-3.0 | 语言：Rust + Flutter (Dart)  
> 核心：远程桌面（TeamViewer 替代），全平台支持

---

## 一、项目架构概览

RustDesk 采用**Rust 核心 + Flutter UI**的双层架构，不像传统 Android 应用使用 Kotlin/Java + NDK。

```
rustdesk/
├── libs/hbb_common/       — 通用库：Protobuf、编解码、TCP/UDP 封装、配置文件
├── libs/scrap/            — 屏幕捕获（各平台实现）
├── libs/enigo/            — 键盘/鼠标控制（平台相关）
├── libs/clipboard/        — 剪贴板（Windows/Linux/macOS）
├── src/server/            — 服务端：音频/剪贴板/输入/视频 + 网络连接
├── src/client.rs          — 发起对端连接
├── src/rendezvous_mediator.rs  — P2P 连接管理（TCP 打洞/中继）
├── src/platform/          — 平台特定代码
└── flutter/               — Flutter UI（桌面 + 移动端全用 Flutter）
```

**关键设计**：`crate-type = ["cdylib", "staticlib", "rlib"]` 使得 `librustdesk` 可以编译为 `.so`（Android/iOS）或静态库。

---

## 二、Android 构建流程

### 2.1 构建工具链

| 组件 | 版本 | 用途 |
|------|------|------|
| Rust toolchain | 1.75 | 编译 Rust 核心 |
| NDK | r28c | Android 原生工具链 |
| cargo-ndk | 3.1.2 | 调用 NDK 进行交叉编译 |
| Flutter | 3.24.5 | UI 框架 + 桥接层 |
| flutter_rust_bridge | 1.80.1 | Rust-Dart FFI 自动代码生成 |

### 2.2 构建步骤（精简）

```
1. cargo-ndk 编译 Rust 为 cdylib (.so)
   → cargo ndk --platform 21 --target aarch64-linux-android build --release --features flutter,hwcodec

2. 复制 liblibrustdesk.so + libc++_shared.so 到 jniLibs/

3. Flutter 构建 APK
   → cd flutter && flutter build apk --release --target-platform android-arm64
```

### 2.3 ndk_arm64.sh 脚本

```bash
cargo ndk --platform 21 --target aarch64-linux-android build \
  --locked --release --features flutter,hwcodec
```

支持四个 Android ABI：
- **arm64-v8a** (`aarch64-linux-android`) — 主要目标
- **armeabi-v7a** (`armv7-linux-androideabi`)
- **x86_64** (`x86_64-linux-android`) — 模拟器
- **x86** (`i686-linux-android`) — 模拟器

### 2.4 对 Operit 的启发

cargo-ndk 是 Android 交叉编译的标准方式，Operit 的 MCP 插件未来编译到 Android 也可以用完全相同的流程：
```bash
cargo ndk --platform 21 --target aarch64-linux-android build --release
```

---

## 三、Rust-Android 桥接方案

### 3.1 flutter_rust_bridge (v1.80.1)

RustDesk 使用 **flutter_rust_bridge**（简称 FRB）实现 Rust→Dart FFI 桥接，这是目前最成熟的方案。

**工作原理**：
1. 在 Rust 端定义 `pub fn`（用 FRB 注解标记）
2. FRB 代码生成器（通过 bridge.yml CI）自动生成 Dart FFI 绑定
3. Dart 端通过这些绑定调用 Rust 函数，就像调用普通 Dart 函数一样

**特点**：
- 跨平台：Android/iOS/Windows/Linux/macOS 一套桥接
- 强类型：生成类型安全的 Dart 代码
- 流支持：Rust 端的 stream 可以映射到 Dart 的 Stream

### 3.2 Cargo.toml 中的桥接配置

```toml
[features]
flutter = ["flutter_rust_bridge"]

[dependencies]
flutter_rust_bridge = { version = "=1.80", features = ["uuid"], optional = true }

[lib]
crate-type = ["cdylib", "staticlib", "rlib"]
```

**关键点**：
- `flutter` feature gate 控制是否启用桥接
- `cdylib` 产出 `.so` 给 Android/iOS
- `staticlib` 产出 `.a` 给桌面端
- `rlib` 用于单元测试

### 3.3 对比 Operit 的 MCP 桥接

| 维度 | RustDesk (FRB) | Operit (MCP) |
|------|---------------|--------------|
| 通信协议 | FFI (共享内存) | stdio JSON-RPC |
| 延迟 | 微秒级 | 毫秒级（进程间） |
| 代码生成 | 自动（FRB codegen） | 手动/半自动 |
| 跨平台 | 全平台 NDK 编译 | 需各平台打包 |
| 复杂度 | 中等（需了解 FRB） | 低（标准 JSON-RPC） |

**结论**：FRB 性能更好（无序列化开销），但 MCP 的松耦合更灵活。Operit 的场景更倾向于 MCP（插件化、热插拔）。

---

## 四、网络层架构

### 4.1 三层网络模型

```
┌─────────────────────────────────────────────┐
│  Rendezvous Server（注册/发现/NAT 打洞协调）  │
├─────────────────────────────────────────────┤
│  Relay Server（直连失败时的中继服务）          │
├─────────────────────────────────────────────┤
│  P2P 直连（TCP 打洞成功后的加密通道）          │
└─────────────────────────────────────────────┘
```

### 4.2 核心组件

| 组件 | 文件 | 职责 |
|------|------|------|
| RendezvousMediator | `src/rendezvous_mediator.rs` | 连接 rendezvous server，协调 P2P 连接 |
| hbb_common | `libs/hbb_common/` | Protobuf 消息定义 + TCP/UDP 封装 |
| stunclient | 外部依赖 | STUN 协议获取公网地址 |
| default-net | 外部依赖 | 网络接口发现 |

### 4.3 连接建立流程

```
1. 客户端 A → Rendezvous Server → 注册上线
2. 客户端 B 请求连接 A
3. Rendezvous Server 协调 TCP 打洞
4. 打洞成功 → P2P 加密通道
5. 打洞失败 → Relay Server 中转
6. 通道建立后，Protobuf 消息传输音频/视频/输入事件
```

### 4.4 技术栈

| 库 | 用途 |
|----|------|
| `tokio` | 异步运行时 |
| `protobuf` | 消息序列化（自定义 message.proto） |
| `stunclient` | STUN 获取公网 IP |
| `parity-tokio-ipc` | 进程间通信（服务端模式） |
| `magnum-opus` | Opus 音频编解码 |
| `sha2` | 连接加密握手 |

---

## 五、三个核心问题回答

### (1) Android 构建流程是什么？

```bash
# 三步走：
1. NDK 设置          → setup-ndk (r28c)
2. Rust 交叉编译      → cargo ndk --target aarch64-linux-android build --release
3. Flutter 打包      → flutter build apk --release
```

编译产物：`liblibrustdesk.so`（~15MB）放入 `jniLibs/arm64-v8a/`  
与 NDK 的 `libc++_shared.so` 一起打包进 APK。

**对 Operit 的意义**：完全可复用的交叉编译流程，NDK + cargo-ndk + target add 即可。

### (2) 用了什么 Rust-Android 桥接方案？

**flutter_rust_bridge v1.80.1**。这是目前最成熟的 Rust↔Flutter FFI 方案：
- 自动生成 Dart 绑定代码（通过 CI 中的 bridge.yml）
- 支持复杂类型、流、异步调用
- 一套代码覆盖 Android + iOS + 桌面端

**替代方案**：
- `dart:ffi` 手动绑定（更轻量但需要手写所有绑定代码）
- `jnigen` + JNI（传统方式，更复杂）

### (3) 网络层架构？

三层 P2P + 中继模型：
- **Rendezvous Server**：节点注册、NAT 类型检测、打洞协调
- **Relay Server**：TCP 中转（无法直连时的后备方案）
- **P2P 直连**：通过 TCP 打洞建立的加密通道

核心使用 Tokio 异步 + Protobuf 序列化 + STUN 协议。

---

## 六、对 Hermes_Rust_Operit_App 的可复用点

1. **cargo-ndk 交叉编译流程** — 完全可复用到 Operit 的 MCP 插件编译（`cargo ndk --target aarch64-linux-android build --release`）
2. **flutter_rust_bridge** — 如果未来 Operit 需要 Flutter UI 绑定 Rust 核心，FRB 是最佳参考
3. **Rendezvous/Relay 架构** — 如果 Operit 需要远程控制/协同功能，可以直接参考此 P2P 模型
4. **Protobuf 消息定义** — hbb_common 中的 Protobuf 模式设计（消息版本、错误码、序列化效率）值得借鉴
5. **Feature gate 设计** — RustDesk 用 `flutter` feature 控制是否编译桥接层，Operit 也可以类似地控制 MCP/非 MCP 模式

## 七、建议

- **优先复用 cargo-ndk 流程**到 Operit 的 MCP 插件 CI 中
- 如果 Operit 未来需要远程协助功能，**不要从头实现网络层**，直接参考 RustDesk 的 rendezvous/relay 架构
- RustDesk 的 Android 构建是全自动 CI 的（GitHub Actions + 缓存），Operit 的 CI 也应按此模式优化
