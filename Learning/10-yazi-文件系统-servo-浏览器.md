# Yazi + Servo — 异步文件系统与浏览器引擎模式
> yazi: https://github.com/sxyazi/yazi | Rust, async I/O
> servo: https://github.com/servo/servo | Rust 浏览器引擎
> 学习日期：2026-07-18

## Yazi: 异步文件系统架构

### Engine Trait — 可插拔文件系统抽象
```rust
pub trait Engine {
    type File: AsyncRead + AsyncSeek + AsyncWrite;
    async fn metadata(&self) -> io::Result<Cha>;
    async fn read_dir(self) -> io::Result<Self::ReadDir>;
    async fn copy<P>(&self, to: P, attrs: Attrs) -> io::Result<u64>;
    fn copy_progressive<P>(&self, to: P) -> mpsc::Receiver<io::Result<u64>>;
    // ... 20+ async ops with defaults
}
```
支持: Local, SSH, Mount, Search 虚拟文件系统

### 任务调度
- **Micro/Macro 双工作池**：5 micro + 10 macro workers
- **优先级通道**：async_priority_channel (low/normal/high)
- **可丢弃任务**：select! { work, cancel_token } — 导航离开即时取消

### CompletionToken
```rust
CompletionToken { inner: Arc<(AtomicU8, Notify)> }
```
零分配，无锁，完美适合 LLM 工具取消

## Servo: 浏览器引擎模式

### 对 Obscura 的借鉴
1. **Pipeline 架构**：每 tab/iframe 一个独立 Pipeline
2. **集中资源管理**：cookies, cache, connections → 单线程管理
3. **Display List 模型**：Layout 产生 display lists 而非像素 — 解耦分析/渲染
4. **独立 crates**：html5ever, cssparser, selectors, url — 都可用

## 对 Hermes 的整合

### FileSystemTool 重构
1. 定义 Engine trait → Local + Memory 实现
2. 全部 async + mpsc::Receiver 进度
3. select! { work, cancel_token } → LLM 可取消
4. 优先级通道 → 紧急/后台操作分离

### BrowserTool (obscura)
1. Pipeline-per-context 多标签隔离
2. 集中资源管理
3. 利用 servo 独立 crates
