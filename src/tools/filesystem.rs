//! Filesystem tool — safe read/write/list operations within allowed directories.
//!
//! Implements [`ToolHandler`] for the following operations:
//! - `read_file`      — read file contents as string
//! - `write_file`     — write (overwrite) a file
//! - `list_directory` — list entries in a directory
//! - `search_files`   — regex search inside files
//! - `get_file_info`  — metadata (size, modified, permissions)
//! - `create_directory` — mkdir -p
//! - `delete_file`    — remove a file (not directories by default)
//! - `move_file`      — rename / move a file

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use regex::Regex;
use walkdir::WalkDir;

use super::{ToolHandler, ToolInfo, ToolResult};

/// Allowed base directories.  Every path must resolve inside one of these.
/// An empty set means **all paths are allowed** (use with caution).
#[derive(Clone)]
pub struct FileSystemTool {
    allowed_dirs: HashSet<PathBuf>,
}

impl FileSystemTool {
    /// Create a new filesystem tool with the given allowed directories.
    ///
    /// Pass an empty vec for unrestricted access (not recommended).
    pub fn new(allowed_dirs: Vec<PathBuf>) -> Self {
        Self {
            allowed_dirs: allowed_dirs
                .into_iter()
                .map(|p| p.canonicalize().unwrap_or(p))
                .collect(),
        }
    }

    /// Resolve and validate a user-supplied path.
    fn resolve(&self, user_path: &str) -> Result<PathBuf> {
        let path = Path::new(user_path);

        // If absolute, canonicalize.
        let resolved = if path.is_absolute() {
            path.canonicalize()
                .with_context(|| format!("cannot resolve path: {}", user_path))?
        } else {
            // Relative paths are resolved against current-dir.
            let abs = std::env::current_dir()?.join(path);
            abs.canonicalize()
                .with_context(|| format!("cannot resolve path: {}", user_path))?
        };

        // Validate.
        self.validate(&resolved)?;
        Ok(resolved)
    }

    /// Ensure `path` lives inside (or equals) an allowed directory.
    fn validate(&self, path: &Path) -> Result<()> {
        if self.allowed_dirs.is_empty() {
            return Ok(()); // unrestricted
        }
        for allowed in &self.allowed_dirs {
            if path.starts_with(allowed) || path == allowed {
                return Ok(());
            }
        }
        bail!(
            "path '{}' is outside allowed directories: {:?}",
            path.display(),
            self.allowed_dirs
        );
    }

    // ── operation helpers ─────────────────────────────────────────

    fn op_read_file(&self, args: &serde_json::Value) -> Result<String> {
        let path_str = get_string_arg(args, "path")?;
        let p = self.resolve(&path_str)?;
        let content =
            fs::read_to_string(&p).with_context(|| format!("cannot read {}", p.display()))?;

        let lines: Vec<&str> = content.lines().collect();
        let numbered: String = lines
            .iter()
            .enumerate()
            .map(|(i, l)| format!("{:>6}| {}", i + 1, l))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!(
            "## {}\n\n```\n{}\n```\n\n**{} lines**",
            p.display(),
            numbered,
            lines.len()
        ))
    }

    fn op_write_file(&self, args: &serde_json::Value) -> Result<String> {
        let path_str = get_string_arg(args, "path")?;
        let content = get_string_arg(args, "content")?;
        let p = self.resolve(&path_str)?;

        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&p, &content).with_context(|| format!("cannot write {}", p.display()))?;

        Ok(format!(
            "## Written\n\n`{}` — {} bytes",
            p.display(),
            content.len()
        ))
    }

    fn op_list_directory(&self, args: &serde_json::Value) -> Result<String> {
        let path_str = get_string_arg(args, "path")?;
        let p = self.resolve(&path_str)?;

        let mut entries: Vec<String> = Vec::new();
        for entry in fs::read_dir(&p)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            let prefix = if meta.is_dir() { "📁" } else { "📄" };
            entries.push(format!(
                "{} {}  ({})",
                prefix,
                entry.file_name().to_string_lossy(),
                format_size(meta.len())
            ));
        }
        entries.sort();

        Ok(format!(
            "## {}\n\n{}",
            p.display(),
            if entries.is_empty() {
                "(empty)".into()
            } else {
                entries.join("\n")
            }
        ))
    }

    fn op_search_files(&self, args: &serde_json::Value) -> Result<String> {
        let dir_str = get_string_arg(args, "directory")?;
        let pattern = get_string_arg(args, "pattern")?;
        let dir = self.resolve(&dir_str)?;

        let re = Regex::new(&pattern).with_context(|| format!("invalid regex: {}", pattern))?;

        let mut results: Vec<String> = Vec::new();
        for entry in WalkDir::new(&dir)
            .max_depth(20)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for (line_no, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        results.push(format!(
                            "{}:{}: {}",
                            entry.path().display(),
                            line_no + 1,
                            line.trim()
                        ));
                    }
                }
            }
        }

        Ok(format!(
            "## Search: `{}` in {}\n\n{} matches\n\n{}",
            pattern,
            dir.display(),
            results.len(),
            results.join("\n")
        ))
    }

    fn op_get_file_info(&self, args: &serde_json::Value) -> Result<String> {
        let path_str = get_string_arg(args, "path")?;
        let p = self.resolve(&path_str)?;
        let meta = fs::metadata(&p)?;

        let kind = if meta.is_dir() {
            "directory"
        } else if meta.is_symlink() {
            "symlink"
        } else {
            "file"
        };
        let modified = meta
            .modified()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|_| "unknown".into());

        Ok(format!(
            "## {}\n\n- Kind: {}\n- Size: {}\n- Modified: {}\n- Read-only: {}",
            p.display(),
            kind,
            format_size(meta.len()),
            modified,
            meta.permissions().readonly()
        ))
    }

    fn op_create_directory(&self, args: &serde_json::Value) -> Result<String> {
        let path_str = get_string_arg(args, "path")?;
        let p = self.resolve(&path_str)?;
        fs::create_dir_all(&p)?;
        Ok(format!("## Created\n\n`{}`", p.display()))
    }

    fn op_delete_file(&self, args: &serde_json::Value) -> Result<String> {
        let path_str = get_string_arg(args, "path")?;
        let p = self.resolve(&path_str)?;
        let meta = fs::metadata(&p)?;
        if meta.is_dir() {
            bail!("cannot delete a directory with delete_file; use a directory-aware tool");
        }
        fs::remove_file(&p)?;
        Ok(format!("## Deleted\n\n`{}`", p.display()))
    }

    fn op_move_file(&self, args: &serde_json::Value) -> Result<String> {
        let src_str = get_string_arg(args, "source")?;
        let dst_str = get_string_arg(args, "destination")?;
        let src = self.resolve(&src_str)?;
        let dst = self.resolve(&dst_str)?;
        fs::rename(&src, &dst)?;
        Ok(format!(
            "## Moved\n\n`{}` → `{}`",
            src.display(),
            dst.display()
        ))
    }
}

