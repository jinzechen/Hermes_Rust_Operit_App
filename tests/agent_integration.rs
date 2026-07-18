//! Integration tests for the Agent — end-to-end verification of
//! AgentManager creation, message sending, tool registry, session management,
//! runtime configuration mutation, and health checks.
//!
//! These tests use real components (no mocking). The LLM provider will fail
//! with a connection error during `send_message`, but we can still verify
//! that sessions are created, history is recorded, and the agent does not
//! panic.

use anyhow::Result;
use hermes_operit_core::core::agent::AgentManager;
use hermes_operit_core::core::config::AppConfig;
use hermes_operit_core::core::tool_registry::{ToolHandler, ToolRegistry, ToolSchema};
use hermes_operit_core::tools::{
    BrowserTool, FileSystemTool, MarkdownTool, TerminalTool, VisionTool, WebTool,
};
use serde_json::Value;
use std::sync::Arc;

// ── helpers ──────────────────────────────────────────────────────────────────

fn test_config() -> AppConfig {
    AppConfig {
        model: "test-model".into(),
        api_key: "sk-test".into(),
        api_endpoint: "https://api.example.com/v1/chat/completions".into(),
        temperature: 0.5,
        max_tokens: 1024,
    }
}

/// A simple counting tool for testing tool registration and execution.
#[derive(Debug)]
struct CounterTool {
    count: std::sync::atomic::AtomicUsize,
}

impl CounterTool {
    fn new() -> Self {
        Self {
            count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn count(&self) -> usize {
        self.count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl ToolHandler for CounterTool {
    fn execute(&self, _params: Value) -> Result<String> {
        self.count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(format!("Count: {}", self.count()))
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "counter".into(),
            description: "Increments and returns a counter".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {}
            }),
        }
    }
}

/// A tool that echoes back whatever string is passed to it.
#[derive(Debug)]
struct EchoTool;

impl ToolHandler for EchoTool {
    fn execute(&self, params: Value) -> Result<String> {
        let text = params
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("<no text>");
        Ok(text.to_string())
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "echo".into(),
            description: "Echoes back the provided text".into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "text": { "type": "string", "description": "Text to echo" }
                },
                "required": ["text"]
            }),
        }
    }
}

// ── test_agent_creation ──────────────────────────────────────────────────────

#[test]
fn test_agent_creation() -> Result<()> {
    // Create an AgentManager without default tools.
    let agent = AgentManager::new(test_config())?;

    // Verify initial state.
    assert_eq!(agent.tool_count(), 0);
    assert_eq!(agent.session_count(), 0);
    assert!(agent.list_tool_names().is_empty());
    assert!(agent.list_sessions().is_empty());

    Ok(())
}

// ── test_send_message ────────────────────────────────────────────────────────

#[test]
fn test_send_message() -> Result<()> {
    let agent = AgentManager::new(test_config())?;

    // send_message will fail because the LLM endpoint is unreachable,
    // but the session and user message should still be created.
    let result = agent.send_message("test-session", "Hello, agent!");

    // The LLM call will fail, but we can verify the session was created.
    assert!(agent.has_session("test-session"));

    // The session history should contain the user message.
    let history = agent.get_session_history("test-session");
    assert!(!history.is_empty(), "Expected at least the user message");
    assert_eq!(history[0].role, "user");
    assert_eq!(history[0].content, "Hello, agent!");

    // Verify that we get an error from send_message (LLM unreachable).
    match result {
        Ok(resp) => {
            // If it somehow succeeded, verify response is non-empty.
            assert!(!resp.session_id.is_empty());
        }
        Err(_) => {
            // Expected: the LLM endpoint is unreachable.
        }
    }

    Ok(())
}

// ── test_tool_registry ───────────────────────────────────────────────────────

#[test]
fn test_tool_registry() -> Result<()> {
    // Test the standalone ToolRegistry.
    let mut registry = ToolRegistry::new();
    assert_eq!(registry.count(), 0);
    assert!(registry.list_all().is_empty());

    // Register the echo tool.
    registry.register("echo", Arc::new(EchoTool));
    assert_eq!(registry.count(), 1);
    assert_eq!(registry.list_all(), vec!["echo"]);

    // Look up by name.
    assert!(registry.get("echo").is_some());
    assert!(registry.get("nonexistent").is_none());

    // Execute the tool.
    let result = registry.execute_tool(
        "echo",
        serde_json::json!({"text": "hello from registry"}),
    )?;
    assert_eq!(result, "hello from registry");

    // Execute a missing tool.
    let err = registry
        .execute_tool("nonexistent", serde_json::json!({}))
        .unwrap_err();
    assert!(err.to_string().contains("Tool not found"));

    // List tool schemas.
    let schemas = registry.list_tool_schemas();
    assert_eq!(schemas.len(), 1);
    assert_eq!(schemas[0].name, "echo");
    assert!(!schemas[0].description.is_empty());

    Ok(())
}

