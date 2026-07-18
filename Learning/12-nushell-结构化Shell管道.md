# Nushell — 结构化 Shell 模式 学习报告
> 项目：https://github.com/nushell/nushell | Rust
> 学习日期：2026-07-18

## 核心创新：结构化数据管道
传统 Shell: text → parse → process → format → text
Nushell:   struct → filter → transform → struct

## 管道架构
- Input (source/producer) → Filter (transform) → Output (sink)
- 类型系统：int, float, string, bool, datetime, duration, filesize, binary, list, record, table, closure, cell-path, nothing
- Table = list<record> 是核心抽象

## 插件协议
- 独立执行文件，stdio/local-socket 通信
- Hello 握手 (version + feature 协商)
- Call/CallResponse + 流式 Data 消息
- Engine 回调 (插件可查询引擎状态)
- Async Signal (Ctrl+C)
- GC 自动停止空闲插件

## 对 Hermes 的 7 条启示
1. **Structured ToolPipelineData enum** 替代文本管道
2. **Typed tool signatures** 静态管道验证
3. **Two-stage execution** (先 plan types 再 execute)
4. **Streaming tool I/O** 处理大输出
5. **Plugin-style tool registration** 含 discovery/versioning/GC
6. **Scoped environments** per pipeline 安全并行
7. **Table-centric data model** 通用交换格式
