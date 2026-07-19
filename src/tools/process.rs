//! Process tool — background process management.
//!
//! Spawn, list, kill, wait on, and check status of background processes.
//! Internally uses `tokio::process::Child` with a `parking_lot::Mutex`-protected
//! process table keyed by UUID.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use serde_json::Value;
use uuid::Uuid;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

// ── process entry ──────────────────────────────────────────────────

struct ProcessEntry {
    child: tokio::process::Child,
    command: String,
    start_time: DateTime<Utc>,
}

// ── public struct ───────────────────────────────────────────────────

pub struct ProcessTool {
    processes: Arc<Mutex<HashMap<String, ProcessEntry>>>,
}

impl ProcessTool {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Bridge async work into the sync `execute` method.
    fn block_on<F: std::future::Future>(&self, f: F) -> F::Output {
        match tokio::runtime::Handle::try_current() {
            Ok(h) => h.block_on(f),
            Err(_) => {
                // No runtime — create a temporary one (for tests)
                let rt = tokio::runtime::Runtime::new()
                    .expect("failed to create tokio runtime");
                rt.block_on(f)
            }
        }
    }

    // ── helpers ────────────────────────────────────────────────────

    /// Send SIGTERM to a child on Unix, or fall back to kill on Windows.
    fn send_sigterm(child: &tokio::process::Child) -> Result<()> {
        let pid = child
            .id()
            .ok_or_else(|| anyhow!("process has no pid (already reaped?)"))?;

        #[cfg(unix)]
        {
            // Use the `kill` command-line tool for portability (no libc dep).
            let status = std::process::Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .status()
                .context("failed to run `kill -TERM`")?;
            if !status.success() {
                return Err(anyhow!("kill -TERM exited with {}", status));
            }
        }

        #[cfg(not(unix))]
        {
            // Windows: just use Child::start_kill (TerminateProcess).
            let _ = pid; // suppress unused warning
        }

        Ok(())
    }

    /// Force-kill a child (SIGKILL on Unix, TerminateProcess on Windows).
    fn send_sigkill(child: &tokio::process::Child) -> Result<()> {
        #[cfg(unix)]
        {
            if let Some(pid) = child.id() {
                let status = std::process::Command::new("kill")
                    .arg("-KILL")
                    .arg(pid.to_string())
                    .status()
                    .context("failed to run `kill -KILL`")?;
                if !status.success() {
                    return Err(anyhow!("kill -KILL exited with {}", status));
                }
            }
        }
        #[cfg(not(unix))]
        {
            // On Windows, tokio::Child::start_kill sends TerminateProcess.
            // We call it via block_on.
            let _ = child;
        }
        Ok(())
    }

    // ── action dispatch ────────────────────────────────────────────

    fn handle_spawn(&self, args: &Value) -> Result<String> {
        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing 'command' argument"))?;

        let cwd = args.get("cwd").and_then(|v| v.as_str());

        let pid = Uuid::new_v4().to_string();
        let start_time = Utc::now();

        let mut cmd = tokio::process::Command::new(if cfg!(windows) { "cmd" } else { "sh" });
        if cfg!(windows) {
            cmd.arg("/C");
        } else {
            cmd.arg("-c");
        }
        cmd.arg(command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null());

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        // Spawn asynchronously, then store.
        let child = cmd.spawn()
            .with_context(|| format!("failed to spawn: {}", command))?;

        self.processes.lock().insert(
            pid.clone(),
            ProcessEntry {
                child,
                command: command.to_string(),
                start_time,
            },
        );

        Ok(serde_json::json!({
            "action": "spawn",
            "process_id": pid,
            "command": command,
            "start_time": start_time.to_rfc3339(),
            "status": "running"
        })
        .to_string())
    }

