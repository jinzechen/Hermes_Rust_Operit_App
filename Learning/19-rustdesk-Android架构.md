# RustDesk Android 架构 — 深度学习报告（含 UA Rust 分析）

> 仓库：https://github.com/rustdesk/rustdesk  
> Stars：118,423 | License：AGPL-3.0 | 语言：Rust + Flutter (Dart)  
> 核心：远程桌面（TeamViewer 替代），全平台支持

---

## 一、UA Rust 深度分析结果

### 1.1 扫描概况

| 指标 | 值 |
|------|----|
| 文件数 | 11 (4 Rust, 2 TOML, 2 Shell, 3 YAML) |
| 知识图谱节点 | 21 |
| 关系连线 | 15 |
| 架构层数 | 2 |
| 引导步数 | 5 |

### 1.2 核心文件规模

| 文件 | 大小 | 职责 |
|------|------|------|
| `.github/workflows/flutter-build.yml` | **92.7KB** | CI 构建全流程（Windows/Android/Linux/macOS） |
| `src/client.rs` | **157KB** | 客户端核心：连接管理、P2P 通道 |
| `src/rendezvous_mediator.rs` | **39.6KB** | Rendezvous 中介协议：NAT 打洞 + 注册 |
| `flutter/build_fdroid.sh` | **14.5KB** | F-Droid 构建脚本 |
| `Cargo.toml` | **7.9KB** | 工作区配置（6 个 sub-crates） |
| `flutter/pubspec.yaml` | **6.2KB** | Flutter 依赖（flutter_rust_bridge v1.80.1） |

### 1.3 Android 构建 CI 流程（flutter-build.yml 92.7KB 精华）

```
build-rustdesk-android job:
  matrix:
    - aarch64-linux-android (arm64-v8a)
    - armv7-linux-androideabi (armeabi-v7a)
    - x86_64-linux-android (emulator)

  steps:
    1. setup-ndk (r28c)               ← NDK 版本锁死
    2. rustup target add <target>
    3. cargo install cargo-ndk 3.1.2  ← cargo-ndk 版本锁死
    4. flutter/ndk_arm64.sh:         ← 核心构建命令
       cargo ndk --platform 21 --target aarch64-linux-android build \
         --locked --release --features flutter,hwcodec
    5. cp liblibrustdesk.so → jniLibs/
    6. flutter build apk --release
```

---

## 二、Android 构建流程详解

### 2.1 工具链

| 组件 | 版本 | 说明 |
|------|------|------|
| Rust toolchain | 1.75 | 刻意锁死（1.78 有 i128 ABI 变化导致 Sciter 挂） |
| NDK | r28c | Android 原生开发套件 |
| cargo-ndk | 3.1.2 | Rust→Android 交叉编译桥梁 |
| Flutter | 3.24.5 | UI 框架 |
| flutter_rust_bridge | 1.80.1 | Rust-Dart FFI 自动生成 |

### 2.2 支持 ABI

| ABI | Rust Target | 用途 |
|-----|-------------|------|
| arm64-v8a | `aarch64-linux-android` | 主力（～99% 设备） |
| armeabi-v7a | `armv7-linux-androideabi` | 老旧设备 |
| x86_64 | `x86_64-linux-android` | 模拟器 |
| x86 | `i686-linux-android` | 旧模拟器 |

### 2.3 构建命令（ndk_arm64.sh）

```bash
cargo ndk --platform 21 --target aarch64-linux-android build \
  --locked --release --features flutter,hwcodec
```

- `--platform 21` → Android 5.0+ (API 21)
- `--features flutter` → 启用 flutter_rust_bridge 代码生成
- `--features hwcodec` → 启用硬件编解码（mediacodec）
- 产物：`target/aarch64-linux-android/release/liblibrustdesk.so`

### 2.4 对 Operit 的启发

完全可复用的交叉编译流程：
```bash
cargo ndk --platform 21 --target aarch64-linux-android build --release
```
这就是 Operit MCP 插件未来的 Android 编译命令。

