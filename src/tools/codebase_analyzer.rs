//! CodebaseAnalyzer — wraps Understand_Anything_Rust as a Hermes ToolHandler.
//!
//! Enables `/understand`-like functionality: scan code, build knowledge graph,
//! and generate HTML/Markdown reports — all from within the Agent.

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use serde_json::{json, Value};

use crate::core::tool_registry::{ToolHandler, ToolSchema};

/// Tool that analyzes a codebase and returns a knowledge graph + report.
pub struct CodebaseAnalyzer;

impl CodebaseAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl ToolHandler for CodebaseAnalyzer {
    fn execute(&self, params: Value) -> Result<String> {
        let path_str = params["path"].as_str().unwrap_or(".");
        let format = params["format"].as_str().unwrap_or("json");
        let root = Path::new(path_str);

        // Phase 1: Scan
        let scan = ua_core::scanner::scan_project(root)
            .map_err(|e| anyhow::anyhow!("Scan failed: {}", e))?;

        // Phase 2: Parse
        let registry = ua_core::parser::ParserRegistry::default();
        let mut parsed = Vec::new();
        for file in &scan.files {
            if file.file_category == ua_core::types::FileCategory::Code {
                if let Ok(p) = registry.parse(&root.join(&file.path)) {
                    parsed.push(p);
                }
            }
        }

        // Phase 3: Build graph
        let graph = ua_core::graph::build_graph(root, &scan, &parsed);

        match format {
            "html" => {
                let html = ua_core::report::to_html(&graph);
                Ok(html)
            }
            "md" | "markdown" => {
                let md = ua_core::report::to_markdown(&graph);
                Ok(md)
            }
            _ => {
                let json_str = serde_json::to_string_pretty(&graph)
                    .unwrap_or_else(|_| "{}".to_string());
                Ok(json_str)
            }
        }
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "analyze_codebase".to_string(),
            description: "分析代码库，生成知识图谱和可读报告。输入项目路径，输出 JSON 图谱 / HTML 网页 / Markdown 文档。".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "项目根目录路径（默认当前目录）"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["json", "html", "md"],
                        "description": "输出格式：json（知识图谱）、html（交互式网页）、md（Markdown文档），默认 json"
                    }
                },
                "required": []
            }),
        }
    }
}

/// Factory function for registering the tool.
pub fn register(tool_registry: &mut crate::core::tool_registry::ToolRegistry) {
    tool_registry.register("analyze_codebase", Arc::new(CodebaseAnalyzer::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_format() {
        let analyzer = CodebaseAnalyzer::new();
        let schema = analyzer.schema();
        assert_eq!(schema.name, "analyze_codebase");
        assert!(schema.description.contains("知识图谱"));
    }
}
