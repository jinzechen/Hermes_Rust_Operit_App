//! Markdown tool — renders Markdown to terminal-formatted text,
//! extracts code blocks, and applies basic syntax highlighting hints.
//!
//! Implements [`ToolHandler`].

use anyhow::bail;
use regex::Regex;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

pub struct MarkdownTool;

impl MarkdownTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MarkdownTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolHandler for MarkdownTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "markdown".into(),
            description: "Render Markdown to terminal text, extract code blocks".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["render", "extract_code", "strip_formatting"]
                    },
                    "text": { "type": "string", "description": "Markdown source text" },
                    "language": {
                        "type": "string",
                        "description": "Filter code blocks by language (for extract_code)"
                    }
                },
                "required": ["action", "text"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let text = arguments.get("text").and_then(|v| v.as_str()).unwrap_or("");

        match action {
            "render" => Ok(render_markdown(text)),
            "extract_code" => {
                let lang_filter = arguments.get("language").and_then(|v| v.as_str());
                let blocks = extract_code_blocks(text, lang_filter);
                Ok(blocks.join("\n\n---\n\n"))
            }
            "strip_formatting" => Ok(strip_markdown_formatting(text)),
            _ => bail!("unknown markdown action: '{}'", action),
        }
    }
}

/// Convert Markdown text to terminal-friendly plain text.
pub fn render_markdown(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_lines: Vec<String> = Vec::new();

    for line in input.lines() {
        // Fenced code blocks.
        if line.trim_start().starts_with("```") {
            if in_code_block {
                // End of code block.
                out.push_str(&render_code_block(&code_lang, &code_lines));
                out.push('\n');
                code_lines.clear();
                code_lang.clear();
                in_code_block = false;
            } else {
                // Start of code block.
                let lang = line.trim_start().strip_prefix("```").unwrap_or("").trim();
                code_lang = lang.to_string();
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            code_lines.push(line.to_string());
            continue;
        }

        // Headings.
        if let Some(heading) = line.strip_prefix("###### ") {
            out.push_str(&format!("  {}\n", heading));
        } else if let Some(heading) = line.strip_prefix("##### ") {
            out.push_str(&format!("  {}\n", heading));
        } else if let Some(heading) = line.strip_prefix("#### ") {
            out.push_str(&format!("  {}\n", heading));
        } else if let Some(heading) = line.strip_prefix("### ") {
            out.push_str(&format!("## {}\n", heading));
        } else if let Some(heading) = line.strip_prefix("## ") {
            out.push_str(&format!("## {}\n", heading));
        } else if let Some(heading) = line.strip_prefix("# ") {
            out.push_str(&format!("# {}\n", heading));
        }
        // Blockquote.
        else if let Some(quoted) = line.strip_prefix("> ") {
            out.push_str(&format!("  │ {}\n", quoted));
        }
        // Unordered list.
        else if let Some(item) = line
            .strip_prefix("- ")
            .or_else(|| line.strip_prefix("* "))
            .or_else(|| line.strip_prefix("+ "))
        {
            out.push_str(&format!("  • {}\n", item));
        }
        // Ordered list (simplistic — matches "1. ", "12. ", etc.)
        else if let Some(captures) = Regex::new(r"^(\d+)\.\s+(.*)").unwrap().captures(line) {
            let num = &captures[1];
            let item = &captures[2];
            out.push_str(&format!("  {}. {}\n", num, item));
        }
        // Horizontal rule.
        else if line.trim() == "---" || line.trim() == "***" || line.trim() == "___" {
            out.push_str("──────────────────────────────\n");
        }
        // Regular paragraph.
        else {
            // Strip basic inline formatting.
            let cleaned = strip_inline_formatting(line);
            if !cleaned.trim().is_empty() {
                out.push_str(&cleaned);
                out.push('\n');
            } else {
                out.push('\n');
            }
        }
    }

    // Unclosed code block at EOF.
    if in_code_block && !code_lines.is_empty() {
        out.push_str(&render_code_block(&code_lang, &code_lines));
        out.push('\n');
    }

    out
}

/// Render a code block with a language label and basic indentation.
fn render_code_block(lang: &str, lines: &[String]) -> String {
    let mut out = String::new();
    if !lang.is_empty() {
        out.push_str(&format!("  [{}]\n", lang));
    }
    out.push_str("  ┌─\n");
    for line in lines {
        out.push_str(&format!("  │ {}\n", line));
    }
    out.push_str("  └─");
    out
}

/// Strip basic inline Markdown formatting: bold, italic, code, links.
fn strip_inline_formatting(line: &str) -> String {
    // Bold + italic (***)
    let re_bold_italic = Regex::new(r"\*\*\*(.+?)\*\*\*").unwrap();
    let line = re_bold_italic.replace_all(line, "$1").to_string();
    // Bold (**)
    let re_bold = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    let line = re_bold.replace_all(&line, "$1").to_string();
    // Italic (* or _)
    let re_italic_star = Regex::new(r"\*(.+?)\*").unwrap();
    let line = re_italic_star.replace_all(&line, "$1").to_string();
    let re_italic_underscore = Regex::new(r"_(.+?)_").unwrap();
    let line = re_italic_underscore.replace_all(&line, "$1").to_string();
    // Inline code
    let re_code = Regex::new(r"`(.+?)`").unwrap();
    let line = re_code.replace_all(&line, "$1").to_string();
    // Links: [text](url)
    let re_link = Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap();
    let line = re_link.replace_all(&line, "$1").to_string();
    // Images: ![alt](url)
    let re_img = Regex::new(r"!\[([^\]]*)\]\([^)]+\)").unwrap();
    re_img.replace_all(&line, "[IMG]").to_string()
}

/// Extract fenced code blocks from Markdown text.
pub fn extract_code_blocks(input: &str, language_filter: Option<&str>) -> Vec<String> {
    let mut blocks: Vec<String> = Vec::new();
    let mut in_block = false;
    let mut current_lang = String::new();
    let mut current_lines: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.trim_start().starts_with("```") {
            if in_block {
                // End block.
                let lang_matches = match language_filter {
                    Some(f) => current_lang.eq_ignore_ascii_case(f),
                    None => true,
                };
                if lang_matches {
                    blocks.push(current_lines.join("\n"));
                }
                current_lines.clear();
                current_lang.clear();
                in_block = false;
            } else {
                current_lang = line
                    .trim_start()
                    .strip_prefix("```")
                    .unwrap_or("")
                    .trim()
                    .to_string();
                in_block = true;
            }
            continue;
        }

        if in_block {
            current_lines.push(line.to_string());
        }
    }

    // Unclosed block at end.
    if in_block {
        let lang_matches = match language_filter {
            Some(f) => current_lang.eq_ignore_ascii_case(f),
            None => true,
        };
        if lang_matches {
            blocks.push(current_lines.join("\n"));
        }
    }

    blocks
}