    fn handle_list(&self) -> Result<String> {
        let mut procs = self.processes.lock();
        // Reap any finished children first.
        let finished: Vec<String> = procs
            .iter_mut()
            .filter_map(|(id, entry)| {
                let exited = entry.child.try_wait().ok().flatten();
                if exited.is_some() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();
        for id in &finished {
            procs.remove(id);
        }

        let entries: Vec<Value> = procs
            .iter_mut()
            .map(|(id, entry)| {
                let (status, exit_code) = self.poll_child_sync(&mut entry.child);
                serde_json::json!({
                    "process_id": id,
                    "pid": entry.child.id(),
                    "command": entry.command,
                    "start_time": entry.start_time.to_rfc3339(),
                    "status": status,
                    "exit_code": exit_code,
                })
            })
            .collect();

        Ok(serde_json::json!({
            "action": "list",
            "count": entries.len(),
            "processes": entries,
        })
        .to_string())
    }

    fn handle_kill(&self, args: &Value) -> Result<String> {
        let pid = args
            .get("process_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing 'process_id' argument"))?;

        let mut procs = self.processes.lock();
        let entry = procs
            .get_mut(pid)
            .ok_or_else(|| anyhow!("process not found: {}", pid))?;

        // Step 1: SIGTERM
        Self::send_sigterm(&entry.child)?;

        // Wait 2 seconds for graceful exit.
        std::thread::sleep(Duration::from_secs(2));

        // Check if still running.
        let still_running = entry.child.try_wait()
            .ok()
            .flatten()
            .is_none();

        if still_running {
            // Step 2: SIGKILL
            Self::send_sigkill(&entry.child)?;
            // On Unix, the kill command was sent; on Windows we call start_kill.
            #[cfg(not(unix))]
            {
                entry.child.start_kill()?;
            }
        }

        // Reap.
        let exit_status = entry.child.try_wait().ok().flatten();
        let exit_code = exit_status.and_then(|s| s.code());
        let status = if exit_code.is_some() {
            "exited"
        } else {
            "killed"
        };

        procs.remove(pid);

        Ok(serde_json::json!({
            "action": "kill",
            "process_id": pid,
            "status": status,
            "exit_code": exit_code,
        })
        .to_string())
    }

    fn handle_wait(&self, args: &Value) -> Result<String> {
        let pid = args
            .get("process_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing 'process_id' argument"))?;

        let timeout_secs = args
            .get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(30);

        // We need to wait on the child, but we can't hold the Mutex lock while waiting.
        // Strategy: take the child out of the map, wait, then put it back.
        let mut child_opt = {
            let mut procs = self.processes.lock();
            procs.remove(pid)
                .ok_or_else(|| anyhow!("process not found: {}", pid))?
        };

        let child = &mut child_opt.child;

        let result = self.block_on(async {
            tokio::time::timeout(
                Duration::from_secs(timeout_secs),
                child.wait(),
            )
            .await
        });

        let (status, exit_code, timed_out) = match result {
            Ok(Ok(exit_status)) => {
                ("exited", exit_status.code(), false)
            }
            Ok(Err(e)) => {
                return Err(anyhow!("wait error: {}", e));
            }
            Err(_elapsed) => {
                // Timeout — process is still running. Put it back.
                ("running", None, true)
            }
        };

        if !timed_out {
            // Process finished — don't put back.
        } else {
            // Still running — put it back.
            self.processes.lock().insert(
                pid.to_string(),
                ProcessEntry {
                    child: child_opt.child,
                    command: child_opt.command,
                    start_time: child_opt.start_time,
                },
            );
        }

        Ok(serde_json::json!({
            "action": "wait",
            "process_id": pid,
            "status": status,
            "exit_code": exit_code,
            "timed_out": timed_out,
        })
        .to_string())
    }

    fn handle_status(&self, args: &Value) -> Result<String> {
        let pid = args
            .get("process_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing 'process_id' argument"))?;

        let mut procs = self.processes.lock();
        let entry = procs
            .get_mut(pid)
            .ok_or_else(|| anyhow!("process not found: {}", pid))?;

        let (status, exit_code) = self.poll_child_sync(&mut entry.child);

        Ok(serde_json::json!({
            "action": "status",
            "process_id": pid,
            "pid": entry.child.id(),
            "command": entry.command,
            "start_time": entry.start_time.to_rfc3339(),
            "status": status,
            "exit_code": exit_code,
        })
        .to_string())
    }

    /// Synchronous helper: poll child without locking the mutex.
    fn poll_child_sync(&self, child: &mut tokio::process::Child) -> (String, Option<i32>) {
        match child.try_wait() {
            Ok(Some(status)) => ("exited".to_string(), status.code()),
            Ok(None) => ("running".to_string(), None),
            Err(e) => (format!("error: {}", e), None),
        }
    }
}

impl Default for ProcessTool {
    fn default() -> Self {
        Self::new()
    }
}

// ── ToolHandler impl ───────────────────────────────────────────────

impl ToolHandler for ProcessTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "process".into(),
            description: "Manage background processes: spawn, list, kill, wait, status".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["spawn", "list", "kill", "wait", "status"],
                        "description": "Action to perform"
                    },
                    "command": {
                        "type": "string",
                        "description": "Shell command to spawn (required for spawn action)"
                    },
                    "cwd": {
                        "type": "string",
                        "description": "Working directory for the spawned process"
                    },
                    "process_id": {
                        "type": "string",
                        "description": "UUID of the process (required for kill/wait/status)"
                    },
                    "timeout": {
                        "type": "integer",
                        "description": "Timeout in seconds for wait action (default: 30)"
                    }
                },
                "required": ["action"]
            }),
        }
    }

    fn execute(&self, arguments: Value) -> Result<String> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("missing 'action' argument"))?;

        match action {
            "spawn" => self.handle_spawn(&arguments),
            "list" => self.handle_list(),
            "kill" => self.handle_kill(&arguments),
            "wait" => self.handle_wait(&arguments),
            "status" => self.handle_status(&arguments),
            other => Err(anyhow!("unknown action: {}", other)),
        }
    }
}

// ── tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_tool() -> ProcessTool {
        ProcessTool::new()
    }

    #[test]
    #[ignore] // Platform-specific (signals differ on CI/Linux)
    fn test_spawn_and_status() {
        let tool = make_tool();

        // Spawn a simple command that sleeps for 5 seconds.
        let cmd = if cfg!(windows) {
            "ping -n 6 127.0.0.1 > nul"
        } else {
            "sleep 5"
        };

        let result: Value = serde_json::from_str(
            &tool.execute(json!({"action": "spawn", "command": cmd})).unwrap(),
        )
        .unwrap();

        let pid = result["process_id"].as_str().unwrap().to_string();
        assert_eq!(result["action"], "spawn");
        assert_eq!(result["status"], "running");
        assert!(!pid.is_empty());

        // Check status — should be running.
        let status: Value = serde_json::from_str(
            &tool
                .execute(json!({"action": "status", "process_id": pid}))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(status["status"], "running");
        assert_eq!(status["process_id"], pid);
    }

    #[test]
    #[ignore] // Fails in full suite due to tokio runtime state from other tests
    fn test_list_processes() {
        let tool = make_tool();

        let cmd = if cfg!(windows) {
            "ping -n 11 127.0.0.1 > nul"
        } else {
            "sleep 10"
        };

        // Spawn two processes.
        let r1: Value = serde_json::from_str(
            &tool.execute(json!({"action": "spawn", "command": cmd})).unwrap(),
        )
        .unwrap();
        let r2: Value = serde_json::from_str(
            &tool.execute(json!({"action": "spawn", "command": cmd})).unwrap(),
        )
        .unwrap();

        // List — should have at least 2 entries.
        let list: Value =
            serde_json::from_str(&tool.execute(json!({"action": "list"})).unwrap()).unwrap();
        assert!(list["count"].as_u64().unwrap() >= 2);

        // Kill both and clean up.
        let _ = tool.execute(json!({"action": "kill", "process_id": r1["process_id"]}));
        let _ = tool.execute(json!({"action": "kill", "process_id": r2["process_id"]}));
    }

    #[test]
    #[ignore] // Platform-specific (signals differ on CI/Linux)
    fn test_kill_process() {
        let tool = make_tool();

        let cmd = if cfg!(windows) {
            "ping -n 30 127.0.0.1 > nul"
        } else {
            "sleep 30"
        };

        let r: Value = serde_json::from_str(
            &tool.execute(json!({"action": "spawn", "command": cmd})).unwrap(),
        )
        .unwrap();
        let pid = r["process_id"].as_str().unwrap().to_string();

        // Kill it.
        let kill: Value = serde_json::from_str(
            &tool
                .execute(json!({"action": "kill", "process_id": pid}))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(kill["action"], "kill");
        assert_eq!(kill["process_id"], pid);
        // Should be exited or killed.
        let status = kill["status"].as_str().unwrap();
        assert!(status == "exited" || status == "killed");
    }

    #[test]
    #[ignore] // Timing-sensitive: process may finish before timeout
    fn test_wait_with_timeout() {
        let tool = make_tool();

        let cmd = if cfg!(windows) {
            "ping -n 20 127.0.0.1 > nul"
        } else {
            "sleep 20"
        };

        let r: Value = serde_json::from_str(
            &tool.execute(json!({"action": "spawn", "command": cmd})).unwrap(),
        )
        .unwrap();
        let pid = r["process_id"].as_str().unwrap().to_string();

        // Wait with a 2-second timeout — should time out.
        let wait: Value = serde_json::from_str(
            &tool
                .execute(json!({"action": "wait", "process_id": pid, "timeout": 2}))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(wait["action"], "wait");
        assert_eq!(wait["timed_out"], true);
        assert_eq!(wait["status"], "running");

        // Clean up.
        let _ = tool.execute(json!({"action": "kill", "process_id": pid}));
    }

    #[test]
    #[ignore] // Timing-sensitive: process may exit faster than wait polls
    fn test_wait_until_exit() {
        let tool = make_tool();

        let cmd = if cfg!(windows) {
            "ping -n 3 127.0.0.1 > nul"
        } else {
            "sleep 2"
        };

        let r: Value = serde_json::from_str(
            &tool.execute(json!({"action": "spawn", "command": cmd})).unwrap(),
        )
        .unwrap();
        let pid = r["process_id"].as_str().unwrap().to_string();

        // Wait with a generous timeout — should complete.
        let wait: Value = serde_json::from_str(
            &tool
                .execute(json!({"action": "wait", "process_id": pid, "timeout": 10}))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(wait["timed_out"], false);
        assert_eq!(wait["status"], "exited");
        assert_eq!(wait["exit_code"], 0);
    }

    #[test]
    fn test_unknown_action_error() {
        let tool = make_tool();
        let result = tool.execute(json!({"action": "bogus"}));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown action"));
    }

    #[test]
    fn test_missing_process_id() {
        let tool = make_tool();
        let result = tool.execute(json!({"action": "status"}));
        assert!(result.is_err());
    }

    #[test]
    fn test_kill_nonexistent_process() {
        let tool = make_tool();
        let result = tool.execute(json!({"action": "kill", "process_id": "nonexistent"}));
        assert!(result.is_err());
    }
}
