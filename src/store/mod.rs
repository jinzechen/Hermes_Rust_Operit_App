//! PluginStore — manages Skills, MCP server plugins, and external plugin packages.
//!
//! Skills are directories containing a `SKILL.md` manifest.
//! MCP servers are described by JSON config files.
//! External plugins are downloaded from the store (GitHub Releases), installed
//! into `~/.hermes/plugins/`, and managed via the plugin manager.

pub mod index;
pub mod installer;
pub mod manager;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};

// Re-export the store module types for convenient access.
pub use index::{
    builtin_index, fetch_index, find_plugin, load_cache, load_cached_or_builtin, save_cache,
    PluginIndex, StoreIndex,
};
pub use installer::{download_plugin, extract_zip, install_plugin, verify_checksum};
pub use manager::{
    check_update, disable_plugin, enable_plugin, install, list_installed, uninstall_plugin,
    upgrade_plugin, PluginState,
};

// ── domain types ───────────────────────────────────────────────────

/// Metadata for an installed skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub path: PathBuf,
    pub enabled: bool,
}

/// Metadata for an installed MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpInfo {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub enabled: bool,
}

/// MCP server config on disk (JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

// ── PluginStore ────────────────────────────────────────────────────

/// Manages skill and MCP plugins stored under a base directory.
///
/// Expected layout:
/// ```text
/// base_path/
///   skills/
///     <skill-name>/
///       SKILL.md
///       ...
///   mcp/
///     <server-name>.json
/// ```
pub struct PluginStore {
    base_path: PathBuf,
    skills_dir: PathBuf,
    mcp_dir: PathBuf,
}

impl PluginStore {
    /// Create a new PluginStore rooted at `base_path`.
    /// Creates the `skills/` and `mcp/` subdirectories if needed.
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self> {
        let base = base_path.into();
        let skills_dir = base.join("skills");
        let mcp_dir = base.join("mcp");

        fs::create_dir_all(&skills_dir)?;
        fs::create_dir_all(&mcp_dir)?;

        Ok(Self {
            base_path: base,
            skills_dir,
            mcp_dir,
        })
    }

    /// Return the base path.
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    // ── skills ──────────────────────────────────────────────────

    /// Scan `skills/` for directories containing `SKILL.md`, parse and return info.
    pub fn list_skills(&self) -> Result<Vec<SkillInfo>> {
        let mut skills = Vec::new();
        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        for entry in fs::read_dir(&self.skills_dir)? {
            let entry = entry?;
            let dir = entry.path();
            if !dir.is_dir() {
                continue;
            }
            let manifest = dir.join("SKILL.md");
            if manifest.exists() {
                if let Some(info) = self.parse_skill_manifest(&manifest, &dir) {
                    skills.push(info);
                }
            }
        }

        skills.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(skills)
    }

    /// Install a skill from `path` (copied into `skills/<name>/`).
    ///
    /// `path` should point to a skill directory containing `SKILL.md`.
    pub fn install_skill(&self, path: impl AsRef<Path>) -> Result<SkillInfo> {
        let src = path.as_ref();
        if !src.is_dir() {
            bail!("skill source is not a directory: {}", src.display());
        }

        let manifest = src.join("SKILL.md");
        if !manifest.exists() {
            bail!("no SKILL.md found in skill directory: {}", src.display());
        }

        // Derive name from directory name.
        let name = src
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unnamed")
            .to_string();

        let dst = self.skills_dir.join(&name);
        if dst.exists() {
            bail!("skill '{}' is already installed at {}", name, dst.display());
        }

        copy_dir_all(src, &dst)?;
        let info = self
            .parse_skill_manifest(&dst.join("SKILL.md"), &dst)
            .ok_or_else(|| anyhow!("failed to parse installed skill manifest"))?;

        Ok(info)
    }