#[async_trait]
impl ToolHandler for FileSystemTool {
    fn name(&self) -> &str {
        "filesystem"
    }

    fn info(&self) -> ToolInfo {
        ToolInfo {
            name: "filesystem".into(),
            description: "Read, write, list, search, and manage files within allowed directories"
                .into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": [
                            "read_file", "write_file", "list_directory",
                            "search_files", "get_file_info", "create_directory",
                            "delete_file", "move_file"
                        ]
                    },
                    "path": { "type": "string", "description": "File or directory path" },
                    "content": { "type": "string", "description": "Content for write operations" },
                    "pattern": { "type": "string", "description": "Regex for search_files" },
                    "source": { "type": "string", "description": "Source path for move_file" },
                    "destination": { "type": "string", "description": "Destination path for move_file" }
                },
                "required": ["operation"]
            }),
        }
    }

    async fn execute(&self, arguments: serde_json::Value) -> ToolResult {
        let op = arguments
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let result: Result<String> = match op {
            "read_file" => self.op_read_file(&arguments),
            "write_file" => self.op_write_file(&arguments),
            "list_directory" => self.op_list_directory(&arguments),
            "search_files" => self.op_search_files(&arguments),
            "get_file_info" => self.op_get_file_info(&arguments),
            "create_directory" => self.op_create_directory(&arguments),
            "delete_file" => self.op_delete_file(&arguments),
            "move_file" => self.op_move_file(&arguments),
            _ => bail!("unknown filesystem operation: '{}'", op),
        };

        match result {
            Ok(content) => ToolResult {
                content,
                is_error: false,
            },
            Err(e) => ToolResult {
                content: format!("**Error:** {}", e),
                is_error: true,
            },
        }
    }
}

// ── helpers ────────────────────────────────────────────────────────

fn get_string_arg(args: &serde_json::Value, key: &str) -> Result<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("missing or invalid argument: '{}'", key))
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut idx = 0;
    while size >= 1024.0 && idx + 1 < UNITS.len() {
        size /= 1024.0;
        idx += 1;
    }
    format!("{:.1} {}", size, UNITS[idx])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0.0 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1_048_576), "1.0 MB");
    }

    #[test]
    fn test_read_write_roundtrip() {
        let dir = std::env::temp_dir().join("hermes_fs_test");
        let _ = fs::create_dir_all(&dir);

        let tool = FileSystemTool::new(vec![dir.clone()]);
        let test_file = dir.join("test.txt");

        // write
        let args = serde_json::json!({
            "operation": "write_file",
            "path": test_file.to_str().unwrap(),
            "content": "hello\nworld"
        });
        let res = futures::executor::block_on(tool.execute(args));
        assert!(!res.is_error);

        // read
        let args = serde_json::json!({
            "operation": "read_file",
            "path": test_file.to_str().unwrap(),
        });
        let res = futures::executor::block_on(tool.execute(args));
        assert!(!res.is_error);
        assert!(res.content.contains("hello"));
        assert!(res.content.contains("world"));

        // cleanup
        let _ = fs::remove_file(&test_file);
        let _ = fs::remove_dir(&dir);
    }

    #[test]
    fn test_path_validation_outside_denied() {
        let tool = FileSystemTool::new(vec![std::env::temp_dir()]);
        let bad = Path::new("/etc/passwd");
        let err = tool.validate(bad);
        assert!(err.is_err());
    }
}
