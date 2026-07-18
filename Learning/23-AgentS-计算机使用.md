# Agent-S 计算机使用 — 学习报告（含 UA Rust 分析）

> 仓库：https://github.com/simular-ai/Agent-S  
> Stars：12,029 | License：Apache-2.0 | 语言：Python  
> 核心：S3 版超越人类（OSWorld 72.6%），ICLR 2025

---

## 一、项目概况

Agent-S 是一个**计算机使用 Agent** 框架，通过截图观察 GUI 界面、规划动作、执行操作来完成桌面任务。OSWorld 基准达到 72.60%，是首个超越人类基线（70.41%）的纯 AI 系统。

---

## 二、架构分析

### S3 三层架构

```
1. Observation (截图 → 视觉理解)
   ├── 屏幕截图 → VLM 解析 → UI 元素识别
   └── 无障碍树 → 元素坐标提取

2. Planning (规划)
   ├── S2 Generalist: 全局规划（做什么）
   └── S2 Specialist: 具体执行（怎么做）

3. Execution (动作)
   ├── 鼠标点击/拖拽
   ├── 键盘输入
   └── 滚动等系统操作
```

### S2 双模型架构

| 角色 | 模型 | 职责 |
|------|------|------|
| **Generalist** | 大型 VLM | 理解截图，制定高层次计划 |
| **Specialist** | 中型 VLM | 将计划转化为具体的 UI 动作 |

### 扩展律（Scaling Law）

Agent-S 发现：**更多推理步骤 = 更好结果**
- 推理步骤从 1 增至 5，OSWorld 得分从 48% → 72.6%
- 但超过 5 步后收益递减（边际效用）

---

## 三、三个核心问题回答

### (1) 计算机使用 Agent 的架构？

```
循环: 截图 → 视觉理解 → 规划 → 执行 → 再截图...
                                ↓
S2 模型: Generalist(大局) + Specialist(动作)
```

### (2) 怎么做到超人类的？

三个方面：
1. **多步推理** — 不是一步到位，而是逐步观察 + 动作循环
2. **双模型分工** — Generalist 看大局，Specialist 做具体动作
3. **错误恢复** — 失败后重试，而非直接放弃

### (3) 能不能用在 Android 上？

**可以借鉴但不直接可用**：
- Agent-S 设计用于桌面 GUI（完整 OS）
- Android 需要 ADB + Accessibility Service 替代截图+鼠标
- 架构理念（Observation→Planning→Execution）可移植
- Hermes 已有的 `browser.rs` + `vision.rs` 可组成类似管线

---

## 四、对 Hermes_Rust_Operit_App 的参考价值

| 可借鉴点 | 说明 |
|----------|------|
| **观察-规划-执行循环** | Hermes Agent 的 tool calling 天然适配此模式 |
| **S2 双模型** | Generalist=Agent 决策层, Specialist=ToolHandler |
| **多步推理扩展律** | Hermes 应允许多步 tool calling 而非单步 |
| **错误恢复** | Agent 执行失败后应自动重试/回退 |

---

## 五、三个可复用点

1. **循环架构** — Observation→Planning→Execution 循环可嵌入 Hermes Agent 的 tool use 循环
2. **S2 分工** — Agent (Generalist) + ToolHandler (Specialist) 已天然实现
3. **扩展律数据** — 5 步推理是最优的实证经验
