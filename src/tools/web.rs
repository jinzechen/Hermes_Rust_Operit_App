//! Web tool — DuckDuckGo search and page fetching with caching.
//!
//! Uses `reqwest` (blocking) + `scraper` for HTML parsing and
//! integrates `moka` for in-memory caching of search and fetch results.

use anyhow::{bail, Context};
use scraper::{Html, Selector};
use std::time::Duration;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

/// Moka synchronous cache for search and fetch results.
type Cache<K, V> = moka::sync::Cache<K, V>;

pub struct WebTool {
    /// Cache search results keyed by query string (TTL: 5 min).
    search_cache: Cache<String, serde_json::Value>,
    /// Cache fetched page content keyed by URL (TTL: 10 min).
    fetch_cache: Cache<String, String>,
    http_client: reqwest::blocking::Client,
}

impl WebTool {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("HermesAgent/1.0 (AI assistant; +https://hermes-agent.nousresearch.com)")
            .timeout(Duration::from_secs(20))
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self {
            search_cache: Cache::builder()
                .time_to_live(Duration::from_secs(300)) // 5 min
                .max_capacity(500)
                .build(),
            fetch_cache: Cache::builder()
                .time_to_live(Duration::from_secs(600)) // 10 min
                .max_capacity(200)
                .build(),
            http_client: client,
        })
    }
}

impl ToolHandler for WebTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "web".into(),
            description: "Web search via DuckDuckGo and page content fetching with caching".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["search", "fetch"]
                    },
                    "query": {
                        "type": "string",
                        "description": "Search query (for search action)"
                    },
                    "url": {
                        "type": "string",
                        "description": "URL to fetch (for fetch action)"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum number of search results (default: 10)",
                        "default": 10
                    }
                },
                "required": ["action"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match action {
            "search" => self.op_search(&arguments),
            "fetch" => self.op_fetch(&arguments),
            _ => bail!("unknown web action: '{}'", action),
        }
    }
}

// ── operation implementations ─────────────────────────────────────

impl WebTool {
    fn op_search(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("missing 'query' argument for search"))?;

        let max_results = args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(10)
            .min(20) as usize;

        // Check cache first.
        let cache_key = format!("q:{}:{}", query, max_results);
        if let Some(cached) = self.search_cache.get(&cache_key) {
            return Ok(format!(
                "## Web Search (cached)\n\nQuery: {}\n\n{}",
                query,
                format_search_results(&cached)
            ));
        }

