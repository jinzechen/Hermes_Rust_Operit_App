//! Sandbox wrapper — restricted execution environment.
//!
//! Provides `Sandbox`, a placeholder for running commands with
//! restricted filesystem access.  TODO: integrate seccomp (Linux)
//! and landlock for real sandboxing.

use std::path::PathBuf;
use std::process::Command as StdCommand;

use anyhow::{bail, Result};

/// A sandbox that can execute commands with restricted filesystem access.
///
/// Currently a thin wrapper around `std::process::Command`.
pub struct Sandbox;

impl Sandbox {
    /// Create a new sandbox.
    pub fn new() -> Self {
        Self
    }

    /// Run a command, restricting access to `allowed_paths`.
    ///
    /// On Linux this will eventually use seccomp + landlock.
    /// For now it runs the command unrestricted.
    pub fn restricted_execute(
        &self,
        command: &str,
        allowed_paths: &[PathBuf],
    ) -> Result<(String, String, i32)> {
        if !cfg!(target_os = "linux") {
            bail!("sandbox restricted_execute is only supported on Linux hosts (TODO: Windows job objects)");
        }

        // Split command.
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            bail!("empty command");
        }

        // TODO: Apply seccomp filter to restrict syscalls.
        // TODO: Apply landlock rules for `allowed_paths`.

        let _ = allowed_paths; // Used once seccomp/landlock is integrated.

        let output = StdCommand::new(parts[0]).args(&parts[1..]).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, code))
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_creation() {
        let _sandbox = Sandbox::new();
    }
}
