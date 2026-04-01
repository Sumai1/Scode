mod api;
mod agent;
mod tools;
mod utils;
mod config;
mod ui;

use anyhow::Result;
use clap::Parser;
use config::Config;

#[derive(Parser)]
#[command(name = "scode")]
#[command(about = "Scode - AI Code Agent", long_about = None)]
#[command(version = "0.6.1")]
struct Cli {
    /// Prompt to send to the agent
    #[arg(short, long)]
    prompt: Option<String>,

    /// API key (or set SCODE_API_KEY env var)
    #[arg(long, env = "SCODE_API_KEY")]
    api_key: Option<String>,

    /// Base URL (or set SCODE_BASE_URL env var)
    #[arg(long, env = "SCODE_BASE_URL")]
    base_url: Option<String>,

    /// Model to use
    #[arg(short, long)]
    model: Option<String>,

    /// Resume a previous session
    #[arg(short, long)]
    resume: Option<String>,

    /// List all saved sessions
    #[arg(long)]
    list_sessions: bool,

    /// Initialize default config file
    #[arg(long)]
    init_config: bool,

    /// Disable colors
    #[arg(long)]
    no_color: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize config if requested
    if cli.init_config {
        Config::init_default().await?;
        return Ok(());
    }

    // Load config
    let mut config = Config::load().await?;

    // Override config with CLI args
    if let Some(key) = cli.api_key {
        config.api.api_key = Some(key);
    }
    if let Some(url) = cli.base_url {
        config.api.base_url = Some(url);
    }
    if let Some(model) = cli.model {
        config.api.model = Some(model);
    }
    if cli.no_color {
        config.ui.colors = false;
    }

    // Get API key
    let api_key = config.api.api_key.clone()
        .ok_or_else(|| anyhow::anyhow!("API key not provided. Set ANTHROPIC_API_KEY or use --api-key"))?;

    let mut agent = agent::Agent::new(api_key, config.api.base_url.clone(), config.clone())?;
    
    // Create UI separately to avoid borrow issues
    let ui = ui::UI::new(config.ui.colors, config.ui.show_progress);