        // Build DuckDuckGo HTML search URL.
        let search_url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding(&query)
        );

        let resp = self
            .http_client
            .get(&search_url)
            .send()
            .with_context(|| format!("DuckDuckGo search request failed for: {}", query))?;

        let body = resp.text().context("failed to read search response body")?;

        let document = Html::parse_document(&body);

        // Parse DDG HTML results: each result is a div.result with
        // a.result__a for the title/URL and a.result__snippet for the snippet.
        let result_sel = Selector::parse(".result").unwrap();
        let title_sel = Selector::parse("a.result__a").unwrap();
        let snippet_sel = Selector::parse("a.result__snippet").unwrap();

        let mut results: Vec<serde_json::Value> = Vec::new();
        for result_elem in document.select(&result_sel) {
            if results.len() >= max_results {
                break;
            }

            let title = result_elem
                .select(&title_sel)
                .next()
                .map(|e| e.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            let url = result_elem
                .select(&title_sel)
                .next()
                .and_then(|e| e.value().attr("href"))
                .map(|h| {
                    // DDG wraps URLs: //duckduckgo.com/l/?uddg=REAL_URL&...
                    if let Some(start) = h.find("uddg=") {
                        let rest = &h[start + 5..];
                        let end = rest.find('&').unwrap_or(rest.len());
                        urlencoding::decode(&rest[..end]).unwrap_or_else(|_| h.to_string())
                    } else {
                        h.to_string()
                    }
                })
                .unwrap_or_default();

            let snippet = result_elem
                .select(&snippet_sel)
                .next()
                .map(|e| e.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            if !title.is_empty() || !url.is_empty() {
                results.push(serde_json::json!({
                    "title": title,
                    "url": url,
                    "snippet": snippet
                }));
            }
        }

        // If DDG HTML parsing didn't yield results, fall back to raw link extraction.
        if results.is_empty() {
            let link_sel = Selector::parse("a[href]").unwrap();
            for link in document.select(&link_sel) {
                if results.len() >= max_results {
                    break;
                }
                if let Some(href) = link.value().attr("href") {
                    if href.starts_with("http") && !href.contains("duckduckgo.com") {
                        let title = link.text().collect::<Vec<_>>().join(" ").trim().to_string();
                        results.push(serde_json::json!({
                            "title": title,
                            "url": href,
                            "snippet": ""
                        }));
                    }
                }
            }
        }

        let result_json = serde_json::json!({ "results": results, "query": query });

        // Store in cache.
        self.search_cache.insert(cache_key, result_json.clone());

        Ok(format!(
            "## Web Search\n\nQuery: {}\nResults: {}\n\n{}",
            query,
            results.len(),
            format_search_results(&result_json)
        ))
    }

    fn op_fetch(&self, args: &serde_json::Value) -> anyhow::Result<String> {
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("missing 'url' argument for fetch"))?;

        // Check cache.
        if let Some(cached) = self.fetch_cache.get(url) {
            return Ok(cached);
        }

        let resp = self
            .http_client
            .get(url)
            .send()
            .with_context(|| format!("HTTP GET failed for: {}", url))?;

        let status = resp.status();
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let body = resp.text().context("failed to read response body")?;

        let is_html = content_type.contains("html");

        let (extracted, title) = if is_html {
            let document = Html::parse_document(&body);
            let title = document
                .select(&Selector::parse("title").unwrap())
                .next()
                .map(|e| e.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();

            // Extract text from body, removing scripts and styles.
            let body_text = {
                // Remove script/style tags by selecting body text only.
                let body_sel = Selector::parse("body").unwrap();
                document
                    .select(&body_sel)
                    .next()
                    .map(|b| {
                        b.text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .split_whitespace()
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .unwrap_or_default()
            };

            let truncated: String = body_text.chars().take(8000).collect();
            let suffix = if body_text.len() > 8000 { "…" } else { "" };

            (format!("{}{}", truncated, suffix), title)
        } else {
            let truncated: String = body.chars().take(4000).collect();
            let suffix = if body.len() > 4000 { "…" } else { "" };
            (format!("{}{}", truncated, suffix), String::new())
        };

        let output = format!(
            "## Fetch\n\nURL: {}\nStatus: {}\nContent-Type: {}\nTitle: {}\nSize: {} bytes\n\n{}\n",
            url,
            status.as_u16(),
            content_type,
            title,
            body.len(),
            extracted
        );

        // Cache the result.
        self.fetch_cache.insert(url.to_string(), output.clone());

        Ok(output)
    }
}

// ── helpers ───────────────────────────────────────────────────────

/// Simple URL encoding for search query parameters.
fn urlencoding(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => result.push('+'),
            _ => {
                result.push('%');
                result.push(HEX_CHARS[(byte >> 4) as usize]);
                result.push(HEX_CHARS[(byte & 0x0F) as usize]);
            }
        }
    }
    result
}

const HEX_CHARS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

/// Format search results as readable text.
fn format_search_results(json: &serde_json::Value) -> String {
    let results = json
        .get("results")
        .and_then(|v| v.as_array())
        .map(|a| a.as_slice())
        .unwrap_or(&[]);

    if results.is_empty() {
        return "(no results)".into();
    }

    let mut out = String::new();
    for (i, r) in results.iter().enumerate() {
        let title = r.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let url = r.get("url").and_then(|v| v.as_str()).unwrap_or("");
        let snippet = r.get("snippet").and_then(|v| v.as_str()).unwrap_or("");

        out.push_str(&format!("{}. **{}**\n", i + 1, title));
        out.push_str(&format!("   {}\n", url));
        if !snippet.is_empty() {
            out.push_str(&format!("   {}\n", snippet));
        }
        out.push('\n');
    }
    out
}

// ── URL decode helper (for DDG redirect URLs) ─────────────────────

mod urlencoding {
    pub fn decode(input: &str) -> Result<String, std::string::FromUtf8Error> {
        let mut bytes = Vec::with_capacity(input.len());
        let mut chars = input.bytes();
        while let Some(b) = chars.next() {
            match b {
                b'%' => {
                    let hi = chars.next().unwrap_or(b'0');
                    let lo = chars.next().unwrap_or(b'0');
                    let byte = (hex_val(hi) << 4) | hex_val(lo);
                    bytes.push(byte);
                }
                b'+' => bytes.push(b' '),
                _ => bytes.push(b),
            }
        }
        String::from_utf8(bytes)
    }

    fn hex_val(b: u8) -> u8 {
        match b {
            b'0'..=b'9' => b - b'0',
            b'A'..=b'F' => b - b'A' + 10,
            b'a'..=b'f' => b - b'a' + 10,
            _ => 0,
        }
    }
}
