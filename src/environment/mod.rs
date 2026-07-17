//! Alpine environment manager.
//!
//! Downloads and manages an Alpine Linux rootfs, then executes
//! commands inside it via `proot`.  On non-Linux platforms all
//! operations return placeholder / error results.

pub mod sandbox;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

/// Configuration for the Alpine environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Root directory where the Alpine rootfs lives.
    pub root_path: PathBuf,
    /// URL of the Alpine minirootfs tarball to download.
    pub rootfs_url: String,
    /// Alpine release (e.g. "3.19").
    pub release: String,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("./alpine_rootfs"),
            rootfs_url: "https://dl-cdn.alpinelinux.org/alpine/v3.19/releases/x86_64/alpine-minirootfs-3.19.1-x86_64.tar.gz".into(),
            release: "3.19".into(),
        }
    }
}

/// Manages an Alpine Linux environment.
pub struct Environment {
    config: EnvironmentConfig,
}

impl Environment {
    /// Create a new environment manager rooted at `root_path`.
    pub fn new(root_path: impl Into<PathBuf>) -> Self {
        let config = EnvironmentConfig {
            root_path: root_path.into(),
            ..Default::default()
        };
        Self { config }
    }

    /// Create with a custom config.
    pub fn with_config(config: EnvironmentConfig) -> Self {
        Self { config }
    }

    /// Return the rootfs path.
    pub fn root_path(&self) -> &Path {
        &self.config.root_path
    }

    /// Check whether the Alpine rootfs is ready to use.
    pub fn is_ready(&self) -> bool {
        let etc = self.config.root_path.join("etc");
        let bin = self.config.root_path.join("bin");
        etc.exists() && bin.exists()
    }

    /// Download and extract the Alpine minirootfs.
    ///
    /// On non-Linux platforms this returns an error.
    pub fn initialize(&self) -> Result<()> {
        if !cfg!(target_os = "linux") {
            bail!("Alpine environment initialization is only supported on Linux hosts");
        }

        if self.is_ready() {
            return Ok(()); // Already initialized.
        }

        fs::create_dir_all(&self.config.root_path)?;

        let tar_gz = self.config.root_path.join("alpine-rootfs.tar.gz");

        // Download.
        let status = StdCommand::new("curl")
            .args([
                "-L",
                "-o",
                tar_gz.to_str().unwrap_or("alpine-rootfs.tar.gz"),
                &self.config.rootfs_url,
            ])
            .status()
            .context("failed to download Alpine rootfs (curl not found or network error)")?;
        if !status.success() {
            bail!("curl exited with non-zero status");
        }

        // Extract.
        let status = StdCommand::new("tar")
            .args([
                "-xzf",
                tar_gz.to_str().unwrap_or("alpine-rootfs.tar.gz"),
                "-C",
                self.config.root_path.to_str().unwrap_or("."),
            ])
            .status()
            .context("failed to extract Alpine rootfs")?;
        if !status.success() {
            bail!("tar exited with non-zero status");
        }

        // Cleanup.
        let _ = fs::remove_file(&tar_gz);

        Ok(())
    }

    /// Install packages inside the Alpine rootfs using `apk`.
    ///
    /// Only works after `initialize()` has succeeded.
    pub fn install_packages(&self, packages: Vec<String>) -> Result<()> {
        if !cfg!(target_os = "linux") {
            bail!("Alpine package installation is only supported on Linux hosts");
        }
        if !self.is_ready() {
            bail!("Alpine rootfs not initialized — call initialize() first");
        }

        // Use proot to run apk inside the rootfs.
        let mut cmd = StdCommand::new("proot");
        cmd.arg("-r")
            .arg(&self.config.root_path)
            .arg("-b")
            .arg("/dev")
            .arg("-b")
            .arg("/proc")
            .arg("-b")
            .arg("/sys")
            .arg("/sbin/apk")
            .arg("add")
            .arg("--no-cache");
        for pkg in &packages {
            cmd.arg(pkg);
        }

        let status = cmd.status().context("failed to run apk via proot")?;
        if !status.success() {
            bail!("apk add exited with non-zero status");
        }

        Ok(())
    }

    /// Execute a command inside the Alpine rootfs via proot.
    ///
    /// Returns (stdout, stderr, exit_code).
    pub fn execute(&self, command: &str) -> Result<(String, String, i32)> {
        if !cfg!(target_os = "linux") {
            bail!("Alpine execution is only supported on Linux hosts");
        }
        if !self.is_ready() {
            bail!("Alpine rootfs not initialized — call initialize() first");
        }

        // Split command into program + args (simple whitespace split).
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            bail!("empty command");
        }

        let output = StdCommand::new("proot")
            .arg("-r")
            .arg(&self.config.root_path)
            .arg("-b")
            .arg("/dev")
            .arg("-b")
            .arg("/proc")
            .arg("-b")
            .arg("/sys")
            .arg("-w")
            .arg("/root")
            .arg(&parts[0])
            .args(&parts[1..])
            .output()
            .context("failed to execute command via proot")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ready_false_by_default() {
        let env = Environment::new(std::env::temp_dir().join("hermes_env_test"));
        assert!(!env.is_ready());
    }
}
