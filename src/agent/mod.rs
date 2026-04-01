pub mod subagent;
pub mod prompts;
pub mod skills;

use anyhow::Result;
use std::sync::Arc;
use crate::api::{client::AnthropicClient, types::*};
use crate::tools::{ToolRegistry, base::Tool};
use crate::utils::{SessionManager, ContextManager, PlanningMode};
use crate::config::Config;
use crate::ui::UI;
pub use subagent::{SubAgentManager, SubAgentTask, TaskStatus};
pub use prompts::PromptTemplate;
pub use skills::{SkillManager, Skill};

pub struct Agent {
    client: AnthropicClient,
    tools: Arc<ToolRegistry>,
    messages: Vec<Message>,
    max_iterations: usize,
    session_manager: SessionManager,
    context_manager: ContextManager,
    planning_mode: PlanningMode,
    subagent_manager: SubAgentManager,
    config: Config,
    ui: UI,
    in_plan_mode: bool,
    current_plan: Vec<String>,
    prompt_template: PromptTemplate,
    skill_manager: SkillManager,
}

impl Agent {
    pub fn new(api_key: String, base_url: Option<String>, config: Config) -> Result<Self> {
        let client = AnthropicClient::new(api_key.clone(), base_url.clone());
        let context_manager = ContextManager::new(AnthropicClient::new(api_key.clone(), base_url.clone()));
        let planning_mode = PlanningMode::new(AnthropicClient::new(api_key, base_url));
        let ui = UI::new(config.ui.colors, config.ui.show_progress);
        
        Ok(Self {
            client,
            tools: Arc::new(ToolRegistry::new()),
            messages: Vec::new(),
            max_iterations: config.agent.max_iterations,
            session_manager: SessionManager::new()?,
            context_manager,
            planning_mode,
            subagent_manager: SubAgentManager::new(),
            config,
            ui,
            in_plan_mode: false,
            current_plan: Vec::new(),
            prompt_template: PromptTemplate::new(),
            skill_manager: SkillManager::new()?,
        })
    }

    pub fn ui(&self) -> &UI {
        &self.ui
    }

    pub fn skill_manager(&mut self) -> &mut SkillManager {
        &mut self.skill_manager
    }

    pub fn skill_manager_ref(&self) -> &SkillManager {
        &self.skill_manager
    }

    pub async fn spawn_subagent(&self, task: String) -> String {
        let client = AnthropicClient::new(
            self.config.api.api_key.clone().unwrap_or_default(),
            self.config.api.base_url.clone(),
        );
        self.subagent_manager.spawn(client, self.tools.clone(), task).await
    }

    pub async fn run_subagent(&self, id: &str) -> Result<String> {
        self.subagent_manager.run_agent(id, self.max_iterations).await
    }

    pub async fn list_subagents(&self) -> Vec<SubAgentTask> {
        self.subagent_manager.list_agents().await
    }

    pub async fn run_subagents_parallel(&self) -> Vec<Result<String>> {
        self.subagent_manager.run_all_parallel(self.max_iterations).await
    }

    pub async fn enter_plan_mode(&mut self, task: &str) -> Result<()> {
        println!("\n🎯 Entering planning mode...");
        let steps = self.planning_mode.create_plan(task).await?;
        self.planning_mode.display_plan(&steps);
        self.current_plan = steps;
        self.in_plan_mode = true;
        Ok(())
    }

    pub fn exit_plan_mode(&mut self) {
        self.in_plan_mode = false;
        self.current_plan.clear();
        println!("✅ Exited planning mode");
    }

    pub fn is_in_plan_mode(&self) -> bool {
        self.in_plan_mode
    }

    pub fn get_current_plan(&self) -> &[String] {
        &self.current_plan
    }

    pub async fn init(&mut self) -> Result<()> {
        self.session_manager.init().await?;
        self.session_manager.new_session();
        self.skill_manager.init().await?;
        Ok(())
    }

    pub async fn load_session(&mut self, session_id: String) -> Result<()> {
        self.messages = self.session_manager.load_session(&session_id).await?;
        self.session_manager.set_session(session_id);
        println!("📂 Loaded {} messages from session", self.messages.len());
        Ok(())
    }

