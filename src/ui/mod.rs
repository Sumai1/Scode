use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct UI {
    colors_enabled: bool,
    show_progress: bool,
}

impl UI {
    pub fn new(colors_enabled: bool, show_progress: bool) -> Self {
        Self {
            colors_enabled,
            show_progress,
        }
    }

    pub fn print_header(&self, text: &str) {
        if self.colors_enabled {
            println!("{}", text.bright_cyan().bold());
        } else {
            println!("{}", text);
        }
    }

    pub fn print_success(&self, text: &str) {
        if self.colors_enabled {
            println!("{} {}", "✓".green().bold(), text.green());
        } else {
            println!("✓ {}", text);
        }
    }

    pub fn print_error(&self, text: &str) {
        if self.colors_enabled {
            eprintln!("{} {}", "✗".red().bold(), text.red());
        } else {
            eprintln!("✗ {}", text);
        }
    }

    pub fn print_warning(&self, text: &str) {
        if self.colors_enabled {
            println!("{} {}", "⚠".yellow().bold(), text.yellow());
        } else {
            println!("⚠ {}", text);
        }
    }

    pub fn print_info(&self, text: &str) {
        if self.colors_enabled {
            println!("{} {}", "ℹ".blue().bold(), text.blue());
        } else {
            println!("ℹ {}", text);
        }
    }

    pub fn print_tool(&self, name: &str) {
        if self.colors_enabled {
            println!("{} Executing tool: {}", "🔧".bright_white(), name.bright_yellow().bold());
        } else {
            println!("🔧 Executing tool: {}", name);
        }
    }

    pub fn print_iteration(&self, current: usize, max: usize) {
        if self.colors_enabled {
            println!("\n{} Iteration {}/{}", 
                "🔄".bright_white(), 
                current.to_string().bright_cyan().bold(), 
                max.to_string().bright_white()
            );
        } else {
            println!("\n🔄 Iteration {}/{}", current, max);
        }
    }

    pub fn print_tokens(&self, input: u32, output: u32) {
        if self.colors_enabled {
            println!("{} Tokens: input={}, output={}", 
                "📊".bright_white(),
                input.to_string().bright_green(),
                output.to_string().bright_green()
            );
        } else {
            println!("📊 Tokens: input={}, output={}", input, output);
        }
    }

    pub fn print_plan_header(&self) {
        if self.colors_enabled {
            println!("\n{}", "📋 Plan:".bright_magenta().bold());
            println!("{}", "━".repeat(50).bright_black());
        } else {
            println!("\n📋 Plan:");
            println!("{}", "━".repeat(50));
        }
    }

    pub fn print_plan_step(&self, index: usize, step: &str) {
        if self.colors_enabled {
            println!("{}. {}", 
                index.to_string().bright_cyan().bold(), 
                step.bright_white()
            );
        } else {
            println!("{}. {}", index, step);
        }
    }

    pub fn print_plan_footer(&self) {
        if self.colors_enabled {
            println!("{}\n", "━".repeat(50).bright_black());
        } else {
            println!("{}\n", "━".repeat(50));
        }
    }

    pub fn create_progress_bar(&self, len: u64, message: &str) -> Option<ProgressBar> {
        if !self.show_progress {
            return None;
        }

        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        pb.set_message(message.to_string());
        Some(pb)
    }

    pub fn create_spinner(&self, message: &str) -> Option<ProgressBar> {
        if !self.show_progress {
            return None;
        }

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    }

    pub fn print_agent_response(&self, text: &str) {
        if self.colors_enabled {
            println!("\n{} {}:\n{}\n", 
                "🤖".bright_white(), 
                "Scode".bright_cyan().bold(),
                text.bright_white()
            );
        } else {
            println!("\n🤖 Scode:\n{}\n", text);
        }
    }

    pub fn print_user_prompt(&self) {
        if self.colors_enabled {
            print!("{}: ", "You".bright_green().bold());
        } else {
            print!("You: ");
        }
    }

    pub fn print_session_info(&self, id: &str, count: usize, created: &str, updated: &str) {
        if self.colors_enabled {
            println!("  {} - {} messages", 
                id.bright_yellow(), 
                count.to_string().bright_cyan()
            );
            println!("    Created: {}", created.bright_black());
            println!("    Updated: {}", updated.bright_black());
        } else {
            println!("  {} - {} messages", id, count);
            println!("    Created: {}", created);
            println!("    Updated: {}", updated);
        }
    }

    pub fn print_config_item(&self, key: &str, value: &str) {
        if self.colors_enabled {
            println!("   {}: {}", 
                key.bright_yellow(), 
                value.bright_white()
            );
        } else {
            println!("   {}: {}", key, value);
        }
    }
}
