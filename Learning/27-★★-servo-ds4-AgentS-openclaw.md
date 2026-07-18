# servo + ds4 + Agent-S + openclaw：次要参考项目速查

> 以下项目有参考价值但非核心，简要记录关键点

---

## servo (37,413⭐, Rust)

浏览器引擎，可作为 obscura 之外的 Rust 纯浏览器方案参考。但 obscura（19K⭐）更成熟且已打包为 Operit_MCPS 插件。

**Hermes 评分**：★★

---

## ds4 (7⭐, C)

DeepSeek V4 Flash 本地推理引擎（Metal-only, 128GB RAM 起）。Android 上不现实。

**Hermes 评分**：★

---

## Agent-S (12,029⭐, Python)

计算机使用 Agent（OSWorld 72.6% 超人类）。架构（Observation→Planning→Execution）可借鉴，但 Python 代码无法直接使用。

**Hermes 评分**：★★

---

## openclaw (383,352⭐, TypeScript)

多供应商 AI Agent 平台。其 Provider 抽象和 skill 系统设计思路可参考，但 Hermes 已有 hermes-agent-rs 提供更成熟方案。

**Hermes 评分**：★★
