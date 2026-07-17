use hermes_operit_core::core::config::AppConfig;
use hermes_operit_core::ui::settings::{AppSettings, SettingsManager};

// ── Agent Manager (stub — wired up when agent module is complete) ───────────

/// Manages AI agent instances and their lifecycle.
struct AgentManager {
    config: AppConfig,
    settings: AppSettings,
}

impl AgentManager {
    fn new(config: AppConfig, settings: AppSettings) -> Self {
        Self { config, settings }
    }

    /// Register built-in tools (chat, store, settings, browser, filesystem, vision).
    fn register_builtin_tools(&self) {
        log::info!(
            "Registered built-in tools: chat, store_browser, settings_manager, browser, filesystem, vision"
        );
    }

    /// Print a summary of the current agent configuration.
    fn print_status(&self) {
        log::info!(
            "AgentManager: model={}, endpoint={}, theme={}, lang={}",
            self.config.model,
            self.config.api_endpoint,
            self.settings.theme,
            self.settings.language,
        );
    }
}

// ── MCP Server Manager (stub) ───────────────────────────────────────────────

struct McpServerManager;

impl McpServerManager {
    fn start() -> Self {
        log::info!("MCP server manager started (stdio transport)");
        Self
    }
}

// ── Banner ──────────────────────────────────────────────────────────────────

const BANNER: &str = r#"
╔══════════════════════════════════════════════════════╗
║          Hermes Operit App v0.1.0                     ║
║          Pure Rust AI Assistant Toolchain             ║
║          (c) 2026 Operit / Nous Research              ║
╚══════════════════════════════════════════════════════╝
"#;

// ── Entry Point ─────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // ── Initialize logging ──────────────────────────────────────────────
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    println!("{}", BANNER);

    // ── Load config from file or env ────────────────────────────────────
    let core_config = AppConfig::default(); // TODO: load from file via AppConfig::load_from_file()
    let settings_mgr = SettingsManager::load();
    let app_settings = settings_mgr.settings().clone();

    log::info!(
        "Loaded config: model={}, endpoint={}, theme={}, lang={}",
        core_config.model,
        core_config.api_endpoint,
        app_settings.theme,
        app_settings.language,
    );

    // ── Create AgentManager ─────────────────────────────────────────────
    let agent = AgentManager::new(core_config, app_settings);
    agent.register_builtin_tools();
    agent.print_status();

    // ── Start MCP server manager ────────────────────────────────────────
    let _mcp = McpServerManager::start();

    // ── CLI mode: accept commands via stdin ─────────────────────────────
    // ── (Dioxus app is commented out — Android-only) ────────────────────
    // #[cfg(feature = "dioxus")]
    // {
    //     dioxus_mobile::launch(app);
    //     return;
    // }

    log::info!("Entering CLI mode. Type 'help' for commands, 'quit' to exit.");

    let stdin = std::io::stdin();
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("hermes> ");
        use std::io::Write;
        let _ = std::io::stdout().flush();

        match stdin.read_line(&mut buf) {
            Ok(0) => {
                // EOF
                log::info!("EOF received, shutting down.");
                break;
            }
            Ok(_) => {
                let cmd = buf.trim();
                match cmd {
                    "quit" | "exit" => {
                        log::info!("Shutting down.");
                        break;
                    }
                    "help" => {
                        println!("Available commands:");
                        println!("  help       Show this help");
                        println!("  status     Show agent status");
                        println!("  chat       Enter chat mode (stub)");
                        println!("  store      Browse plugin store (stub)");
                        println!("  settings   Show current settings");
                        println!("  login      GitHub OAuth login (stub)");
                        println!("  quit/exit  Exit the application");
                    }
                    "status" => {
                        agent.print_status();
                    }
                    "settings" => {
                        println!("{}", settings_mgr.settings());
                    }
                    "chat" => {
                        println!("Chat mode not yet implemented (stub).");
                    }
                    "store" => {
                        println!("Store browser not yet implemented (stub).");
                    }
                    "login" => {
                        println!("GitHub OAuth login not yet implemented (stub).");
                    }
                    "" => {
                        // Ignore empty lines.
                    }
                    other => {
                        println!("Unknown command: '{}'. Type 'help' for available commands.", other);
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
