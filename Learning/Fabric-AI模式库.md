# Fabric：255 个 AI Prompt 模式分析

> **仓库**：https://github.com/danielmiessler/fabric (43,117⭐, Go)  
> **核心**：255 个社区贡献的 AI prompt 模板 + 9 种执行策略  
> **Hermes_Rust_Operit_App 评分**：★★★（patterns 可借鉴，不是核心）

---

## 一、Pattern 格式

```
data/patterns/<name>/
├── system.md    — AI 系统提示（角色定义+输出格式+指令）
└── user.md      — 用户输入模板（可选）
```

示例（summarize pattern）：

```markdown
# IDENTITY and PURPOSE
You are an expert content summarizer...

# OUTPUT SECTIONS
- ONE SENTENCE SUMMARY: ...
- MAIN POINTS: ...
- TAKEAWAYS: ...

# OUTPUT INSTRUCTIONS
- You only output human readable Markdown.
- Do not output warnings or notes.

# INPUT:
INPUT:
```

---

## 二、对 Hermes_Rust_Operit_App 的作用

| 能力 | 说明 | 优先级 |
|------|------|--------|
| pattern 格式 | SKILL.md 可借鉴其 system.md 结构 | ★★★★ |
| 255 个模板 | 高价值 pattern（summarize/review_code）可直接移植 | ★★★ |
| 9 种策略 | CoT/AoT/ToT/Reflexion 可嵌入 Hermes 推理层 | ★★★ |

### 评分：★★★

Fabric 的 pattern 格式和 255 个模板是高质量资源，但 hermes-agent-rs 已有 skill 系统，pattern 可批量导入。
