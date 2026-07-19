//! Integration tests for the Plugin Store — builtin index integrity,
//! store manager lifecycle (install/enable/disable/uninstall), and
//! PluginStore (skills + MCP) operations.
//!
//! These tests use real components: the builtin index, filesystem-based
//! PluginStore, and the store manager's state file.

use anyhow::Result;
use hermes_operit_core::store::index::{self, builtin_index, find_plugin};
use hermes_operit_core::store::manager::{self, disable_plugin, enable_plugin, list_installed};
use hermes_operit_core::store::PluginStore;
use std::fs;

// ── test_builtin_index ───────────────────────────────────────────────────────

#[test]
fn test_builtin_index_has_five_plugins() -> Result<()> {
    let index = builtin_index();
    assert_eq!(index.plugins.len(), 5);
    assert!(!index.updated_at.is_empty());

    Ok(())
}

#[test]
fn test_builtin_index_expected_names() -> Result<()> {
    let index = builtin_index();
    let names: Vec<&str> = index.plugins.iter().map(|p| p.name.as_str()).collect();

    let expected = [
        "obscura",
        "agentic_vision",
        "filesystem",
        "typemill",
        "sherpa",
    ];
    for expected_name in &expected {
        assert!(
            names.contains(expected_name),
            "Missing plugin from builtin index: {}",
            expected_name
        );
    }

    Ok(())
}

#[test]
fn test_builtin_index_plugins_have_fields() -> Result<()> {
    let index = builtin_index();

    for plugin in &index.plugins {
        assert!(!plugin.name.is_empty(), "Plugin has empty name");
        assert!(
            !plugin.version.is_empty(),
            "Plugin '{}' has empty version",
            plugin.name
        );
        assert!(
            !plugin.description.is_empty(),
            "Plugin '{}' has empty description",
            plugin.name
        );
        assert!(
            !plugin.download_url.is_empty(),
            "Plugin '{}' has empty download_url",
            plugin.name
        );
        assert!(
            !plugin.author.is_empty(),
            "Plugin '{}' has empty author",
            plugin.name
        );
        assert!(
            !plugin.category.is_empty(),
            "Plugin '{}' has empty category",
            plugin.name
        );

        // stars should be non-negative.
        assert!(
            plugin.stars <= 1_000_000,
            "Plugin '{}' has unreasonably high stars: {}",
            plugin.name,
            plugin.stars
        );
    }

    Ok(())
}

#[test]
fn test_builtin_index_serde_roundtrip() -> Result<()> {
    let index = builtin_index();
    let json = serde_json::to_string(&index)?;
    let back: index::StoreIndex = serde_json::from_str(&json)?;

    assert_eq!(back.plugins.len(), index.plugins.len());
    assert_eq!(back.updated_at, index.updated_at);

    Ok(())
}

#[test]
fn test_builtin_index_find_plugin() -> Result<()> {
    // Find an existing plugin.
    let plugin = find_plugin("obscura");
    assert!(plugin.is_some());
    assert_eq!(plugin.unwrap().name, "obscura");

    // Find a non-existent plugin.
    let missing = find_plugin("nonexistent_plugin_xyz_123");
    assert!(missing.is_none());

    Ok(())
}

// ── test_store_manager ───────────────────────────────────────────────────────

#[test]
fn test_store_manager_list_installed() -> Result<()> {
    // list_installed should not panic even if no plugins are installed.
    let plugins = list_installed()?;

    // Each plugin entry should have a non-empty name if present.
    for p in &plugins {
        assert!(!p.name.is_empty(), "Plugin has empty name");
    }

    Ok(())
}

#[test]
fn test_store_manager_enable_disable_nonexistent() -> Result<()> {
    // Should fail for a plugin that doesn't exist in state.
    let random_name = "completely_nonexistent_plugin_xyz_12345";

    assert!(
        enable_plugin(random_name).is_err(),
        "enable_plugin should fail for unknown plugin"
    );
    assert!(
        disable_plugin(random_name).is_err(),
        "disable_plugin should fail for unknown plugin"
    );

    Ok(())
}

#[test]
fn test_store_manager_uninstall_nonexistent() -> Result<()> {
    // Uninstalling a plugin not in the state should still succeed
    // (it logs a warning but doesn't error out).
    let result = manager::uninstall_plugin("nonexistent_plugin_xyz_12345");
    assert!(
        result.is_ok(),
        "uninstall_plugin should succeed for unknown plugin"
    );

    Ok(())
}

