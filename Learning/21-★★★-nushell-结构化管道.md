# 16 — nushell：结构化管道参考

> **仓库**：https://github.com/nushell/nushell (40,036⭐, Rust)  
> **核心价值**：结构化数据管道，Agent 工具链的输入输出格式参考  
> **Hermes_Rust_Operit_App 评分**：★★★（工具链设计参考）

---

## 核心设计

```bash
# 传统 Shell：纯文本管道
ls | grep foo | awk '{print $1}'

# Nushell：结构化管道
ls | where name =~ "foo" | select name size
```

### 对 Hermes 的作用

Agent 工具的输入输出可以借鉴 Nushell 的结构化数据理念。Hermes 的 tool_registry 已支持 JSON Schema 格式。
