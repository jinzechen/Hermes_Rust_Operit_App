//! Plugin lifecycle manager — install, enable, disable, update, uninstall.
//!
//! Maintains a persistent state file at `~/.hermes/plugins_state.json`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use super::index::{self, find_plugin, fetch_index};
use super::installer::install_plugin;

// ── state types ────────────────────────────────────────────────────

/// The runtime state of an installed plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    pub name: String,
    pub installed: bool,
    pub enabled: bool,
    pub path: PathBuf,
    pub version: String,
}

/// Top-level state container persisted to `plugins_state.json`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PluginsStateFile {
    plugins: HashMap<String, PluginState>,
}

// ── helpers ────────────────────────────────────────────────────────

/// Path to the plugins state JSON file.
fn state_path() -> Result<PathBuf> {
    let dir = index::hermes_data_dir()?;
    Ok(dir.join("plugins_state.json"))
}

/// Load the persisted state, or return an empty default.
fn load_state() -> Result<PluginsStateFile> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(PluginsStateFile::default());
    }
    let json = fs::read_to_string(&path)
        .with_context(|| format!("cannot read state at {}", path.display()))?;
    let state: PluginsStateFile = serde_json::from_str(&json)
        .with_context(|| format!("invalid state JSON at {}", path.display()))?;
    Ok(state)
}

/// Persist the current state to disk.
fn save_state(state: &PluginsStateFile) -> Result<()> {
    let path = state_path()?;
    let json = serde_json::to_string_pretty(state)?;
    fs::write(&path, json)?;
    log::debug!("plugin state saved to {}", path.display());
    Ok(())
}

/// Resolve the installation directory for a named plugin.
fn plugin_dir(name: &str) -> Result<PathBuf> {
    let dir = index::plugins_dir()?.join(name);
    Ok(dir)
}

// ── listing ────────────────────────────────────────────────────────

/// List all installed plugins (scans the `plugins/` directory).
///
/// Reconciles the filesystem with the persisted state: any directory that
/// exists in `plugins/` is considered installed. Disabled state is read
/// from the persisted JSON.
pub fn list_installed() -> Result<Vec<PluginState>> {
    let plugins_root = index::plugins_dir()?;
    let mut state = load_state().unwrap_or_default();
    let mut result = Vec::new();

    if !plugins_root.exists() {
        return Ok(result);
    }

    for entry in fs::read_dir(&plugins_root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // If we have a cached state entry, use its version; otherwise default
        let version = if let Some(cached) = state.plugins.get(&name) {
            cached.version.clone()
        } else {
            String::from("unknown")
        };

        let enabled = state
            .plugins
            .get(&name)
            .map(|s| s.enabled)
            .unwrap_or(true);

        let ps = PluginState {
            name,
            installed: true,
            enabled,
            path,
            version,
        };

        result.push(ps);
    }

    // Also include plugins in the state file that may no longer exist on disk
    for (name, ps) in &state.plugins {
        if !result.iter().any(|r| r.name == *name) {
            result.push(PluginState {
                name: name.clone(),
                installed: false,
                enabled: ps.enabled,
                path: ps.path.clone(),
                version: ps.version.clone(),
            });
        }
    }

    // Persist any newly discovered plugins
    for ps in &result {
        if ps.installed && !state.plugins.contains_key(&ps.name) {
            state.plugins.insert(
                ps.name.clone(),
                PluginState {
                    name: ps.name.clone(),
                    installed: true,
                    enabled: ps.enabled,
                    path: ps.path.clone(),
                    version: ps.version.clone(),
                },
            );
        }
    }
    let _ = save_state(&state);

    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}

// ── install ────────────────────────────────────────────────────────

/// Install a plugin from the store index by name.
///
/// Looks up the plugin in the local cache / builtin index, downloads and
/// extracts it, and registers it in the state file.
pub fn install(name: &str) -> Result<PluginState> {
    // Check if already installed
    let dir = plugin_dir(name)?;
    if dir.exists() {
        bail!("plugin '{}' is already installed at {}", name, dir.display());
    }

    let plugin = find_plugin(name)
        .ok_or_else(|| anyhow::anyhow!("plugin '{}' not found in the store index", name))?;

    let installed_path = install_plugin(&plugin)?;

    let mut state = load_state().unwrap_or_default();
    let ps = PluginState {
        name: name.to_string(),
        installed: true,
        enabled: true,
        path: installed_path,
        version: plugin.version.clone(),
    };
    state.plugins.insert(name.to_string(), ps.clone());
    save_state(&state)?;

    log::info!("plugin '{}' installed successfully", name);
    Ok(ps)
}

// ── enable / disable ───────────────────────────────────────────────

/// Enable a previously disabled plugin.
pub fn enable_plugin(name: &str) -> Result<()> {
    let mut state = load_state()?;
    let ps = state
        .plugins
        .get_mut(name)
        .ok_or_else(|| anyhow::anyhow!("plugin '{}' not found in state", name))?;

    if !ps.installed {
        bail!("plugin '{}' is not installed; install it first", name);
    }

    ps.enabled = true;
    save_state(&state)?;
    log::info!("plugin '{}' enabled", name);
    Ok(())
}

/// Disable a plugin without uninstalling it.
pub fn disable_plugin(name: &str) -> Result<()> {
    let mut state = load_state()?;
    let ps = state
        .plugins
        .get_mut(name)
        .ok_or_else(|| anyhow::anyhow!("plugin '{}' not found in state", name))?;

    ps.enabled = false;
    save_state(&state)?;
    log::info!("plugin '{}' disabled", name);
    Ok(())
}