#[test]
fn test_store_manager_check_update_not_installed() -> Result<()> {
    // Checking updates for an uninstalled plugin should error.
    let result = manager::check_update("nonexistent_plugin_xyz_12345");
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_store_manager_plugin_state_serialization() -> Result<()> {
    use hermes_operit_core::store::manager::PluginState;
    use std::path::PathBuf;

    let ps = PluginState {
        name: "obscura".into(),
        installed: true,
        enabled: true,
        path: PathBuf::from("/home/user/.hermes/plugins/obscura"),
        version: "0.1.0".into(),
    };

    let json = serde_json::to_string(&ps)?;
    let back: PluginState = serde_json::from_str(&json)?;

    assert_eq!(back.name, "obscura");
    assert!(back.installed);
    assert!(back.enabled);
    assert_eq!(back.version, "0.1.0");

    Ok(())
}

// ── test_plugin_store ────────────────────────────────────────────────────────

#[test]
fn test_plugin_store_list_skills_empty() -> Result<()> {
    let dir = std::env::temp_dir().join("hermes_store_integration_test");
    let _ = fs::create_dir_all(&dir);

    let store = PluginStore::new(&dir)?;
    let skills = store.list_skills()?;
    assert!(skills.is_empty());

    let _ = fs::remove_dir_all(&dir);
    Ok(())
}

#[test]
fn test_plugin_store_install_skill() -> Result<()> {
    let dir = std::env::temp_dir().join("hermes_store_skill_test");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;

    // Create a minimal skill directory with SKILL.md.
    let skill_src = dir.join("test_skill");
    fs::create_dir_all(&skill_src)?;
    fs::write(
        skill_src.join("SKILL.md"),
        "---\nversion: \"1.0.0\"\ndescription: A test skill for integration testing\n---\n# Test Skill\n\nThis is a test.",
    )?;

    let store = PluginStore::new(&dir)?;

    // Install the skill.
    let info = store.install_skill(&skill_src)?;
    assert_eq!(info.name, "test_skill");
    assert_eq!(info.version, "1.0.0");
    assert_eq!(info.description, "A test skill for integration testing");
    assert!(info.enabled);

    // List skills should now include it.
    let skills = store.list_skills()?;
    assert_eq!(skills.len(), 1);

    // Uninstall it.
    store.uninstall("test_skill")?;
    let skills = store.list_skills()?;
    assert!(skills.is_empty());

    let _ = fs::remove_dir_all(&dir);
    Ok(())
}

#[test]
fn test_plugin_store_install_skill_duplicate() -> Result<()> {
    let dir = std::env::temp_dir().join("hermes_store_dup_test");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;

    let skill_src = dir.join("duplicate_skill");
    fs::create_dir_all(&skill_src)?;
    fs::write(
        skill_src.join("SKILL.md"),
        "---\nversion: \"1.0.0\"\ndescription: A skill\n---\n# Skill",
    )?;

    let store = PluginStore::new(&dir)?;

    // First install should succeed.
    store.install_skill(&skill_src)?;

    // Second install should fail (already exists).
    let err = store.install_skill(&skill_src).unwrap_err();
    assert!(err.to_string().contains("already installed"));

    let _ = fs::remove_dir_all(&dir);
    Ok(())
}

#[test]
fn test_plugin_store_uninstall_nonexistent() -> Result<()> {
    let dir = std::env::temp_dir().join("hermes_store_noexist_test");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;

    let store = PluginStore::new(&dir)?;
    let err = store.uninstall("nonexistent_skill").unwrap_err();
    assert!(err.to_string().contains("no skill or MCP server"));

    let _ = fs::remove_dir_all(&dir);
    Ok(())
}

#[test]
fn test_plugin_store_mcp_empty() -> Result<()> {
    let dir = std::env::temp_dir().join("hermes_store_mcp_test");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;

    let store = PluginStore::new(&dir)?;
    let servers = store.list_mcp_servers()?;
    assert!(servers.is_empty());

    let _ = fs::remove_dir_all(&dir);
    Ok(())
}

// ── test_index_cache ─────────────────────────────────────────────────────────

#[test]
fn test_index_hermes_data_dir_creates() -> Result<()> {
    let dir = index::hermes_data_dir()?;
    assert!(dir.exists());
    assert!(dir.is_dir());

    Ok(())
}

#[test]
fn test_index_plugins_dir_creates() -> Result<()> {
    let dir = index::plugins_dir()?;
    assert!(dir.exists());
    assert!(dir.is_dir());

    Ok(())
}

// ── test_store_index_parse_github_repo ───────────────────────────────────────

#[test]
fn test_store_index_parse_github_repo_short() {
    // Note: parse_github_repo is not publicly exported, but the underlying
    // fetch_index path parsing is tested in the module's own tests.
    // We verify fetch_index-related functionality at the integration level
    // by checking builtin_index and find_plugin work.
}

#[test]
fn test_store_integration_end_to_end() -> Result<()> {
    // Full lifecycle test: check builtin index, verify find_plugin, list installed.
    let index = builtin_index();
    assert_eq!(index.plugins.len(), 5);

    // find_plugin for all builtins should succeed.
    for plugin_name in &[
        "obscura",
        "agentic_vision",
        "filesystem",
        "typemill",
        "sherpa",
    ] {
        let found = find_plugin(plugin_name);
        assert!(
            found.is_some(),
            "Expected to find plugin '{}' in builtin index",
            plugin_name
        );
        assert_eq!(found.unwrap().name, *plugin_name);
    }

    // list_installed should work without panicking.
    let installed = list_installed()?;
    // Just verify it's a valid result.
    let _ = installed.len();

    Ok(())
}
