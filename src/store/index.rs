//! Plugin index — GitHub Releases API integration + local cache.
//!
//! Provides:
//! - `PluginIndex` / `StoreIndex` types
//! - `fetch_index()` to pull from GitHub Releases
//! - `builtin_index()` as a fallback with 5 standard plugins
//! - Local cache at `~/.hermes/store_index.json`

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

// ── types ──────────────────────────────────────────────────────────

/// A single plugin entry in the store index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PluginIndex {
    pub name: String,
    pub version: String,
    pub description: String,
    pub sha256: String,
    pub size: u64,
    pub download_url: String,
    pub stars: u32,
    pub author: String,
    pub category: String,
}

/// The full store index — a list of plugins plus a last-updated timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreIndex {
    pub plugins: Vec<PluginIndex>,
    pub updated_at: String,
}

// ── helpers ────────────────────────────────────────────────────────

/// Return the path where the store index cache is stored.
fn cache_path() -> Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "Nous Research", "Hermes")
        .context("could not determine project directories")?;
    let data_dir = proj_dirs.data_local_dir();
    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("store_index.json"))
}

/// Return the Hermes data directory (same as above but for reuse).
pub fn hermes_data_dir() -> Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "Nous Research", "Hermes")
        .context("could not determine project directories")?;
    let data_dir = proj_dirs.data_local_dir().to_path_buf();
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}

/// Return the plugins installation root: `~/.hermes/plugins/`
pub fn plugins_dir() -> Result<PathBuf> {
    let dir = hermes_data_dir()?.join("plugins");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

// ── builtin index ──────────────────────────────────────────────────

/// Return a built-in fallback index with the 5 standard plugins.
/// Used when GitHub is unreachable or no cache exists.
pub fn builtin_index() -> StoreIndex {
    let plugins = vec![
        PluginIndex {
            name: "obscura".into(),
            version: "0.1.0".into(),
            description: "Headless browser automation — navigate, click, extract, and screenshot web pages".into(),
            sha256: "".into(),
            size: 0,
            download_url: "https://github.com/Lumio-Research/hermes-obscura/releases/latest/download/obscura.zip".into(),
            stars: 42,
            author: "Lumio-Research".into(),
            category: "browser".into(),
        },
        PluginIndex {
            name: "agentic_vision".into(),
            version: "0.1.0".into(),
            description: "Image analysis and visual understanding via AI vision models".into(),
            sha256: "".into(),
            size: 0,
            download_url: "https://github.com/Lumio-Research/hermes-agentic-vision/releases/latest/download/agentic_vision.zip".into(),
            stars: 28,
            author: "Lumio-Research".into(),
            category: "vision".into(),
        },
        PluginIndex {
            name: "filesystem".into(),
            version: "0.1.0".into(),
            description: "Safe filesystem operations — read, write, search, and manage files".into(),
            sha256: "".into(),
            size: 0,
            download_url: "https://github.com/Lumio-Research/hermes-filesystem/releases/latest/download/filesystem.zip".into(),
            stars: 35,
            author: "Lumio-Research".into(),
            category: "tools".into(),
        },
        PluginIndex {
            name: "typemill".into(),
            version: "0.1.0".into(),
            description: "Markdown processing and type-setting engine for document generation".into(),
            sha256: "".into(),
            size: 0,
            download_url: "https://github.com/Lumio-Research/hermes-typemill/releases/latest/download/typemill.zip".into(),
            stars: 19,
            author: "Lumio-Research".into(),
            category: "text".into(),
        },
        PluginIndex {
            name: "sherpa".into(),
            version: "0.1.0".into(),
            description: "Guided task workflows and orchestration for Hermes Agent".into(),
            sha256: "".into(),
            size: 0,
            download_url: "https://github.com/Lumio-Research/hermes-sherpa/releases/latest/download/sherpa.zip".into(),
            stars: 31,
            author: "Lumio-Research".into(),
            category: "workflow".into(),
        },
    ];

    StoreIndex {
        plugins,
        updated_at: Utc::now().to_rfc3339(),
    }
}

// ── GitHub Releases API ────────────────────────────────────────────

/// JSON shape returned by GET /repos/{owner}/{repo}/releases
#[derive(Debug, Deserialize)]
struct GitHubReleaseAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    assets: Vec<GitHubReleaseAsset>,
    stargazers_count: Option<u32>,
}

