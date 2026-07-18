# Rust 网页抓取/搜索引擎生态

> **来源**：hermes-agent-rs 的 `tools/web.rs` 需要网页搜索+抓取能力  
> **Hermes_Rust_Operit_App 评分**：★★★★★（Agent 联网搜索核心）

---

## 一、Rust web 生态

| 项目 | Stars | 用途 | 在 Hermes 中的方案 |
|------|-------|------|-----------------|
| **reqwest** | 11K | HTTP 客户端 | ✅ 已有依赖 |
| **scraper** | 2K+ | HTML 解析（CSS 选择器） | `tools/web.rs` 的基础 |
| **select.rs** | 1K | HTML DOM 操作 | 可选替代 |
| **headless_chrome** | 2K+ | CDP 浏览器控制 | 类似 obscura |
| **fantoccini** | 1K+ | WebDriver 客户端 | 可选替代 |

---

## 二、实现方案

```rust
use reqwest::Client;
use scraper::{Html, Selector};

// 网页搜索+抓取
async fn web_search(query: &str) -> Result<Vec<String>> {
    let client = Client::new();
    // 1. 搜索（通过搜索引擎 API 或 HTML）
    let resp = client
        .get(format!("https://api.duckduckgo.com/?q={}", query))
        .send().await?;
    
    // 2. 解析 HTML
    let doc = Html::parse_document(&resp.text().await?);
    let selector = Selector::parse("a.result__a")?;
    
    // 3. 提取链接+文本
    let results: Vec<_> = doc.select(&selector)
        .map(|el| el.text().collect())
        .collect();
    Ok(results)
}
```

### 评分：★★★★★

reqwest + scraper 是 Rust 网页抓取的标准组合。Hermes_Rust_Operit_App 的 `tools/web.rs` 可直接使用。