    pub async fn list_sessions(&self) -> Result<()> {
        let sessions = self.session_manager.list_sessions().await?;
        
        if sessions.is_empty() {
            println!("No saved sessions found.");
            return Ok(());
        }

        println!("\n📚 Saved Sessions:\n");
        for session in sessions {
            println!("  {} - {} messages", 
                session.id, 
                session.message_count
            );
            println!("    Created: {}", session.created_at.format("%Y-%m-%d %H:%M"));
            println!("    Updated: {}", session.updated_at.format("%Y-%m-%d %H:%M"));
            println!();
        }
        
        Ok(())
    }

    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(Message {
            role: "user".to_string(),
            content: vec![ContentBlock::Text { text: content }],
        });
    }

    pub async fn run(&mut self) -> Result<String> {
        let mut iteration = 0;

        // Check if context needs compaction
        if self.context_manager.needs_compaction(&self.messages) {
            println!("⚠️  Context approaching limit, compacting...");
            self.messages = self.context_manager.compact_messages(self.messages.clone()).await?;
        }

        loop {
            if iteration >= self.max_iterations {
                return Ok("Max iterations reached".to_string());
            }
            iteration += 1;

            println!("\n🔄 Iteration {}/{}", iteration, self.max_iterations);

            let request = CreateMessageRequest {
                model: "claude-sonnet-4-5-20250929".to_string(),
                max_tokens: 8192,
                messages: self.messages.clone(),
                tools: Some(self.tools.to_api_tools()),
                system: Some(self.get_system_prompt()),
            };

            let response = self.client.create_message(request).await?;

            println!("📊 Tokens: input={}, output={}", 
                response.usage.input_tokens, 
                response.usage.output_tokens
            );

            // Check if we need to execute tools
            let has_tool_use = response.content.iter().any(|block| {
                matches!(block, ContentBlock::ToolUse { .. })
            });

            if has_tool_use {
                // Execute tools
                let mut tool_results = Vec::new();

                for block in &response.content {
                    if let ContentBlock::ToolUse { id, name, input } = block {
                        println!("🔧 Executing tool: {}", name);
                        
                        if let Some(tool) = self.tools.get(name) {
                            // Check permission
                            if tool.requires_permission() {
                                println!("⚠️  Tool '{}' requires permission", name);
                                println!("   Input: {}", serde_json::to_string_pretty(input)?);
                                print!("   Allow? [y/N]: ");
                                use std::io::{self, Write};
                                io::stdout().flush()?;
                                
                                let mut response = String::new();
                                io::stdin().read_line(&mut response)?;
                                
                                if !response.trim().eq_ignore_ascii_case("y") {
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: id.clone(),
                                        content: "Permission denied by user".to_string(),
                                        is_error: Some(true),
                                    });
                                    continue;
                                }
                            }

                            match tool.execute(input.clone()).await {
                                Ok(result) => {
                                    println!("✅ Tool completed");
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: id.clone(),
                                        content: result,
                                        is_error: None,
                                    });
                                }
                                Err(e) => {
                                    println!("❌ Tool failed: {}", e);
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: id.clone(),
                                        content: format!("Error: {}", e),
                                        is_error: Some(true),
                                    });
                                }
                            }
                        } else {
                            tool_results.push(ContentBlock::ToolResult {
                                tool_use_id: id.clone(),
                                content: format!("Unknown tool: {}", name),
                                is_error: Some(true),
                            });
                        }
                    }
                }

                // Add assistant message and tool results
                let assistant_msg = Message {
                    role: "assistant".to_string(),
                    content: response.content,
                };
                let tool_msg = Message {
                    role: "user".to_string(),
                    content: tool_results,
                };
                
                self.messages.push(assistant_msg.clone());
                self.messages.push(tool_msg.clone());
                
                // Save to session
                self.session_manager.save_message(assistant_msg).await?;
                self.session_manager.save_message(tool_msg).await?;
            } else {
                // No more tools to execute, return final response
                let assistant_msg = Message {
                    role: "assistant".to_string(),
                    content: response.content.clone(),
                };
                self.messages.push(assistant_msg.clone());
                self.session_manager.save_message(assistant_msg).await?;
                
                for block in &response.content {
                    if let ContentBlock::Text { text } = block {
                        return Ok(text.clone());
                    }
                }
                return Ok("No response text".to_string());
            }
        }
    }

    fn get_system_prompt(&self) -> String {
        let mut prompt = if self.in_plan_mode {
            PromptTemplate::get_planning_prompt()
        } else {
            self.prompt_template.get_system_prompt().to_string()
        };

        // Append active skills
        let skills_prompt = self.skill_manager.get_active_skills_prompt();
        if !skills_prompt.is_empty() {
            prompt.push_str("\n\n");
            prompt.push_str(&skills_prompt);
        }

        prompt
    }
}