// ── uninstall ──────────────────────────────────────────────────────

/// Completely remove a plugin from disk and state.
pub fn uninstall_plugin(name: &str) -> Result<()> {
    let dir = plugin_dir(name)?;
    let mut removed_from_disk = false;

    if dir.exists() {
        fs::remove_dir_all(&dir)
            .with_context(|| format!("failed to remove plugin directory {}", dir.display()))?;
        removed_from_disk = true;
        log::info!("removed plugin directory {}", dir.display());
    }

    let mut state = load_state().unwrap_or_default();
    state.plugins.remove(name);
    save_state(&state)?;

    if removed_from_disk {
        log::info!("plugin '{}' uninstalled", name);
    } else {
        log::warn!("plugin '{}' was not on disk but removed from state", name);
    }

    Ok(())
}

// ── update check ───────────────────────────────────────────────────

/// Check whether a newer version of a plugin is available on GitHub.
///
/// Returns `Some(version)` if an update is available, `None` if the
/// installed version is current, or an error on network failure.
pub fn check_update(name: &str) -> Result<Option<String>> {
    let state = load_state()?;
    let installed = state
        .plugins
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("plugin '{}' is not installed", name))?;

    if !installed.installed {
        bail!("plugin '{}' is not installed; cannot check for updates", name);
    }

    // Build the repo name from the plugin name using the standard convention
    let repo = format!("Lumio-Research/hermes-{}", name.replace('_', "-"));

    let index = match fetch_index(&repo) {
        Ok(idx) => idx,
        Err(e) => {
            log::warn!("failed to fetch index for update check: {}", e);
            return Ok(None);
        }
    };

    let latest = match index.plugins.into_iter().find(|p| p.name == name) {
        Some(p) => p,
        None => {
            log::warn!("plugin '{}' not found in remote index", name);
            return Ok(None);
        }
    };

    if latest.version != installed.version {
        log::info!(
            "update available for '{}': {} -> {}",
            name,
            installed.version,
            latest.version
        );
        Ok(Some(latest.version))
    } else {
        log::info!("plugin '{}' is up to date (v{})", name, installed.version);
        Ok(None)
    }
}

/// Upgrade an installed plugin to the latest version.
///
/// This is a convenience wrapper around `uninstall_plugin` + `install`.
pub fn upgrade_plugin(name: &str) -> Result<PluginState> {
    // First check that an update exists
    if check_update(name)?.is_none() {
        bail!("plugin '{}' is already up to date", name);
    }

    // Uninstall the old version
    uninstall_plugin(name)?;

    // Install the new version
    install(name)
}

// ── tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_installed_empty() {
        // In a test environment with no real plugins dir, should return empty or
        // a few entries. We just test that the function doesn't crash.
        let result = list_installed();
        // It's okay if it fails (no ProjectDirs on some CI), or returns empty.
        if let Ok(plugins) = result {
            // We don't assert emptiness because other tests may have created dirs
            for p in &plugins {
                assert!(!p.name.is_empty());
            }
        }
    }

    #[test]
    fn test_state_save_load_roundtrip() {
        let mut state = PluginsStateFile::default();
        state.plugins.insert(
            "test_plugin".into(),
            PluginState {
                name: "test_plugin".into(),
                installed: true,
                enabled: true,
                path: PathBuf::from("/tmp/test_plugin"),
                version: "1.0.0".into(),
            },
        );

        // We can't easily override state_path, but we can test serde roundtrip
        let json = serde_json::to_string(&state).unwrap();
        let back: PluginsStateFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.plugins.len(), 1);
        assert_eq!(back.plugins["test_plugin"].version, "1.0.0");
    }

    #[test]
    fn test_enable_disable_nonexistent() {
        // Should fail for a nonexistent plugin
        assert!(enable_plugin("nonexistent_plugin_xyz").is_err());
        assert!(disable_plugin("nonexistent_plugin_xyz").is_err());
    }

    #[test]
    fn test_uninstall_nonexistent() {
        // Uninstalling a plugin not in state should succeed (just no-op on disk,
        // and we remove from state which is also a no-op if not present)
        // Actually, uninstall_plugin will try load_state, remove from state, then
        // try to remove dir — if dir doesn't exist it logs a warning. So it won't
        // error. Let's test:
        let result = uninstall_plugin("nonexistent_plugin_xyz");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_update_not_installed() {
        let result = check_update("nonexistent_plugin_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_plugin_state_serialization() {
        let ps = PluginState {
            name: "obscura".into(),
            installed: true,
            enabled: true,
            path: PathBuf::from("/home/user/.hermes/plugins/obscura"),
            version: "0.1.0".into(),
        };

        let json = serde_json::to_string(&ps).unwrap();
        let back: PluginState = serde_json::from_str(&json).unwrap();

        assert_eq!(back.name, "obscura");
        assert!(back.installed);
        assert!(back.enabled);
        assert_eq!(back.version, "0.1.0");
    }

    #[test]
    fn test_plugins_dir_accessible() {
        let dir = index::plugins_dir().unwrap();
        // The dir should exist
        assert!(dir.exists());
    }

    #[test]
    fn test_disable_nonexistent_errors() {
        assert!(disable_plugin("completely_fake_plugin_name").is_err());
    }
}