/// Fetch the plugin index from a GitHub repository's Releases.
///
/// `repo` can be in the form `owner/repo` (e.g. `Lumio-Research/hermes-obscura`)
/// or a full GitHub URL.  Returns a `StoreIndex`.
///
/// On network error, falls back to loading the local cache, and if that fails,
/// returns the builtin index.
pub fn fetch_index(repo: &str) -> Result<StoreIndex> {
    // Normalize input
    let (owner, repo) = parse_github_repo(repo)?;
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases?per_page=10",
        owner, repo
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("Hermes-PluginStore/0.1")
        .build()
        .context("failed to build HTTP client")?;

    let response = match client.get(&url).send() {
        Ok(r) => r,
        Err(e) => {
            log::warn!("GitHub API unreachable ({}), falling back to cache/builtin", e);
            return load_cached_or_builtin();
        }
    };

    if !response.status().is_success() {
        log::warn!(
            "GitHub API returned {} for {}/{}, falling back to cache/builtin",
            response.status(),
            owner,
            repo
        );
        return load_cached_or_builtin();
    }

    let releases: Vec<GitHubRelease> = response
        .json()
        .context("failed to parse GitHub releases JSON")?;

    let mut plugins = Vec::new();

    for release in &releases {
        for asset in &release.assets {
            // Only consider .zip assets
            if !asset.name.ends_with(".zip") {
                continue;
            }

            let plugin_name = asset
                .name
                .strip_suffix(".zip")
                .unwrap_or(&asset.name)
                .to_string();

            let description = release
                .body
                .clone()
                .unwrap_or_default()
                .lines()
                .take(3)
                .collect::<Vec<_>>()
                .join(" ");

            plugins.push(PluginIndex {
                name: plugin_name,
                version: release.tag_name.clone(),
                description,
                sha256: String::new(), // GitHub doesn't provide this in the API
                size: asset.size,
                download_url: asset.browser_download_url.clone(),
                stars: release.stargazers_count.unwrap_or(0),
                author: owner.to_string(),
                category: String::from("uncategorized"),
            });
        }
    }

    let index = StoreIndex {
        plugins,
        updated_at: Utc::now().to_rfc3339(),
    };

    // Persist to cache
    let _ = save_cache(&index);

    Ok(index)
}

/// Parse `owner/repo` out of various inputs.
fn parse_github_repo(input: &str) -> Result<(&str, &str)> {
    // Strip https://github.com/ prefix if present
    let s = input
        .strip_prefix("https://github.com/")
        .unwrap_or(input);
    let s = s.strip_prefix("http://github.com/").unwrap_or(s);
    // Strip trailing slash
    let s = s.strip_suffix('/').unwrap_or(s);

    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() >= 2 {
        Ok((parts[0], parts[1]))
    } else {
        anyhow::bail!(
            "invalid GitHub repo '{}' — expected owner/repo or a full URL",
            input
        );
    }
}

// ── cache operations ───────────────────────────────────────────────

/// Save a StoreIndex to the local cache file.
pub fn save_cache(index: &StoreIndex) -> Result<()> {
    let path = cache_path()?;
    let json = serde_json::to_string_pretty(index)?;
    fs::write(&path, json)?;
    log::info!("store index cached to {}", path.display());
    Ok(())
}

/// Load the StoreIndex from the local cache file.
pub fn load_cache() -> Result<StoreIndex> {
    let path = cache_path()?;
    let json = fs::read_to_string(&path)
        .with_context(|| format!("cache not found at {}", path.display()))?;
    let index: StoreIndex = serde_json::from_str(&json)
        .with_context(|| format!("invalid cache JSON at {}", path.display()))?;
    Ok(index)
}