    /// Uninstall a skill or MCP server by name.
    pub fn uninstall(&self, name: &str) -> Result<()> {
        let skill_dir = self.skills_dir.join(name);
        if skill_dir.exists() {
            fs::remove_dir_all(&skill_dir)?;
            return Ok(());
        }

        let mcp_file = self.mcp_dir.join(format!("{}.json", name));
        if mcp_file.exists() {
            fs::remove_file(&mcp_file)?;
            return Ok(());
        }

        bail!("no skill or MCP server named '{}' found", name);
    }

    // ── MCP servers ─────────────────────────────────────────────

    /// List all MCP server configs under `mcp/`.
    pub fn list_mcp_servers(&self) -> Result<Vec<McpInfo>> {
        let mut servers = Vec::new();
        if !self.mcp_dir.exists() {
            return Ok(servers);
        }

        for entry in fs::read_dir(&self.mcp_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let content = fs::read_to_string(&path)?;
            let config: McpConfig = serde_json::from_str(&content)
                .with_context(|| format!("invalid MCP config: {}", path.display()))?;
            servers.push(McpInfo {
                name: config.name,
                command: config.command,
                args: config.args,
                env: config.env,
                enabled: true,
            });
        }

        servers.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(servers)
    }

    /// Install an MCP server from a zip file (expects a `mcp_config.json` inside).
    ///
    /// Currently a placeholder — the zip is extracted and `mcp_config.json` is
    /// moved to the `mcp/` directory.
    pub fn install_mcp(&self, _zip_path: impl AsRef<Path>) -> Result<McpInfo> {
        // TODO: extract zip, locate mcp_config.json, validate, write to mcp/
        // For now, require the user to place a .json config directly in mcp/.
        bail!(
            "zip-based MCP install is not yet implemented. \
             Place a .json config directly in {}",
            self.mcp_dir.display()
        );
    }

    // ── private helpers ─────────────────────────────────────────

    fn parse_skill_manifest(&self, manifest_path: &Path, dir: &Path) -> Option<SkillInfo> {
        let content = fs::read_to_string(manifest_path).ok()?;
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        let version =
            extract_frontmatter_field(&content, "version").unwrap_or_else(|| "0.1.0".into());
        let description =
            extract_frontmatter_field(&content, "description").unwrap_or_else(|| "".into());

        Some(SkillInfo {
            name,
            version,
            description,
            path: dir.to_path_buf(),
            enabled: true,
        })
    }
}

// ── helpers ────────────────────────────────────────────────────────

/// Extract a YAML/JSON frontmatter field from a SKILL.md file.
///
/// Looks for `field: value` between `---` delimiters.
fn extract_frontmatter_field(content: &str, field: &str) -> Option<String> {
    let mut in_frontmatter = false;
    let mut count = 0u8;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !in_frontmatter {
                in_frontmatter = true;
                count += 1;
                if count >= 2 {
                    break;
                }
                continue;
            } else {
                break;
            }
        }
        if !in_frontmatter {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix(&format!("{}:", field)) {
            let value = rest.trim();
            // Handle quoted strings.
            let value = value
                .strip_prefix('"')
                .and_then(|v| v.strip_suffix('"'))
                .unwrap_or(value);
            let value = value
                .strip_prefix('\'')
                .and_then(|v| v.strip_suffix('\''))
                .unwrap_or(value);
            return Some(value.to_string());
        }
    }
    None
}

/// Recursively copy a directory.
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

// ── tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter() {
        let md = r#"---
version: "1.2.3"
description: A test skill
---
# Actual content"#;
        assert_eq!(
            extract_frontmatter_field(md, "version"),
            Some("1.2.3".into())
        );
        assert_eq!(
            extract_frontmatter_field(md, "description"),
            Some("A test skill".into())
        );
        assert_eq!(extract_frontmatter_field(md, "missing"), None);
    }

    #[test]
    fn test_list_skills_empty() {
        let dir = std::env::temp_dir().join("hermes_store_test");
        let _ = fs::create_dir_all(&dir);
        let store = PluginStore::new(&dir).unwrap();
        let skills = store.list_skills().unwrap();
        assert!(skills.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }
}
