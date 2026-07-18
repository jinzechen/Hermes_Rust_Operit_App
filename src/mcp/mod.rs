// MCP (Model Context Protocol) module — JSON-RPC 2.0 over stdio.
// Provides both a custom lightweight client and rmcp-based manager.
//
// Custom client: mcp/client.rs (466 lines, 4 tests, JSON-RPC 2.0)
// rmcp manager: mcp/manager.rs (uses rmcp crate for external MCP servers)

pub mod client;
pub mod manager;

pub use client::McpClient;
pub use manager::McpManager;