/// Remove all Markdown formatting, leaving plain text.
pub fn strip_markdown_formatting(input: &str) -> String {
    let mut out = String::new();
    let mut in_code = false;
    for line in input.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let cleaned = strip_inline_formatting(line);
        // Also strip heading markers.
        let cleaned = cleaned
            .trim_start_matches('#')
            .trim_start_matches('>')
            .trim_start()
            .trim_start_matches('-')
            .trim_start()
            .trim_start_matches('*')
            .trim_start()
            .trim_start_matches('+');
        if !cleaned.is_empty() {
            out.push_str(cleaned);
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_headings() {
        let md = "# Title\n\n## Section\n\n### Sub\n\nSome text.";
        let rendered = render_markdown(md);
        assert!(rendered.contains("# Title"));
        assert!(rendered.contains("## Section"));
        assert!(rendered.contains("## Sub"));
        assert!(rendered.contains("Some text."));
    }

    #[test]
    fn test_render_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let rendered = render_markdown(md);
        assert!(rendered.contains("[rust]"));
        assert!(rendered.contains("fn main() {}"));
    }

    #[test]
    fn test_extract_code_blocks() {
        let md = "```python\nprint('hi')\n```\n\n```rust\nfn main() {}\n```";
        let all = extract_code_blocks(md, None);
        assert_eq!(all.len(), 2);
        assert_eq!(all[0], "print('hi')");

        let rust = extract_code_blocks(md, Some("rust"));
        assert_eq!(rust.len(), 1);
        assert_eq!(rust[0], "fn main() {}");
    }

    #[test]
    fn test_strip_formatting() {
        let md = "**bold** and *italic* and `code` and [link](url)";
        let stripped = strip_markdown_formatting(md);
        assert_eq!(stripped.trim(), "bold and italic and code and link");
    }
}
