//! Integration tests for the Sandbox — process sandbox with timeout,
//! output truncation, path whitelist, stderr capture, and stdin support.
//!
//! These tests execute real system commands and verify correctness of
//! sandbox enforcement features.

use anyhow::Result;
use hermes_operit_core::environment::sandbox::{Sandbox, SandboxResult};
use std::time::Duration;

// ── test_sandbox_echo ────────────────────────────────────────────────────────

#[test]
fn test_sandbox_echo() -> Result<()> {
    let sandbox = Sandbox::builder().build();
    let result = sandbox.execute("echo hello_world");

    assert_eq!(result.exit_code, 0);
    assert!(
        result.stdout.contains("hello_world"),
        "stdout: {}",
        result.stdout
    );
    assert!(!result.timed_out);
    assert!(!result.truncated);

    Ok(())
}

// ── test_sandbox_timeout ─────────────────────────────────────────────────────

#[test]
fn test_sandbox_timeout() -> Result<()> {
    let sandbox = Sandbox::builder().timeout(1).build();

    // A command that will run long enough to trigger the timeout.
    #[cfg(windows)]
    let cmd = "for /L %i in (1,1,5000000) do @cd > nul";
    #[cfg(not(windows))]
    let cmd = "sleep 5";

    let result = sandbox.execute(cmd);
    assert!(result.timed_out, "expected timeout but got: {:?}", result);
    assert_eq!(result.exit_code, -1);

    Ok(())
}

// ── test_sandbox_output_truncation ───────────────────────────────────────────

#[test]
fn test_sandbox_output_truncation() -> Result<()> {
    let sandbox = Sandbox::builder().max_output(100).build();

    // Generate enough output to overflow the 100-byte limit.
    #[cfg(windows)]
    let cmd = "for /L %i in (1,1,200) do @echo %i";
    #[cfg(not(windows))]
    let cmd = "i=0; while [ $i -lt 200 ]; do echo $i; i=$((i+1)); done";

    let result = sandbox.execute(cmd);
    assert!(result.truncated, "expected truncated but got: {:?}", result);
    // stdout should be capped around 100 bytes (allow some margin).
    assert!(
        result.stdout.len() <= 300,
        "stdout len {} should be <= 300",
        result.stdout.len()
    );

    Ok(())
}

// ── test_sandbox_path_whitelist ──────────────────────────────────────────────

#[test]
fn test_sandbox_path_whitelist_allow() -> Result<()> {
    let tmp = std::env::temp_dir();
    let sandbox = Sandbox::builder().allow_path(&tmp).build();

    // Reference the allowed temp directory.
    #[cfg(windows)]
    let cmd = format!("dir {}", tmp.display());
    #[cfg(not(windows))]
    let cmd = format!("ls {}", tmp.display());

    let result = sandbox.execute(&cmd);
    assert_eq!(result.exit_code, 0, "result: {:?}", result);

    Ok(())
}

#[test]
fn test_sandbox_path_whitelist_deny() -> Result<()> {
    let sandbox = Sandbox::builder().allow_path("/only/this/dir").build();

    let result = sandbox.execute("cat /etc/hosts 2>/dev/null");
    assert!(
        result.stderr.contains("access denied"),
        "expected 'access denied' in stderr, got: {}",
        result.stderr
    );
    assert_ne!(result.exit_code, 0);

    Ok(())
}

// ── test_sandbox_stderr_capture ──────────────────────────────────────────────

#[test]
fn test_sandbox_stderr_capture() -> Result<()> {
    let sandbox = Sandbox::builder().build();

    #[cfg(windows)]
    let cmd = "echo error_message 1>&2";
    #[cfg(not(windows))]
    let cmd = "echo error_message >&2";

    let result = sandbox.execute(cmd);
    assert!(
        result.stderr.contains("error_message"),
        "stderr: {}",
        result.stderr
    );
    assert!(
        result.stdout.is_empty() || !result.stdout.contains("error_message"),
        "stdout should not contain the stderr message: {}",
        result.stdout
    );

    Ok(())
}

// ── test_sandbox_stdin ───────────────────────────────────────────────────────

#[test]
fn test_sandbox_stdin() -> Result<()> {
    let sandbox = Sandbox::builder().build();

    // `sort` reads from stdin and outputs sorted lines.
    // Available as sort.exe in System32 on Windows, /usr/bin/sort on Unix.
    let result = sandbox.execute_with_stdin("sort", "banana\napple\ncherry\n");

    assert!(result.stdout.contains("apple"), "stdout: {}", result.stdout);
    assert!(
        result.stdout.contains("banana"),
        "stdout: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains("cherry"),
        "stdout: {}",
        result.stdout
    );
    assert_eq!(result.exit_code, 0, "result: {:?}", result);

    Ok(())
}

