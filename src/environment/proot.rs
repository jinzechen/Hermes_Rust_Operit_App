//! PRoot-based Ubuntu environment manager for Android (Termux).
//!
//! On Android, this module manages an Ubuntu rootfs via `proot`,
//! allowing execution of Ubuntu commands, package management via
//! `apt-get`/`dpkg`, and system introspection.
//!
//! On non-Android platforms, all `execute`-family methods return
//! an error with the message "proot only available on Android",
//! and `detect()` always returns `None`.

use std::path::PathBuf;
use std::process::Command as StdCommand;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ProotConfig
// ---------------------------------------------------------------------------

/// Configuration for the PRoot Ubuntu environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProotConfig {
    /// The `proot` binary name or path.  Defaults to `"proot"`.
    pub proot_command: String,
    /// Path to the Ubuntu rootfs directory.
    pub rootfs_path: PathBuf,
    /// Extra arguments passed to every `proot` invocation (e.g.
    /// `-0` for root, `-b /sdcard` for bind-mounts).
    pub extra_args: Vec<String>,
    /// Default working directory inside the proot container.
    pub default_workdir: PathBuf,
}

impl Default for ProotConfig {
    fn default() -> Self {
        Self {
            proot_command: "proot".to_string(),
            rootfs_path: PathBuf::from(
                "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu",
            ),
            extra_args: Vec::new(),
            default_workdir: PathBuf::from("/root"),
        }
    }
}

impl ProotConfig {
    /// Create a new config pointing at a specific rootfs.
    pub fn new(rootfs: impl Into<PathBuf>) -> Self {
        Self {
            rootfs_path: rootfs.into(),
            ..Default::default()
        }
    }
}

// ---------------------------------------------------------------------------
// ProotEnv
// ---------------------------------------------------------------------------

/// Manages a Ubuntu environment that runs inside `proot` on Android.
///
/// On non-Android hosts every method (except `is_available`) returns an
/// error so callers can degrade gracefully without `#[cfg]` littering
/// the call-sites.
#[derive(Debug, Clone)]
pub struct ProotEnv {
    /// Path to the Ubuntu rootfs.
    pub rootfs_path: PathBuf,
    /// Path to the `proot` binary.
    pub proot_binary: PathBuf,
    /// Whether the rootfs is currently mounted / active.
    pub mounted: bool,
}

impl ProotEnv {
    // ------------------------------------------------------------------
    // Static constructors
    // ------------------------------------------------------------------

    /// Auto-detect a PRoot Ubuntu installation from standard Termux paths.
    ///
    /// On non-Android, always returns `None`.
    /// On Android, probes these locations (in order):
    ///
    /// 1. `$PREFIX/var/lib/proot-distro/installed-rootfs/ubuntu`
    /// 2. `/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu`
    ///
    /// Returns `Some(ProotEnv)` if both `proot` and the rootfs exist.
    pub fn detect() -> Option<Self> {
        if !cfg!(target_os = "android") {
            return None;
        }

        let candidate_rootfs_paths: Vec<PathBuf> = vec![
            // Typical Termux $PREFIX layout
            PathBuf::from(
                "/data/data/com.termux/files/usr/var/lib/proot-distro/installed-rootfs/ubuntu",
            ),
        ];

        // Also check $PREFIX from environment
        if let Ok(prefix) = std::env::var("PREFIX") {
            let env_rootfs =
                PathBuf::from(&prefix).join("var/lib/proot-distro/installed-rootfs/ubuntu");
            if !candidate_rootfs_paths.contains(&env_rootfs) {
                // push front so env var takes priority
                // candidate_rootfs_paths.insert(0, env_rootfs);
                // The vec is small, just rebuild:
                let mut paths = vec![env_rootfs];
                paths.extend(candidate_rootfs_paths);
                return Self::try_paths(&paths);
            }
        }

        Self::try_paths(&candidate_rootfs_paths)
    }

