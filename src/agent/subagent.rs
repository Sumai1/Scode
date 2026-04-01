use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::api::{client::AnthropicClient, types::*};
use crate::tools::ToolRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentTask {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct SubAgent {
    id: String,
    client: AnthropicClient,
    tools: Arc<ToolRegistry>,
    messages: Vec<Message>,
    task: SubAgentTask,
}

impl SubAgent {
    pub fn new(
        client: AnthropicClient,
        tools: Arc<ToolRegistry>,
        task_description: String,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let task = SubAgentTask {
            id: id.clone(),
            description: task_description.clone(),
            status: TaskStatus::Pending,
            result: None,
        };

        Self {
            id,
            client,
            tools,
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![ContentBlock::Text {
                    text: task_description,
                }],
            }],
            task,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn task(&self) -> &SubAgentTask {
        &self.task
    }

    pub async fn run(&mut self, max_iterations: usize) -> Result<String> {
        self.task.status = TaskStatus::Running;

        let mut iteration = 0;
        loop {
            if iteration >= max_iterations {
                self.task.status = TaskStatus::Failed;
                return Ok("Max iterations reached".to_string());
            }
            iteration += 1;

            let request = CreateMessageRequest {
                model: "claude-sonnet-4-5-20250929".to_string(),
                max_tokens: 4096,
                messages: self.messages.clone(),
                tools: Some(self.tools.to_api_tools()),
                system: Some(self.get_system_prompt()),
            };

            let response = self.client.create_message(request).await?;

            let has_tool_use = response
                .content
                .iter()
                .any(|block| matches!(block, ContentBlock::ToolUse { .. }));

            if has_tool_use {
                let mut tool_results = Vec::new();

                for block in &response.content {
                    if let ContentBlock::ToolUse { id, name, input } = block {
                        if let Some(tool) = self.tools.get(name) {
                            match tool.execute(input.clone()).await {
                                Ok(result) => {
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: id.clone(),
                                        content: result,
                                        is_error: None,
                                    });
                                }
                                Err(e) => {
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: id.clone(),
                                        content: format!("Error: {}", e),
                                        is_error: Some(true),
                                    });
                                }
                            }
                        }
                    }
                }

                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: response.content,
                });
                self.messages.push(Message {
                    role: "user".to_string(),
                    content: tool_results,
                });
            } else {
                for block in &response.content {
                    if let ContentBlock::Text { text } = block {
                        self.task.status = TaskStatus::Completed;
                        self.task.result = Some(text.clone());
                        return Ok(text.clone());
                    }
                }
                self.task.status = TaskStatus::Failed;
                return Ok("No response text".to_string());
            }
        }
    }

    fn get_system_prompt(&self) -> String {
        crate::agent::prompts::PromptTemplate::get_subagent_prompt(&self.task.description)
    }
}

pub struct SubAgentManager {
    agents: Arc<Mutex<Vec<Arc<Mutex<SubAgent>>>>>,
}

impl SubAgentManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn spawn(
        &self,
        client: AnthropicClient,
        tools: Arc<ToolRegistry>,
        task: String,
    ) -> String {
        let agent = Arc::new(Mutex::new(SubAgent::new(client, tools, task)));
        let id = agent.lock().await.id().to_string();
        self.agents.lock().await.push(agent);
        id
    }

    pub async fn run_agent(&self, id: &str, max_iterations: usize) -> Result<String> {
        let agents = self.agents.lock().await;
        for agent in agents.iter() {
            let mut agent_lock = agent.lock().await;
            if agent_lock.id() == id {
                return agent_lock.run(max_iterations).await;
            }
        }
        anyhow::bail!("Agent not found: {}", id)
    }

    pub async fn get_status(&self, id: &str) -> Option<SubAgentTask> {
        let agents = self.agents.lock().await;
        for agent in agents.iter() {
            let agent_lock = agent.lock().await;
            if agent_lock.id() == id {
                return Some(agent_lock.task().clone());
            }
        }
        None
    }

    pub async fn list_agents(&self) -> Vec<SubAgentTask> {
        let agents = self.agents.lock().await;
        let mut tasks = Vec::new();
        for agent in agents.iter() {
            let agent_lock = agent.lock().await;
            tasks.push(agent_lock.task().clone());
        }
        tasks
    }

    pub async fn run_all_parallel(&self, max_iterations: usize) -> Vec<Result<String>> {
        let agents = self.agents.lock().await.clone();
        let mut handles = Vec::new();

        for agent in agents {
            let agent_clone = agent.clone();
            let handle = tokio::spawn(async move {
                agent_clone.lock().await.run(max_iterations).await
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(anyhow::anyhow!("Task failed: {}", e))),
            }
        }

        results
    }
}