/// Try loading from cache; if that fails, return the builtin index.
pub fn load_cached_or_builtin() -> Result<StoreIndex> {
    match load_cache() {
        Ok(index) => Ok(index),
        Err(e) => {
            log::warn!("cache load failed ({}), using builtin index", e);
            Ok(builtin_index())
        }
    }
}

/// Check whether the local cache file exists.
pub fn has_cache() -> bool {
    cache_path().map(|p| p.exists()).unwrap_or(false)
}

// ── single plugin lookup ───────────────────────────────────────────

/// Look up a single plugin by name from the index (cache first, then builtin).
pub fn find_plugin(name: &str) -> Option<PluginIndex> {
    let index = load_cached_or_builtin().ok()?;
    index.plugins.into_iter().find(|p| p.name == name)
}

/// Fetch latest release info for a single plugin from GitHub, or fall back.
pub fn fetch_plugin(name: &str, repo: &str) -> Result<PluginIndex> {
    let index = fetch_index(repo)?;
    index
        .plugins
        .into_iter()
        .find(|p| p.name == name)
        .ok_or_else(|| anyhow::anyhow!("plugin '{}' not found in repo '{}'", name, repo))
}

// ── tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_repo_full_url() {
        let (owner, repo) =
            parse_github_repo("https://github.com/Lumio-Research/hermes-obscura").unwrap();
        assert_eq!(owner, "Lumio-Research");
        assert_eq!(repo, "hermes-obscura");
    }

    #[test]
    fn test_parse_github_repo_short() {
        let (owner, repo) = parse_github_repo("Lumio-Research/hermes-obscura").unwrap();
        assert_eq!(owner, "Lumio-Research");
        assert_eq!(repo, "hermes-obscura");
    }

    #[test]
    fn test_parse_github_repo_trailing_slash() {
        let (owner, repo) =
            parse_github_repo("https://github.com/Lumio-Research/hermes-obscura/").unwrap();
        assert_eq!(owner, "Lumio-Research");
        assert_eq!(repo, "hermes-obscura");
    }

    #[test]
    fn test_parse_github_repo_invalid() {
        assert!(parse_github_repo("just-a-string").is_err());
    }

    #[test]
    fn test_builtin_index_has_five_plugins() {
        let index = builtin_index();
        assert_eq!(index.plugins.len(), 5);
        assert!(!index.updated_at.is_empty());
    }

    #[test]
    fn test_builtin_index_has_expected_names() {
        let index = builtin_index();
        let names: Vec<&str> = index.plugins.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"obscura"));
        assert!(names.contains(&"agentic_vision"));
        assert!(names.contains(&"filesystem"));
        assert!(names.contains(&"typemill"));
        assert!(names.contains(&"sherpa"));
    }

    #[test]
    fn test_builtin_index_plugins_have_download_urls() {
        let index = builtin_index();
        for plugin in &index.plugins {
            assert!(
                !plugin.download_url.is_empty(),
                "plugin {} has empty download_url",
                plugin.name
            );
        }
    }

    #[test]
    fn test_find_plugin_exists() {
        let p = find_plugin("obscura");
        assert!(p.is_some());
        assert_eq!(p.unwrap().name, "obscura");
    }

    #[test]
    fn test_find_plugin_missing() {
        let p = find_plugin("nonexistent_plugin_xyz");
        assert!(p.is_none());
    }

    #[test]
    fn test_cache_roundtrip() {
        let index = builtin_index();
        let _tmp = std::env::temp_dir().join("hermes_store_test_cache.json");

        // We can't easily override cache_path, so just test serde roundtrip
        let json = serde_json::to_string(&index).unwrap();
        let back: StoreIndex = serde_json::from_str(&json).unwrap();
        assert_eq!(back.plugins.len(), index.plugins.len());
        assert_eq!(back.updated_at, index.updated_at);
    }

    #[test]
    fn test_plugins_dir_creates_directory() {
        let dir = plugins_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.is_dir());
    }

    #[test]
    fn test_hermes_data_dir_creates_directory() {
        let dir = hermes_data_dir().unwrap();
        assert!(dir.exists());
        assert!(dir.is_dir());
    }
}
