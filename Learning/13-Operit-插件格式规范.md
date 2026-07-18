# Operit 插件/Skill 格式规范 — 源码学习

> 来源：AAswordman/Operit/docs/
> 学习日期：2026-07-18

## Skill 格式 (SKILL.md)

```yaml
---
name: SkillName
description: 描述
---
# Markdown 正文 (AI 指令)
```

存储在 `/sdcard/Download/Operit/skills/<name>/SKILL.md`

## Script 脚本格式

TypeScript/JS 文件，顶部 `/* METADATA {...} */` 块：
```json
{
    "name": "ScriptName",
    "description": "...",
    "category": "Utility",
    "tools": [{
        "name": "tool_name",
        "description": "...",
        "parameters": [{"name":"p","type":"string","required":true}]
    }]
}
```

导出模式: `exports.tool_name = wrapper;`

## ToolPkg 格式 (`.toolpkg` = ZIP)

```
tool_name.toolpkg (ZIP)
├── manifest.json        # 必需: schema_version, toolpkg_id, version, main, subpackages, resources, ui, i18n
├── main.js              # 必需: 主入口
├── packages/            # 子包脚本
├── ui/                  # Compose DSL UI 模块
├── resources/           # 二进制/配置文件
└── i18n/                # 多语言
```

## MCP 连接

ToolPkg 中的 MCP 子进程通过 JSON-RPC stdio 通信，与我们的 McpClient 设计一致。

## 对 Hermes_Rust_Operit_App 的意义

1. `PluginStore` 已支持 SKILL.md 扫描 → ✅ 兼容
2. ToolPkg ZIP 格式 → PluginStore 需加 `.toolpkg` 解析
3. Script METADATA 格式 → 可作为 `ScriptTool` 的元数据源
4. MCP stdio 连接 → McpClient 已实现 ✅
