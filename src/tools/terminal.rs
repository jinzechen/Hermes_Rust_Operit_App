//! Terminal tool — execute shell commands with timeout and output limits.
//!
//! Supports POSIX shell commands on Unix and cmd/powershell on Windows.
//! Enforces a 30-second timeout and 1 MB output limit per invocation.

use anyhow::{bail, Context};
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

pub struct TerminalTool;

impl TerminalTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TerminalTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolHandler for TerminalTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "terminal".into(),
            description:
                "Execute shell commands with 30-second timeout and 1 MB output limit".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "Shell command to execute"
                    },
                    "workdir": {
                        "type": "string",
                        "description": "Working directory for the command (optional)"
                    }
                },
                "required": ["command"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        let cmd_str = arguments
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("missing 'command' argument"))?;

        let workdir = arguments
            .get("workdir")
            .and_then(|v| v.as_str());

        // Detect OS to choose the right shell.
        let (shell, shell_arg) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let mut child = Command::new(shell)
            .arg(shell_arg)
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .current_dir(if let Some(dir) = workdir { dir } else { "." })
            .spawn()
            .with_context(|| format!("failed to spawn command: {}", cmd_str))?;

        // Wait with timeout.
        let timeout = Duration::from_secs(30);
        let max_output = 1_048_576; // 1 MB

        let mut stdout_buf = Vec::with_capacity(8192);
        let mut stderr_buf = Vec::with_capacity(8192);

        // Take stdout and stderr pipes.
        let mut child_stdout = child.stdout.take().unwrap();
        let mut child_stderr = child.stderr.take().unwrap();

        // Use threads to read with timeout.
        let start = std::time::Instant::now();
        let mut timed_out = false;
        let mut exit_code: Option<i32> = None;

        loop {
            // Check if the process has exited.
            match child.try_wait() {
                Ok(Some(status)) => {
                    exit_code = status.code();
                    // Drain remaining output.
                    let _ = child_stdout.read_to_end(&mut stdout_buf);
                    let _ = child_stderr.read_to_end(&mut stderr_buf);
                    break;
                }
                Ok(None) => {
                    // Still running — check timeout.
                    if start.elapsed() >= timeout {
                        timed_out = true;
                        let _ = child.kill();
                        // Drain what we can.
                        let _ = child_stdout.read_to_end(&mut stdout_buf);
                        let _ = child_stderr.read_to_end(&mut stderr_buf);
                        let _ = child.wait(); // reap
                        break;
                    }
                    // Small sleep to avoid busy-waiting.
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => {
                    bail!("failed to wait on process: {}", e);
                }
            }
        }

        // Truncate output to max_output.
        let stdout_str = String::from_utf8_lossy(
            &stdout_buf[..stdout_buf.len().min(max_output)],
        );
        let stderr_str = String::from_utf8_lossy(
            &stderr_buf[..stderr_buf.len().min(max_output)],
        );

        let total_out = stdout_buf.len() + stderr_buf.len();
        let truncated = total_out > max_output;

        let mut out = format!(
            "## Terminal\n\n```\n{}\n```\n\n",
            cmd_str
        );

        if !stdout_str.is_empty() {
            out.push_str(&format!(
                "### stdout ({}{}bytes)\n```\n{}\n```\n\n",
                if stdout_buf.len() > max_output {
                    "truncated, "
                } else {
                    ""
                },
                stdout_buf.len(),
                stdout_str
            ));
        }

        if !stderr_str.is_empty() {
            out.push_str(&format!(
                "### stderr ({}{}bytes)\n```\n{}\n```\n\n",
                if stderr_buf.len() > max_output {
                    "truncated, "
                } else {
                    ""
                },
                stderr_buf.len(),
                stderr_str
            ));
        }

        if timed_out {
            out.push_str("> ⚠ Command timed out after 30 seconds and was killed.\n\n");
        }

        if truncated {
            out.push_str("> ⚠ Output exceeded 1 MB limit and was truncated.\n\n");
        }

        if let Some(code) = exit_code {
            out.push_str(&format!("**Exit code:** {}\n", code));
        } else if !timed_out {
            out.push_str("**Exit code:** (none — killed?)\n");
        }

        Ok(out)
    }
}