---

## 三、Rust-Android 桥接方案

### 3.1 flutter_rust_bridge v1.80.1

RustDesk 使用的桥接方案是目前最成熟的 Rust↔Flutter FFI 方案。

**工作原理**：
1. Rust 端定义 `pub fn`（用 FRB 注解）
2. CI 的 `bridge.yml` 自动生成 Dart 绑定代码
3. Dart 端通过生成代码调用 Rust 函数

**关键 Cargo.toml 配置**：
```toml
[features]
flutter = ["flutter_rust_bridge"]

[dependencies]
flutter_rust_bridge = { version = "=1.80", features = ["uuid"], optional = true }

[lib]
crate-type = ["cdylib", "staticlib", "rlib"]
# cdylib → .so (Android/iOS)
# staticlib → .a (桌面)
# rlib → 单元测试
```

### 3.2 对比 Operit 的 MCP 桥接

| 维度 | RustDesk (FRB) | Operit (MCP) |
|------|---------------|--------------|
| 通信协议 | FFI (共享内存) | stdio JSON-RPC |
| 延迟 | 微秒级 | 毫秒级 |
| 代码生成 | 自动 | 手动/半自动 |
| 跨平台 | 全平台 NDK 编译 | 需各平台打包 |
| 复杂度 | 中等 | 低 |

**结论**：FRB 性能更优，MCP 更灵活。Operit 的场景更适合 MCP（插件化、热插拔）。

---

## 四、网络层架构

### 4.1 三层网络模型

```
Rendezvous Server (注册/发现/NAT 打洞协调)
    ↕ TCP/STUN
Relay Server (直连失败时的中继)
    ↕ TCP
P2P 直连 (加密通道)
```

### 4.2 核心组件

| 组件 | 文件 | 职责 |
|------|------|------|
| RendezvousMediator | `src/rendezvous_mediator.rs` | 连接协调服务器，TCP 打洞 |
| hbb_common | `libs/hbb_common/` | Protobuf 消息 + TCP/UDP 封装 |
| stunclient | 外部依赖 | 获取公网地址 |
| default-net | 外部依赖 | 网络接口发现 |

### 4.3 连接流程

```
1. 客户端 A → Rendezvous Server
2. 客户端 B 请求连接 A
3. Rendezvous 协调 TCP 打洞
4. 成功 → P2P 加密通道
5. 失败 → Relay 中转
```

---

## 五、三个核心问题回答

### (1) Android 构建流程是什么？

三步走：
```bash
# 1. NDK 设置
setup-ndk r28c

# 2. Rust 交叉编译
cargo ndk --platform 21 --target aarch64-linux-android build --release

# 3. Flutter 打包
flutter build apk --release
```

产物：`liblibrustdesk.so` ~15MB → `jniLibs/arm64-v8a/`

### (2) 用了什么 Rust-Android 桥接方案？

**flutter_rust_bridge v1.80.1**：
- 自动生成 Dart FFI 绑定（bridge.yml CI 自动化）
- 支持复杂类型、流、异步
- 一套代码覆盖全部平台

### (3) 网络层架构？

三层 P2P + 中继：
- Rendezvous Server（发现 + 打洞协调）
- Relay Server（后备中转）
- P2P 直连（加密通道）

技术栈：Tokio + Protobuf + STUN

---

## 六、对 Hermes_Rust_Operit_App 的可复用点

| 复用点 | 优先级 | 说明 |
|--------|--------|------|
| cargo-ndk 流程 | ★★★★★ | Operit MCP 插件的 Android 编译可直接复制 |
| FRB vs MCP 对比 | ★★★★ | 验证 MCP 对 Operit 更合适 |
| Rendezvous/Relay 架构 | ★★★ | 未来远程协同功能可直接参考 |
| CI 缓存策略 | ★★★ | GitHub Actions + vcpkg 二进制的缓存优化 |
| 多 ABI 矩阵构建 | ★★★ | arm64-v8a / armeabi-v7a / x86_64 并行编译 |