#[test]
fn test_agent_register_and_execute_tool() -> Result<()> {
    let agent = AgentManager::new(test_config())?;

    // Register the echo tool.
    agent.register_tool(Box::new(EchoTool));
    assert_eq!(agent.tool_count(), 1);

    let names = agent.list_tool_names();
    assert!(names.contains(&"echo".to_string()));

    // Get tool descriptions.
    let descs = agent.get_tool_descriptions();
    assert_eq!(descs.len(), 1);
    assert_eq!(descs[0].0, "echo");
    assert!(!descs[0].1.is_empty());

    Ok(())
}

// ── test_session_management ──────────────────────────────────────────────────

#[test]
fn test_session_management() -> Result<()> {
    let agent = AgentManager::new(test_config())?;

    // Create a session by sending a message.
    let _ = agent.send_message("sess-a", "first message");

    assert!(agent.has_session("sess-a"));
    assert_eq!(agent.session_count(), 1);

    // Create a second session.
    let _ = agent.send_message("sess-b", "second message");

    assert!(agent.has_session("sess-b"));
    assert_eq!(agent.session_count(), 2);

    let sessions = agent.list_sessions();
    assert_eq!(sessions.len(), 2);
    assert!(sessions.contains(&"sess-a".to_string()));
    assert!(sessions.contains(&"sess-b".to_string()));

    // Clear one session.
    agent.clear_session("sess-a");
    assert!(!agent.has_session("sess-a"));
    assert!(agent.has_session("sess-b"));
    assert_eq!(agent.session_count(), 1);

    // Clear the remaining session.
    agent.clear_session("sess-b");
    assert_eq!(agent.session_count(), 0);
    assert!(agent.list_sessions().is_empty());

    // Clearing a non-existent session is a no-op.
    agent.clear_session("no-such-session");
    assert_eq!(agent.session_count(), 0);

    Ok(())
}

#[test]
fn test_session_history_persistence() -> Result<()> {
    let agent = AgentManager::new(test_config())?;

    let _ = agent.send_message("hist", "message one");
    let _ = agent.send_message("hist", "message two");

    let history = agent.get_session_history("hist");
    // Should have at least the two user messages.
    assert!(history.len() >= 2);
    assert_eq!(history[0].role, "user");
    assert_eq!(history[0].content, "message one");
    assert_eq!(history[1].role, "user");
    assert_eq!(history[1].content, "message two");

    // Non-existent session returns empty.
    assert!(agent.get_session_history("no-such").is_empty());

    Ok(())
}

#[test]
fn test_clean_expired_sessions() -> Result<()> {
    let agent = AgentManager::new(test_config())?;

    let _ = agent.send_message("fresh", "hello");

    // With a very small timeout, all sessions should be expired.
    // But we can't mutate session_timeout after construction without
    // a mutable reference. Test that clean_expired_sessions works
    // without panicking with the default 1-hour timeout.
    let cleaned = agent.clean_expired_sessions()?;
    assert_eq!(cleaned, 0); // Fresh sessions shouldn't expire.

    Ok(())
}

// ── test_config_mutation ─────────────────────────────────────────────────────

#[test]
fn test_config_mutation() -> Result<()> {
    let mut agent = AgentManager::new(test_config())?;

    // Read initial config.
    assert_eq!(agent.config().model, "test-model");

    // Mutate model.
    agent.set_config("model", "claude-sonnet-4")?;
    assert_eq!(agent.config().model, "claude-sonnet-4");

    // Mutate endpoint (both "api_endpoint" and "endpoint" keys).
    agent.set_config("endpoint", "https://api.anthropic.com/v1/messages")?;
    assert_eq!(
        agent.config().api_endpoint,
        "https://api.anthropic.com/v1/messages"
    );

    // Mutate temperature.
    agent.set_config("temperature", "0.3")?;
    assert!((agent.config().temperature - 0.3).abs() < 0.001);

    // Invalid temperature.
    let err = agent.set_config("temperature", "not_a_float").unwrap_err();
    assert!(err.to_string().contains("must be a float"));

    // Mutate max_tokens.
    agent.set_config("max_tokens", "8192")?;
    assert_eq!(agent.config().max_tokens, 8192);

    // Invalid max_tokens.
    let err = agent.set_config("max_tokens", "not_a_number").unwrap_err();
    assert!(err.to_string().contains("must be a u32"));

    // Mutate api_key.
    agent.set_config("api_key", "sk-my-new-key")?;
    assert_eq!(agent.config().api_key, "sk-my-new-key");

    // Unknown key.
    let err = agent.set_config("foobar", "value").unwrap_err();
    assert!(err.to_string().contains("Unknown config key"));

    Ok(())
}

// ── test_health_check ────────────────────────────────────────────────────────

