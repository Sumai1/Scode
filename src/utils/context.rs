use anyhow::Result;
use crate::api::{client::AnthropicClient, types::*};

const TOKEN_THRESHOLD: usize = 150_000; // 当接近 200k 时触发压缩
const KEEP_RECENT_MESSAGES: usize = 10; // 保留最近的消息数量

pub struct ContextManager {
    client: AnthropicClient,
}

impl ContextManager {
    pub fn new(client: AnthropicClient) -> Self {
        Self { client }
    }

    pub fn estimate_tokens(&self, messages: &[Message]) -> usize {
        // 简单估算：每个字符约 0.25 tokens
        let mut total = 0;
        for msg in messages {
            for block in &msg.content {
                match block {
                    ContentBlock::Text { text } => {
                        total += text.len() / 4;
                    }
                    ContentBlock::ToolUse { input, .. } => {
                        if let Ok(s) = serde_json::to_string(input) {
                            total += s.len() / 4;
                        }
                    }
                    ContentBlock::ToolResult { content, .. } => {
                        total += content.len() / 4;
                    }
                }
            }
        }
        total
    }

    pub fn needs_compaction(&self, messages: &[Message]) -> bool {
        self.estimate_tokens(messages) > TOKEN_THRESHOLD
    }

    pub async fn compact_messages(&self, messages: Vec<Message>) -> Result<Vec<Message>> {
        if messages.len() <= KEEP_RECENT_MESSAGES {
            return Ok(messages);
        }

        let split_point = messages.len() - KEEP_RECENT_MESSAGES;
        let old_messages = &messages[..split_point];
        let recent_messages = &messages[split_point..];

        println!("🗜️  Compacting {} old messages...", old_messages.len());

        // 创建总结请求
        let summary_prompt = self.create_summary_prompt(old_messages);
        
        let request = CreateMessageRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 2048,
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![ContentBlock::Text { text: summary_prompt }],
            }],
            tools: None,
            system: Some("You are a helpful assistant that summarizes conversation history concisely.".to_string()),
        };

        let response = self.client.create_message(request).await?;

        let summary = response.content.iter()
            .find_map(|block| {
                if let ContentBlock::Text { text } = block {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Previous conversation summary unavailable.".to_string());

        println!("✅ Compaction complete");

        // 构建新的消息列表
        let mut compacted = vec![
            Message {
                role: "user".to_string(),
                content: vec![ContentBlock::Text {
                    text: format!("# Previous Conversation Summary\n\n{}", summary),
                }],
            },
            Message {
                role: "assistant".to_string(),
                content: vec![ContentBlock::Text {
                    text: "I understand. I'll continue from here with the context of our previous conversation.".to_string(),
                }],
            },
        ];

        compacted.extend_from_slice(recent_messages);

        Ok(compacted)
    }

    fn create_summary_prompt(&self, messages: &[Message]) -> String {
        let mut prompt = String::from("Please provide a concise summary of the following conversation, focusing on:\n");
        prompt.push_str("- Key decisions made\n");
        prompt.push_str("- Important information discussed\n");
        prompt.push_str("- Tasks completed\n");
        prompt.push_str("- Current context and state\n\n");
        prompt.push_str("Conversation:\n\n");

        for msg in messages {
            prompt.push_str(&format!("{}:\n", msg.role));
            for block in &msg.content {
                match block {
                    ContentBlock::Text { text } => {
                        prompt.push_str(text);
                        prompt.push('\n');
                    }
                    ContentBlock::ToolUse { name, .. } => {
                        prompt.push_str(&format!("[Used tool: {}]\n", name));
                    }
                    ContentBlock::ToolResult { .. } => {
                        prompt.push_str("[Tool result]\n");
                    }
                }
            }
            prompt.push('\n');
        }

        prompt
    }
}