// ── test_sandbox_non_zero_exit ───────────────────────────────────────────────

#[test]
fn test_sandbox_non_zero_exit() -> Result<()> {
    let sandbox = Sandbox::builder().build();

    // Exit with a specific code.
    #[cfg(windows)]
    let cmd = "exit 42";
    #[cfg(not(windows))]
    let cmd = "exit 42";

    let result = sandbox.execute(cmd);
    assert_eq!(result.exit_code, 42, "result: {:?}", result);
    assert!(!result.timed_out);

    Ok(())
}

// ── test_sandbox_duration ────────────────────────────────────────────────────

#[test]
fn test_sandbox_duration() -> Result<()> {
    let sandbox = Sandbox::builder().build();
    let result = sandbox.execute("echo instant");

    assert!(
        result.duration > Duration::ZERO,
        "duration: {:?}",
        result.duration
    );
    assert_eq!(result.exit_code, 0);

    Ok(())
}

// ── test_sandbox_defaults ────────────────────────────────────────────────────

#[test]
fn test_sandbox_defaults() -> Result<()> {
    // Verify that a sandbox with default settings works for basic commands.
    let sandbox = Sandbox::builder().build();
    let result = sandbox.execute("echo ok");

    assert_eq!(result.exit_code, 0);
    assert!(!result.timed_out);
    assert!(!result.truncated);

    Ok(())
}

// ── test_sandbox_multiple_allowed_paths ──────────────────────────────────────

#[test]
fn test_sandbox_multiple_allowed_paths() -> Result<()> {
    let tmp = std::env::temp_dir();
    let home = std::env::current_dir()?;
    let sandbox = Sandbox::builder()
        .allow_path(&tmp)
        .allow_path(&home)
        .build();

    let result = sandbox.execute("echo ok");
    assert_eq!(result.exit_code, 0, "result: {:?}", result);

    Ok(())
}

// ── test_sandbox_network_blocking ────────────────────────────────────────────

#[test]
fn test_sandbox_network_blocking() -> Result<()> {
    // By default, the sandbox sets proxy env vars to block network.
    let sandbox = Sandbox::builder().build();

    // Verify HTTP_PROXY is set to "nope".
    #[cfg(windows)]
    let cmd = "echo %HTTP_PROXY%";
    #[cfg(not(windows))]
    let cmd = "echo $HTTP_PROXY";

    let result = sandbox.execute(cmd);
    assert!(result.stdout.contains("nope"), "stdout: {}", result.stdout);

    Ok(())
}

#[test]
fn test_sandbox_allow_network() -> Result<()> {
    // When allow_network() is called, proxy vars should NOT be blocked.
    let sandbox = Sandbox::builder().allow_network().build();

    #[cfg(windows)]
    let cmd = "echo %HTTP_PROXY%";
    #[cfg(not(windows))]
    let cmd = "echo $HTTP_PROXY";

    let result = sandbox.execute(cmd);
    // Without network blocking, HTTP_PROXY should be empty or inherit from parent.
    assert!(
        !result.stdout.contains("nope"),
        "HTTP_PROXY should not be 'nope' when network is allowed: {}",
        result.stdout
    );

    Ok(())
}

// ── test_sandbox_command_failure ─────────────────────────────────────────────

#[test]
fn test_sandbox_command_failure() -> Result<()> {
    let sandbox = Sandbox::builder().build();

    // Execute a non-existent command.
    #[cfg(windows)]
    let cmd = "nonexistent_command_xyz_123 2> nul";
    #[cfg(not(windows))]
    let cmd = "nonexistent_command_xyz_123 2>/dev/null";

    let result = sandbox.execute(cmd);
    // Should be non-zero exit code.
    assert_ne!(result.exit_code, 0, "result: {:?}", result);

    Ok(())
}

// ── test_sandbox_work_dir ────────────────────────────────────────────────────

#[test]
fn test_sandbox_work_dir() -> Result<()> {
    let tmp = std::env::temp_dir();
    let sandbox = Sandbox::builder().work_dir(&tmp).build();

    // On Windows `cd` shows current directory, on Unix `pwd`.
    #[cfg(windows)]
    let cmd = "cd";
    #[cfg(not(windows))]
    let cmd = "pwd";

    let result = sandbox.execute(cmd);
    assert_eq!(result.exit_code, 0, "result: {:?}", result);

    Ok(())
}
