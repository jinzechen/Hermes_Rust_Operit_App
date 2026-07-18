# Activepieces MCP 注册表 — 学习报告
> 项目：https://github.com/activepieces/activepieces | 23.3k stars
> 学习日期：2026-07-18

## 核心发现
~400 pieces，60% 社区贡献。**每个 piece 同时是 MCP 服务器**。

## 注册表格式
```
packages/pieces/community/<name>/
  src/index.ts   → createPiece({displayName, description, categories, actions, triggers})
  package.json   → npm 元数据
```

## 对 Hermes Plugin Store 的价值
- 400 个 MCP 服务器的元数据源
- 标准化格式：displayName, categories(PieceCategory enum), logoUrl, authors, actions
- 可作为商店的**默认聚合源**

## 提取策略
Option A: GitHub tree API 扫描
Option B: npm registry 查询 @activepieces/piece-*
Option C: 组合丰富（解析 index.ts 提取完整 action schema）