    fn try_paths(rootfs_paths: &[PathBuf]) -> Option<Self> {
        // Probe common proot binary locations.
        let proot_candidates = &["proot", "/data/data/com.termux/files/usr/bin/proot"];

        for rootfs in rootfs_paths {
            if !rootfs.join("etc").exists() || !rootfs.join("bin").exists() {
                continue;
            }
            for proot_candidate in proot_candidates {
                let proot_bin = PathBuf::from(proot_candidate);
                // For bare "proot" we rely on PATH; assume it's available
                // if we're on Android.  For absolute paths, check existence.
                if proot_bin.is_absolute() && !proot_bin.exists() {
                    continue;
                }
                return Some(Self {
                    rootfs_path: rootfs.clone(),
                    proot_binary: proot_bin,
                    mounted: false,
                });
            }
        }

        None
    }

    /// Create a new `ProotEnv` pointing to a specific rootfs.
    ///
    /// The `proot` binary defaults to `"proot"` (found on `PATH`).
    /// Use [`ProotEnv::with_proot`] for a custom binary path.
    pub fn new(rootfs: impl Into<PathBuf>) -> Self {
        Self {
            rootfs_path: rootfs.into(),
            proot_binary: PathBuf::from("proot"),
            mounted: false,
        }
    }

    /// Create a new `ProotEnv` with an explicit proot binary path.
    pub fn with_proot(rootfs: impl Into<PathBuf>, proot: impl Into<PathBuf>) -> Self {
        Self {
            rootfs_path: rootfs.into(),
            proot_binary: proot.into(),
            mounted: false,
        }
    }

    // ------------------------------------------------------------------
    // Availability
    // ------------------------------------------------------------------

    /// Returns `true` if the `proot` binary **and** the rootfs exist.
    ///
    /// On non-Android this always returns `false`.
    pub fn is_available(&self) -> bool {
        if !cfg!(target_os = "android") {
            return false;
        }
        // If the proot binary path is absolute, check it exists; otherwise
        // assume it is on PATH (we can't meaningfully verify that at
        // compile time on Android without shelling out).
        if self.proot_binary.is_absolute() && !self.proot_binary.exists() {
            return false;
        }
        // Minimal rootfs sanity check.
        self.rootfs_path.join("etc").exists() && self.rootfs_path.join("bin").exists()
    }

    // ------------------------------------------------------------------
    // Execution helpers
    // ------------------------------------------------------------------

    /// Execute a command inside the proot environment.
    ///
    /// The command string is split on whitespace; the first token is the
    /// program and the remaining tokens are arguments.
    ///
    /// Returns the combined stdout of the command.
    ///
    /// # Errors
    ///
    /// Returns an error on non-Android platforms, or if the command fails.
    pub fn execute(&self, command: &str) -> Result<String> {
        self.execute_with_workdir(command, "/root")
    }

    /// Execute a command inside the proot environment with a specific
    /// working directory.
    ///
    /// # Errors
    ///
    /// Returns an error on non-Android platforms, or if the command fails.
    pub fn execute_with_workdir(&self, command: &str, workdir: impl AsRef<str>) -> Result<String> {
        if !cfg!(target_os = "android") {
            bail!("proot only available on Android");
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            bail!("empty command");
        }

        let mut cmd = StdCommand::new(&self.proot_binary);
        cmd.arg("-r").arg(&self.rootfs_path);
        cmd.arg("-w").arg(workdir.as_ref());

        // Bind-mount essential pseudo-filesystems.
        cmd.arg("-b").arg("/dev");
        cmd.arg("-b").arg("/proc");
        cmd.arg("-b").arg("/sys");

        // Program + args.
        cmd.arg(parts[0]);
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }

        let output = cmd
            .output()
            .context("failed to execute command via proot")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "proot command '{}' exited with status {}: {}",
                command,
                output.status,
                stderr.trim()
            );
        }

        Ok(stdout)
    }

    // ------------------------------------------------------------------
    // Package management
    // ------------------------------------------------------------------

    /// Install a package via `apt-get install -y`.
    ///
    /// # Errors
    ///
    /// Returns an error on non-Android platforms, or if the install fails.
    pub fn install_package(&self, name: &str) -> Result<String> {
        self.execute(&format!("apt-get update && apt-get install -y {}", name))
    }

    /// Check whether a package is installed via `dpkg -l`.
    pub fn check_package(&self, name: &str) -> Result<bool> {
        let output = self.execute(&format!("dpkg -l {}", name))?;
        // dpkg -l prints lines like "ii  <pkg>  <ver> ..." for installed packages.
        // A line starting with "ii" (or "hi") means installed.
        Ok(output.lines().any(|line| {
            let trimmed = line.trim();
            trimmed.starts_with("ii") || trimmed.starts_with("hi")
        }))
    }

    /// List all installed packages via `dpkg --list`.
    ///
    /// Returns a vector of package names.
    pub fn list_packages(&self) -> Result<Vec<String>> {
        let output = self.execute("dpkg --list")?;
        let packages: Vec<String> = output
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                // Only consider lines that start with "ii" or "hi" (installed).
                if trimmed.starts_with("ii") || trimmed.starts_with("hi") {
                    // Format: "ii  <name>  <version>  <arch>  <description>"
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        Some(parts[1].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        Ok(packages)
    }

    /// Get the Ubuntu version string via `lsb_release -a`.
    pub fn get_ubuntu_version(&self) -> Result<String> {
        self.execute("lsb_release -a")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// On non-Android (Windows, macOS, Linux desktop) `detect()` must
    /// always return `None`.
    #[test]
    fn detect_on_non_android_returns_none() {
        if !cfg!(target_os = "android") {
            assert!(ProotEnv::detect().is_none());
        }
        // On Android this test is skipped via the if-guard; a real Android
        // CI would exercise the `Some` path separately.
    }

    /// Creating a `ProotEnv` with a non-existent rootfs path should not
    /// panic, and `is_available()` should return `false`.
    #[test]
    fn new_with_nonexistent_path() {
        let env = ProotEnv::new("/nonexistent/ubuntu/rootfs");
        // Construction itself is always fine.
        assert_eq!(env.rootfs_path, PathBuf::from("/nonexistent/ubuntu/rootfs"));
        assert_eq!(env.proot_binary, PathBuf::from("proot"));
        assert!(!env.mounted);
        // On non-Android, is_available is always false.
        // On Android the path doesn't exist, so it's also false.
        assert!(!env.is_available());
    }

    /// `ProotConfig::default()` provides sensible defaults.
    #[test]
    fn proot_config_defaults() {
        let cfg = ProotConfig::default();
        assert_eq!(cfg.proot_command, "proot");
        assert!(!cfg.rootfs_path.as_os_str().is_empty());
        assert!(cfg.extra_args.is_empty());
        assert_eq!(cfg.default_workdir, PathBuf::from("/root"));
    }

    /// `ProotConfig::new` sets rootfs_path and keeps other defaults.
    #[test]
    fn proot_config_new() {
        let cfg = ProotConfig::new("/custom/rootfs");
        assert_eq!(cfg.rootfs_path, PathBuf::from("/custom/rootfs"));
        assert_eq!(cfg.proot_command, "proot"); // defaults preserved
        assert_eq!(cfg.default_workdir, PathBuf::from("/root"));
    }

    /// `ProotEnv::with_proot` allows specifying a custom proot binary.
    #[test]
    fn proot_env_with_proot() {
        let env = ProotEnv::with_proot("/my/rootfs", "/usr/local/bin/proot");
        assert_eq!(env.rootfs_path, PathBuf::from("/my/rootfs"));
        assert_eq!(env.proot_binary, PathBuf::from("/usr/local/bin/proot"));
        assert!(!env.mounted);
    }
}
