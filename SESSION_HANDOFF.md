# SESSION_HANDOFF.md — v4-flash 学习任务清单

> 接收者：deepseek-v4-flash  
> 你的任务：**只学习，不写代码**  
> 每完成一个项目，写一份学习报告到 Learning/ 目录，git commit && git push

---

## 学习任务列表（按顺序逐个执行）

### 任务 1：学习 ds4 本地推理引擎
```
仓库：https://github.com/ldclabs/ds4
重点：DeepSeek 4 Flash 本地推理，Metal 加速
产出：Learning/18-ds4-本地推理.md
问题：(1) 能否在 Android 上运行？(2) 和 llama.cpp 比有什么优势？(3) API 接口是什么样？
```

### 任务 2：学习 RustDesk Android 架构
```
仓库：https://github.com/rustdesk/rustdesk
重点：Rust 写的 Android 应用！80k+ stars
产出：Learning/19-rustdesk-Android架构.md
问题：(1) 他们的 Android 构建流程是什么？(2) 用了什么 Rust-Android 桥接方案？(3) 网络层架构？
```

### 任务 3：学习 fabric AI 模式库
```
仓库：https://github.com/danielmiessler/fabric
重点：255 个 AI patterns，9 种 strategies
产出：Learning/20-fabric-AI模式库.md
问题：(1) pattern 格式规范是什么？(2) 哪些 pattern 适合内置到 Agent？(3) strategy 怎么组合 pattern？
```

### 任务 4：学习 GlueSQL 嵌入式数据库
```
仓库：https://github.com/gluesql/gluesql
重点：Rust 写的 SQL 数据库，可嵌入
产出：Learning/21-gluesql-嵌入式数据库.md
问题：(1) 和 redb 比优缺点？(2) 和 SQLite 比？(3) 是否适合做记忆存储？
```

### 任务 5：学习 EmbedAnything 推理管线
```
仓库：https://github.com/StarlightSearch/EmbedAnything
重点：Rust 推理和嵌入管线
产出：Learning/22-EmbedAnything-推理管线.md
问题：(1) 支持哪些嵌入模型？(2) 管线架构？(3) 能否在 Android 上用？
```

### 任务 6：学习 Agent-S 计算机使用
```
仓库：https://github.com/simular-ai/Agent-S
重点：S3 版超越人类 (OSWorld 72.6%)
产出：Learning/23-AgentS-计算机使用.md
问题：(1) 计算机使用 Agent 的架构？(2) 怎么做到超人类的？(3) 能不能用在 Android 上？
```

### 任务 7：学习 sherpa-rs 语音引擎
```
仓库：搜索 crates.io 上的 sherpa-rs
重点：Rust 语音合成/识别
产出：Learning/24-sherpa-语音引擎.md
问题：(1) API 接口？(2) 支持中文吗？(3) 能否编译到 Android？
```

### 任务 8：学习 MCP Rust 实现
```
仓库：搜索 crates.io 上的 rmcp, mcp-rs, mcp-core
重点：Rust 原生 MCP 协议实现
产出：Learning/25-MCP-Rust生态.md
问题：(1) 哪个最成熟？(2) 和我们的 McpClient 比谁更好？(3) 值得替换吗？
```

### 任务 9：学习 skillsrs/skills-rs
```
仓库：搜索 crates.io 和 GitHub 上的 skillsrs, skills-rs
重点：Rust Skill 框架
产出：Learning/26-skillsrs-Rust技能框架.md
问题：(1) skill 格式如何定义？(2) 和 Operit 的 .skill 兼容吗？(3) 值得用吗？
```

### 任务 10：学习 shadowsocks-rust 网络模式
```
仓库：https://github.com/shadowsocks/shadowsocks-rust
重点：Rust 异步网络编程模式
产出：Learning/27-shadowsocks-网络模式.md
问题：(1) 异步 I/O 架构？(2) 加密层设计？(3) 可复用的网络模式？
```

---

## 执行规则

1. **不要写代码**——只读文档，写报告
2. **每完成一个任务立即 git commit && git push**
3. 报告格式：项目名 + 核心发现 + 3-5 个具体可复用点 + 对 Hermes_Rust_Operit_App 的建议
4. 报告放在 `D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App\Learning\`
5. 文件命名：`<编号>-<英文名>-<中文描述>.md`
6. 用中文写报告，简洁直接
7. 读项目用 curl 拉 README + 关键源文件，不要试图 clone 或编译

---

## 项目上下文速查

```
本地路径：D:\Hermes_Agent_Desktop\Hermes_Download\Hermes_Rust_Operit_App
仓库：https://github.com/jinzechen/Hermes_Rust_Operit_App
已有学习报告：Learning/01-17 (17份)
模型：deepseek-v4-flash（已配置）
```

---

*生成时间：2026-07-18 | 生成者：deepseek-v4-pro*
