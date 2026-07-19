use hermes_operit_core::core::agent::AgentManager;
use hermes_operit_core::core::config::AppConfig;
use hermes_operit_core::core::provider::SmartModelRouter;
use std::io::{self, Write};

const BANNER: &str = r#"
╔══════════════════════════════════════════════════╗
║         Hermes Operit App v0.1.0                  ║
║         Pure Rust AI Assistant Toolchain          ║
║         (c) 2026 Operit / Nous Research           ║
╚══════════════════════════════════════════════════╝
"#;

fn print_tools(agent: &AgentManager) {
    let tools = agent.get_tool_descriptions();
    for (name, desc) in &tools {
        let d = if desc.len() > 50 {
            format!("{}…", &desc[..49])
        } else {
            desc.clone()
        };
        println!("  {} — {}", name, d);
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    println!("{}", BANNER);

    // Load config from ~/.hermes/config.yaml, fallback to default
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    let config =
        AppConfig::load_from_file(std::path::PathBuf::from(&home).join(".hermes/config.yaml"))
            .unwrap_or_else(|e| {
                log::warn!("Config: {} — using defaults.", e);
                AppConfig::default()
            });
    log::info!("model={} endpoint={}", config.model, config.api_endpoint);

    let mut agent = match AgentManager::with_default_tools(config) {
        Ok(a) => a,
        Err(e) => {
            log::error!("AgentManager init failed: {}", e);
            return;
        }
    };

    // Startup display
    let _router = SmartModelRouter::new();
    println!("\nSmartModelRouter:");
    println!("  Compile→T=0.1/tok=2048  CodeReview→T=0.2/tok=4096  Doc→T=0.3/tok=8192");
    println!("  Chat→T=0.7/tok=def      Research→T=0.4/tok=16384");
    println!("\nTools ({}):", agent.tool_count());
    print_tools(&agent);
    println!("\n{}", agent.get_memory_summary());
    let h = agent.health_check();
    println!(
        "\nHealth: provider={} model={} sessions={} uptime={:.0}s",
        if h.provider_ok { "OK" } else { "DOWN" },
        h.model,
        h.session_count,
        h.uptime_secs
    );

    log::info!("CLI ready. Type 'help'.");
    let stdin = io::stdin();
    let mut buf = String::new();
    let sid = format!("cli-{}", std::process::id());

    loop {
        buf.clear();
        print!("\nhermes> ");
        let _ = io::stdout().flush();
        match stdin.read_line(&mut buf) {
            Ok(0) => {
                log::info!("EOF.");
                break;
            }
            Ok(_) => {
                match buf.trim() {
                    "quit" | "exit" => break,
                    "help" => {
                        println!("Commands: help status tools chat memory sessions clear config health quit");
                        let tools = agent.get_tool_descriptions();
                        println!("Tools ({}):", tools.len());
                        for (n, d) in &tools {
                            println!("  {} — {}", n, d);
                        }
                    }
                    "status" => {
                        let h = agent.health_check();
                        println!(
                            "model={} endpoint={} tools={} sessions={} uptime={:.1}s provider={}",
                            h.model,
                            h.endpoint,
                            h.tool_count,
                            h.session_count,
                            h.uptime_secs,
                            if h.provider_ok { "OK" } else { "DOWN" }
                        );
                    }
                    "tools" => {
                        println!("Tools ({}):", agent.tool_count());
                        print_tools(&agent);
                    }
                    "memory" => println!("{}", agent.get_memory_summary()),
                    "sessions" => {
                        let ss = agent.list_sessions();
                        if ss.is_empty() {
                            println!("No active sessions.");
                        } else {
                            for s in &ss {
                                println!("  {}", s);
                            }
                        }
                    }
                    "health" => {
                        let h = agent.health_check();
                        println!(
                            "provider={} model={} tools={} sessions={} uptime={:.0}s",
                            if h.provider_ok { "OK" } else { "DOWN" },
                            h.model,
                            h.tool_count,
                            h.session_count,
                            h.uptime_secs
                        );
                    }
                    cmd if cmd.starts_with("clear ") => {
                        agent.clear_session(&cmd[6..].trim());
                        println!("Cleared.");
                    }
                    cmd if cmd.starts_with("config ") => {
                        if let Some((k, v)) = cmd[7..].split_once(' ') {
                            match agent.set_config(k, v) {
                                Ok(()) => println!("{} → {}", k, v),
                                Err(e) => println!("Error: {}", e),
                            }
                        } else {
                            println!("Usage: config <key> <value>");
                        }
                    }
                    cmd if cmd.starts_with("chat ") => match agent.send_message(&sid, &cmd[5..]) {
                        Ok(r) => {
                            println!("\n{}", r.content);
                            if let Some(ref u) = r.token_usage {
                                println!(
                                    "[in={} out={} total={}]",
                                    u.prompt_tokens, u.completion_tokens, u.total_tokens
                                );
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    },
                    "chat" => {
                        println!("Chat mode. /help /tools /clear /memory /quit");
                        loop {
                            buf.clear();
                            print!("you> ");
                            let _ = io::stdout().flush();
                            match stdin.read_line(&mut buf) {
                                Ok(0) | Err(_) => break,
                                Ok(_) => match buf.trim() {
                                    "/quit" | "/exit" => break,
                                    "/help" => println!("  /help /tools /clear /memory /quit"),
                                    "/tools" => print_tools(&agent),
                                    "/clear" => {
                                        agent.clear_session(&sid);
                                        println!("Cleared.");
                                    }
                                    "/memory" => println!("{}", agent.get_memory_summary()),
                                    "" => continue,
                                    msg => match agent.send_message(&sid, msg) {
                                        Ok(r) => {
                                            println!("ai> {}", r.content);
                                            if let Some(ref u) = r.token_usage {
                                                println!(
                                                    "     [in={} out={} total={}]",
                                                    u.prompt_tokens,
                                                    u.completion_tokens,
                                                    u.total_tokens
                                                );
                                            }
                                        }
                                        Err(e) => println!("error> {}", e),
                                    },
                                },
                            }
                        }
                        println!("Exited chat mode.");
                    }
                    "" => {}
                    other => println!("Unknown: '{}'. Type 'help'.", other),
                }
            }
            Err(e) => {
                log::error!("stdin: {}", e);
                break;
            }
        }
    }
}
