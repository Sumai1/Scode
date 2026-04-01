use anyhow::Result;
use crate::api::{client::AnthropicClient, types::*};

pub struct PlanningMode {
    client: AnthropicClient,
}

impl PlanningMode {
    pub fn new(client: AnthropicClient) -> Self {
        Self { client }
    }

    pub async fn create_plan(&self, task: &str) -> Result<Vec<String>> {
        let prompt = format!(
            r#"Break down the following task into clear, actionable steps.
Each step should be specific and executable.
Return ONLY a numbered list of steps, nothing else.

Task: {}

Steps:"#,
            task
        );

        let request = CreateMessageRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 2048,
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![ContentBlock::Text { text: prompt }],
            }],
            tools: None,
            system: Some("You are a helpful assistant that breaks down tasks into clear steps.".to_string()),
        };

        let response = self.client.create_message(request).await?;

        let plan_text = response
            .content
            .iter()
            .find_map(|block| {
                if let ContentBlock::Text { text } = block {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        // Parse the plan into steps
        let steps: Vec<String> = plan_text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                // Remove numbering like "1. ", "2. ", etc.
                let trimmed = line.trim();
                if let Some(pos) = trimmed.find(|c: char| c == '.' || c == ')') {
                    if trimmed[..pos].chars().all(|c| c.is_numeric()) {
                        return trimmed[pos + 1..].trim().to_string();
                    }
                }
                trimmed.to_string()
            })
            .collect();

        Ok(steps)
    }

    pub fn display_plan(&self, steps: &[String]) {
        println!("\n📋 Plan:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        for (i, step) in steps.iter().enumerate() {
            println!("{}. {}", i + 1, step);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}
