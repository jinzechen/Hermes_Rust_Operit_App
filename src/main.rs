use hermes_operit_core::core::agent::AgentManager;
use hermes_operit_core::core::config::AppConfig;
use hermes_operit_core::tools::filesystem::FileSystemTool;
use hermes_operit_core::tools::markdown::MarkdownTool;
use hermes_operit_core::ui::settings::SettingsManager;

// ── Banner ──────────────────────────────────────────────────────────────────

const BANNER: &str = r#"
╔══════════════════════════════════════════════════╗
║         Hermes Operit App v0.1.0                  ║
║         Pure Rust AI Assistant Toolchain          ║
║         (c) 2026 Operit / Nous Research           ║
╚══════════════════════════════════════════════════╝
"#;

// ── Entry Point ─────────────────────────────────────────────────────────────

fn main() {
    // ── Initialize logging ──────────────────────────────────────────────
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    println!("{}", BANNER);

    // ── Load config ─────────────────────────────────────────────────────
    let core_config = AppConfig::default();
    let settings_mgr = SettingsManager::load();

    log::info!(
        "Loaded config: model={}, endpoint={}",
        core_config.model,
        core_config.api_endpoint,
    );

    // ── Create AgentManager ─────────────────────────────────────────────
    let agent = match AgentManager::new(core_config) {
        Ok(a) => a,
        Err(e) => {
            log::error!("Failed to create AgentManager: {}", e);
            return;
        }
    };

    // ── Register built-in tools ─────────────────────────────────────────
    let fs_tool = FileSystemTool::new(vec![]);
    agent.register_tool(Box::new(fs_tool));

    let md_tool = MarkdownTool::new();
    agent.register_tool(Box::new(md_tool));

    log::info!("Registered {} tools", agent.tool_count());

    // ── CLI mode ────────────────────────────────────────────────────────
    log::info!("Entering CLI mode. Type 'help' for commands, 'quit' to exit.");

    let stdin = std::io::stdin();
    let mut buf = String::new();
    let session_id = format!("cli-{}", std::process::id());

    loop {
        buf.clear();
        print!("hermes> ");
        use std::io::Write;
        let _ = std::io::stdout().flush();

        match stdin.read_line(&mut buf) {
            Ok(0) => {
                log::info!("EOF received, shutting down.");
                break;
            }
            Ok(_) => {
                let input = buf.trim();
                match input {
                    "quit" | "exit" => {
                        log::info!("Shutting down.");
                        break;
                    }
                    "help" => {
                        println!("Available commands:");
                        println!("  help       Show this help");
                        println!("  status     Show agent status");
                        println!("  chat       Start chat session");
                        println!("  chat <msg> Send a message to the agent");
                        println!("  store      Browse plugin store (TODO)");
                        println!("  settings   Show current settings");
                        println!("  login      GitHub OAuth login (TODO)");
                        println!("  tools      List registered tools");
                        println!("  quit/exit  Exit");
                    }
                    "status" => {
                        println!("Agent: model={}", agent.config().model);
                        println!("Endpoint: {}", agent.config().api_endpoint);
                        println!("Session: {}", session_id);
                        println!("Tools registered: {}", agent.tool_count());
                    }
                    "settings" => {
                        println!("{}", settings_mgr.settings());
                    }
                    "tools" => {
                        println!("Registered tools:");
                        for name in agent.list_tool_names() {
                            println!("  - {}", name);
                        }
                    }
                    cmd if cmd.starts_with("chat ") => {
                        let msg = &cmd[5..];
                        println!("Sending: {}", msg);
                        match agent.send_message(&session_id, msg) {
                            Ok(response) => {
                                println!("\n{}", response.content);
                                if !response.tool_calls.is_empty() {
                                    println!("\n[{} tool call(s) made]", response.tool_calls.len());
                                }
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                    "chat" => {
                        println!("Entering chat mode. Type your message, or /quit to exit.");
                        loop {
                            buf.clear();
                            print!("you> ");
                            let _ = std::io::stdout().flush();
                            match stdin.read_line(&mut buf) {
                                Ok(0) | Err(_) => break,
                                Ok(_) => {
                                    let msg = buf.trim();
                                    if msg == "/quit" || msg == "/exit" {
                                        break;
                                    }
                                    if msg.is_empty() {
                                        continue;
                                    }
                                    match agent.send_message(&session_id, msg) {
                                        Ok(response) => {
                                            println!("ai> {}", response.content);
                                        }
                                        Err(e) => {
                                            println!("error> {}", e);
                                        }
                                    }
                                }
                            }
                        }
                        println!("Exited chat mode.");
                    }
                    "store" => {
                        println!("Store browser: not yet implemented.");
                    }
                    "login" => {
                        println!("GitHub OAuth: not yet implemented.");
                    }
                    "" => {}
                    other => {
                        println!("Unknown: '{}'. Type 'help' for commands.", other);
                    }
                }
            }
            Err(e) => {
                log::error!("stdin read error: {}", e);
                break;
            }
        }
    }
}