    ui.print_header("🚀 Scode - AI Code Agent v0.5.0");
    ui.print_header("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    agent.init().await?;

    // List sessions if requested
    if cli.list_sessions {
        agent.list_sessions().await?;
        return Ok(());
    }

    // Resume session if requested
    if let Some(session_id) = cli.resume {
        agent.load_session(session_id).await?;
    }

    if let Some(prompt) = cli.prompt {
        // Single prompt mode
        agent.add_user_message(prompt);
        let response = agent.run().await?;
        ui.print_agent_response(&response);
    } else {
        // Interactive mode
        ui.print_info("Interactive mode - Commands:");
        println!("   /help      - Show available commands");
        println!("   /plan      - Enter planning mode");
        println!("   /spawn     - Spawn sub-agent");
        println!("   /agents    - List sub-agents");
        println!("   /skills    - List available skills");
        println!("   /load      - Load a skill");
        println!("   /unload    - Unload a skill");
        println!("   /install   - Install a skill from URL/GitHub");
        println!("   /remove    - Remove an installed skill");
        println!("   /mode      - Show/change work mode");
        println!("   /clear     - Clear conversation");
        println!("   /sessions  - List saved sessions");
        println!("   /config    - Show configuration");
        println!("   /exit      - Exit (saves session)\n");
        
        loop {
            ui.print_user_prompt();
            use std::io::{self, Write};
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            // Handle commands
            if input.starts_with('/') {
                match input.split_whitespace().next().unwrap() {
                    "/exit" | "/quit" => {
                        ui.print_success("Session saved. Goodbye!");
                        break;
                    }
                    "/clear" => {
                        ui.print_info("Conversation cleared (session preserved)");
                        continue;
                    }
                    "/plan" => {
                        print!("Enter task to plan: ");
                        io::stdout().flush()?;
                        let mut task = String::new();
                        io::stdin().read_line(&mut task)?;
                        let task = task.trim();
                        
                        if !task.is_empty() {
                            agent.enter_plan_mode(task).await?;
                            ui.print_info("Now execute the plan by chatting normally, or type /exit-plan to cancel");
                        }
                        continue;
                    }
                    "/exit-plan" => {
                        agent.exit_plan_mode();
                        continue;
                    }
                    "/spawn" => {
                        print!("Enter sub-agent task: ");
                        io::stdout().flush()?;
                        let mut task = String::new();
                        io::stdin().read_line(&mut task)?;
                        let task = task.trim();
                        
                        if !task.is_empty() {
                            let id = agent.spawn_subagent(task.to_string()).await;
                            ui.print_success(&format!("Spawned sub-agent: {}", id));
                            
                            // Run the sub-agent
                            ui.print_info("Running sub-agent...");
                            match agent.run_subagent(&id).await {
                                Ok(result) => {
                                    ui.print_success("Sub-agent completed");
                                    println!("Result: {}", result);
                                }
                                Err(e) => {
                                    ui.print_error(&format!("Sub-agent failed: {}", e));
                                }
                            }
                        }
                        continue;
                    }
                    "/agents" => {
                        let agents = agent.list_subagents().await;
                        if agents.is_empty() {
                            ui.print_info("No sub-agents");
                        } else {
                            ui.print_info(&format!("Sub-agents ({}):", agents.len()));
                            for task in agents {
                                println!("  {} - {:?}: {}", 
                                    task.id, 
                                    task.status,
                                    task.description
                                );
                            }
                        }
                        continue;
                    }
                    "/help" => {
                        ui.print_info("Available Commands:");
                        println!("   /help       - Show this help message");
                        println!("   /plan       - Create a step-by-step plan");
                        println!("   /exit-plan  - Exit planning mode");
                        println!("   /spawn      - Spawn a sub-agent for a task");
                        println!("   /agents     - List all sub-agents");
                        println!("   /skills     - List available skills");
                        println!("   /load       - Load a skill");
                        println!("   /unload     - Unload a skill");
                        println!("   /install    - Install a skill from URL/GitHub");
                        println!("   /remove     - Remove an installed skill");
                        println!("   /mode       - Show current work mode");
                        println!("   /clear      - Clear current conversation");
                        println!("   /sessions   - List all saved sessions");
                        println!("   /config     - Show current configuration");
                        println!("   /exit       - Exit and save session");
                        println!("\n🎯 Work Modes:");
                        println!("   • Normal    - General coding assistance");
                        println!("   • Planning  - Create detailed plans (/plan)");
                        println!("   • Sub-agent - Parallel task execution (/spawn)");
                        println!("\n🎓 Skills:");
                        println!("   • Load domain-specific knowledge with /load");
                        println!("   • Available: Rust, Web Development, Git Workflow");
                        println!("\n🛠️  Available Tools (22 total):");
                        println!("   • File: file_read, file_write, file_edit, file_list, file_move,");
                        println!("           file_copy, file_delete, file_info");
                        println!("   • Search: glob, grep");
                        println!("   • Git: git_status, git_diff, git_add, git_commit, git_log,");
                        println!("          git_branch, git_pull, git_push");
                        println!("   • System: bash");
                        println!("   • Web: web_fetch, web_search, http_request");
                        println!();
                        continue;
                    }
                    "/skills" => {
                        let skills = agent.skill_manager_ref().list_skills();
                        let active_skills = agent.skill_manager_ref().get_active_skills().to_vec();
                        
                        if skills.is_empty() {
                            ui.print_info("No skills available");
                        } else {
                            ui.print_info(&format!("Available Skills ({}):", skills.len()));
                            for skill in skills {
                                let active = if active_skills.contains(&skill.name) {
                                    "✓"
                                } else {
                                    " "
                                };
                                println!("  [{}] {} - {}", active, skill.name, skill.description);
                                println!("      Category: {} | Tags: {}", skill.category, skill.tags.join(", "));
                            }
                            println!("\nActive skills: {}", active_skills.len());
                            println!("Use /load <skill-name> to activate a skill");
                        }
                        continue;
                    }
                    "/load" => {
                        print!("Enter skill name to load: ");
                        io::stdout().flush()?;
                        let mut skill_name = String::new();
                        io::stdin().read_line(&mut skill_name)?;
                        let skill_name = skill_name.trim();
                        
                        if !skill_name.is_empty() {
                            match agent.skill_manager().activate_skill(skill_name) {
                                Ok(_) => {
                                    ui.print_success(&format!("Loaded skill: {}", skill_name));
                                    if let Some(skill) = agent.skill_manager().get_skill(skill_name) {
                                        println!("   {}", skill.description);
                                    }
                                }
                                Err(e) => {
                                    ui.print_error(&format!("Failed to load skill: {}", e));
                                }
                            }
                        }
                        continue;
                    }
                    "/unload" => {
                        let active = agent.skill_manager().get_active_skills();
                        if active.is_empty() {
                            ui.print_info("No active skills");
                        } else {
                            ui.print_info("Active skills:");
                            for (i, skill_name) in active.iter().enumerate() {
                                println!("   {}. {}", i + 1, skill_name);
                            }
                            print!("Enter skill name to unload (or 'all'): ");
                            io::stdout().flush()?;
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)?;
                            let input = input.trim();
                            
                            if input == "all" {
                                agent.skill_manager().clear_active_skills();
                                ui.print_success("Unloaded all skills");
                            } else if !input.is_empty() {
                                agent.skill_manager().deactivate_skill(input);
                                ui.print_success(&format!("Unloaded skill: {}", input));
                            }
                        }
                        continue;
                    }
                    "/install" => {
                        ui.print_info("Install Skill");
                        println!("   Options:");
                        println!("   1. From URL: /install https://example.com/skill.md");
                        println!("   2. From GitHub: /install username/repo");
                        println!("   3. From GitHub path: /install username/repo/path/to/skill.md");
                        print!("\nEnter URL or GitHub repo: ");
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let input = input.trim();
                        
                        if !input.is_empty() {
                            ui.print_info("Installing skill...");
                            
                            let result = if input.starts_with("http://") || input.starts_with("https://") {
                                agent.skill_manager().install_skill_from_url(input).await
                            } else {
                                agent.skill_manager().install_skill_from_github(input).await
                            };
                            
                            match result {
                                Ok(name) => {
                                    ui.print_success(&format!("Installed skill: {}", name));
                                    println!("   Use /load {} to activate it", name);
                                }
                                Err(e) => {
                                    ui.print_error(&format!("Failed to install skill: {}", e));
                                }
                            }
                        }
                        continue;
                    }
                    "/remove" => {
                        let skills = agent.skill_manager_ref().list_skills();
                        if skills.is_empty() {
                            ui.print_info("No skills installed");
                        } else {
                            ui.print_info("Installed skills:");
                            for (i, skill) in skills.iter().enumerate() {
                                println!("   {}. {} - {}", i + 1, skill.name, skill.description);
                            }
                            print!("Enter skill name to remove: ");
                            io::stdout().flush()?;
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)?;
                            let input = input.trim();
                            
                            if !input.is_empty() {
                                match agent.skill_manager().uninstall_skill(input).await {
                                    Ok(_) => {
                                        ui.print_success(&format!("Removed skill: {}", input));
                                    }
                                    Err(e) => {
                                        ui.print_error(&format!("Failed to remove skill: {}", e));
                                    }
                                }
                            }
                        }
                        continue;
                    }
                    "/mode" => {
                        if agent.is_in_plan_mode() {
                            ui.print_info("Current mode: Planning");
                            println!("   Use /exit-plan to return to normal mode");
                        } else {
                            ui.print_info("Current mode: Normal");
                            println!("   Use /plan to enter planning mode");
                            println!("   Use /spawn to create sub-agents");
                        }
                        continue;
                    }
                    "/sessions" => {
                        agent.list_sessions().await?;
                        continue;
                    }
                    "/config" => {
                        ui.print_info("Current Configuration:");
                        ui.print_config_item("Model", config.api.model.as_deref().unwrap_or("default"));
                        ui.print_config_item("Max iterations", &config.agent.max_iterations.to_string());
                        ui.print_config_item("Auto-approve read-only", &config.agent.auto_approve_read_only.to_string());
                        ui.print_config_item("Colors", &config.ui.colors.to_string());
                        ui.print_config_item("Show tokens", &config.ui.show_tokens.to_string());
                        ui.print_config_item("Config file", &Config::config_path()?.display().to_string());
                        println!();
                        continue;
                    }
                    _ => {
                        ui.print_error("Unknown command. Type /help for available commands.");
                        continue;
                    }
                }
            }

            // Show plan if in planning mode
            if agent.is_in_plan_mode() {
                let plan = agent.get_current_plan();
                if !plan.is_empty() {
                    ui.print_plan_header();
                    for (i, step) in plan.iter().enumerate() {
                        ui.print_plan_step(i + 1, step);
                    }
                    ui.print_plan_footer();
                }
            }

            agent.add_user_message(input.to_string());
            
            match agent.run().await {
                Ok(response) => {
                    ui.print_agent_response(&response);
                }
                Err(e) => {
                    ui.print_error(&format!("Error: {}", e));
                }
            }
        }
    }

    Ok(())
}