#[test]
fn test_health_check() -> Result<()> {
    let agent = AgentManager::with_default_tools(test_config())?;
    let status = agent.health_check();

    // Basic fields should be populated correctly.
    assert_eq!(status.model, "test-model");
    assert_eq!(
        status.endpoint,
        "https://api.example.com/v1/chat/completions"
    );
    assert!(status.tool_count >= 5, "Expected at least 5 tools");
    assert_eq!(status.session_count, 0);
    assert!(status.uptime_secs >= 0.0);

    // provider_ok should be a bool (may be true or false depending on
    // network connectivity — just verify it doesn't panic).
    let _: bool = status.provider_ok;

    Ok(())
}

// ── test_with_default_tools ──────────────────────────────────────────────────

#[test]
fn test_with_default_tools() -> Result<()> {
    let agent = AgentManager::with_default_tools(test_config())?;

    // Should have at least 5 of the 6 default tools.
    let count = agent.tool_count();
    assert!(
        count >= 5,
        "Expected at least 5 default tools, got {}",
        count
    );

    let names = agent.list_tool_names();
    let expected_tools = ["filesystem", "markdown", "terminal", "browser", "vision"];
    for expected in &expected_tools {
        assert!(
            names.contains(&expected.to_string()),
            "Missing default tool: {}",
            expected
        );
    }

    // Each tool should have a non-empty description.
    let descs = agent.get_tool_descriptions();
    assert!(!descs.is_empty());
    for (name, desc) in &descs {
        assert!(!name.is_empty(), "Tool has empty name");
        assert!(!desc.is_empty(), "Tool '{}' has empty description", name);
    }

    // Verify tool schemas are buildable.
    let summary = agent.get_memory_summary();
    assert!(summary.contains("Agent Memory Summary"));
    assert!(summary.contains("Registered tools"));
    assert!(summary.contains("test-model"));

    Ok(())
}

// ── test_multiple_sessions ───────────────────────────────────────────────────

#[test]
fn test_multiple_sessions() -> Result<()> {
    let agent = AgentManager::new(test_config())?;

    let _ = agent.send_message("s1", "msg1");
    let _ = agent.send_message("s2", "msg2");
    let _ = agent.send_message("s3", "msg3");

    let sessions = agent.list_sessions();
    assert_eq!(sessions.len(), 3);
    assert_eq!(agent.session_count(), 3);

    assert!(agent.has_session("s1"));
    assert!(agent.has_session("s2"));
    assert!(agent.has_session("s3"));

    // Each session should have its own history.
    assert!(!agent.get_session_history("s1").is_empty());
    assert!(!agent.get_session_history("s2").is_empty());
    assert!(!agent.get_session_history("s3").is_empty());

    Ok(())
}

// ── test_direct_tool_execution ───────────────────────────────────────────────

#[test]
fn test_direct_tool_execution() -> Result<()> {
    // Verify individual tool schemas return valid JSON Schema objects.
    let tools: Vec<(&str, Box<dyn ToolHandler>)> = vec![
        ("markdown", Box::new(MarkdownTool::new())),
        ("terminal", Box::new(TerminalTool::new())),
        ("browser", Box::new(BrowserTool::new())),
        ("vision", Box::new(VisionTool::new())),
    ];

    for (name, tool) in &tools {
        let schema = tool.schema();
        assert_eq!(schema.name, *name, "Tool schema name mismatch for {}", name);
        assert!(!schema.description.is_empty(), "Tool {} has empty description", name);

        // parameters should be a valid JSON object with "type" field.
        let params = &schema.parameters;
        assert!(params.is_object(), "Tool {} parameters is not an object", name);
        assert!(
            params.get("type").is_some(),
            "Tool {} parameters missing 'type'",
            name
        );
    }

    // Test that the filesystem tool can be created with allowed paths.
    let fs_tool = FileSystemTool::new(vec![std::env::current_dir()?]);
    let schema = fs_tool.schema();
    assert_eq!(schema.name, "filesystem");
    assert!(!schema.description.is_empty());

    // Test that WebTool can be constructed (may fail if no network).
    let web_result = WebTool::new();
    if let Ok(web_tool) = web_result {
        let schema = web_tool.schema();
        assert_eq!(schema.name, "web");
    }

    Ok(())
}

// ── test_counter_tool_execution ──────────────────────────────────────────────

#[test]
fn test_counter_tool_execution() -> Result<()> {
    let counter = Box::new(CounterTool::new());
    let agent = AgentManager::new(test_config())?;

    agent.register_tool(counter);

    // We can't execute via the agent directly (execute_tool_call is private),
    // but we can verify it's registered and listed.
    let names = agent.list_tool_names();
    assert!(names.contains(&"counter".to_string()));

    // Verify schema is produced.
    let descs = agent.get_tool_descriptions();
    let counter_desc = descs
        .iter()
        .find(|(name, _)| name == "counter")
        .expect("counter tool should be registered");
    assert!(!counter_desc.1.is_empty());

    Ok(())
}
