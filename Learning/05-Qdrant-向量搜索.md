# Qdrant 向量搜索 — 学习报告
> 项目：https://github.com/qdrant/qdrant | Rust 原生向量数据库
> 学习日期：2026-07-18

## 核心数据模型
Collection → Segments → Points (id + vector + payload)
- HNSW 索引：亚线性 ANN 搜索
- Payload 索引：过滤 + 搜索融合（单次遍历）
- 量化：Scalar(4x) / Product / Binary(32x)
- 分片 + 复制 + Raft 共识

## API 设计（统一查询入口）
POST /collections/{name}/points/query
{query: [vector], filter: {must/should/must_not}, limit, prefetch: [dense, sparse]}

## 对 Hermes 的整合
**Proposed Schema**: Collection agent_memory
Payload: text, summary, agent_id, session_id, type, timestamp, importance, tags

**四种检索模式**:
1. 语义召回：query by embedding, filter by agent_id+type
2. 事实查询：query by embedding, filter by agent_id+type="fact"
3. 时间加权：filter by timestamp range
4. 混合搜索：prefetch [dense, sparse], RRF 融合

**整合复杂度**：低。Rust native SDK，单 Docker 命令启动。
