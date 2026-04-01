use super::base::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;

pub struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read the complete contents of a file from the file system. Handles text files."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;

        let content = fs::read_to_string(path).await?;
        Ok(content)
    }
}

pub struct FileWriteTool;

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str {
        "file_write"
    }

    fn description(&self) -> &str {
        "Write content to a file. Creates the file if it doesn't exist, overwrites if it does."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;
        let content = input["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing content field"))?;

        // Create parent directories if needed
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, content).await?;
        Ok(format!("Successfully wrote {} bytes to {}", content.len(), path))
    }

    fn requires_permission(&self) -> bool {
        true
    }

    fn is_destructive(&self) -> bool {
        true
    }
}

pub struct FileEditTool;

#[async_trait]
impl Tool for FileEditTool {
    fn name(&self) -> &str {
        "file_edit"
    }

    fn description(&self) -> &str {
        "Edit a file by replacing exact text. Performs string replacement."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to edit"
                },
                "old_text": {
                    "type": "string",
                    "description": "Exact text to find and replace"
                },
                "new_text": {
                    "type": "string",
                    "description": "New text to replace with"
                }
            },
            "required": ["path", "old_text", "new_text"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;
        let old_text = input["old_text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing old_text field"))?;
        let new_text = input["new_text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing new_text field"))?;

        let content = fs::read_to_string(path).await?;
        
        if !content.contains(old_text) {
            anyhow::bail!("Old text not found in file");
        }

        let new_content = content.replace(old_text, new_text);
        fs::write(path, &new_content).await?;

        Ok(format!("Successfully edited {}", path))
    }

    fn requires_permission(&self) -> bool {
        true
    }

    fn is_destructive(&self) -> bool {
        true
    }
}
