use super::base::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;

pub struct FileListTool;

#[async_trait]
impl Tool for FileListTool {
    fn name(&self) -> &str {
        "file_list"
    }

    fn description(&self) -> &str {
        "List files and directories in a path with details (size, modified time)."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Directory path to list"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "List recursively"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;
        let recursive = input["recursive"].as_bool().unwrap_or(false);

        let mut result = String::new();
        
        if recursive {
            let walker = walkdir::WalkDir::new(path);
            for entry in walker {
                let entry = entry?;
                let metadata = entry.metadata()?;
                result.push_str(&format!(
                    "{} {} bytes\n",
                    entry.path().display(),
                    metadata.len()
                ));
            }
        } else {
            let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let metadata = entry.metadata().await?;
                let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };
                result.push_str(&format!(
                    "{} {} {} bytes\n",
                    file_type,
                    entry.file_name().to_string_lossy(),
                    metadata.len()
                ));
            }
        }

        Ok(result)
    }
}

pub struct FileMoveTool;

#[async_trait]
impl Tool for FileMoveTool {
    fn name(&self) -> &str {
        "file_move"
    }

    fn description(&self) -> &str {
        "Move or rename a file or directory."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "from": {
                    "type": "string",
                    "description": "Source path"
                },
                "to": {
                    "type": "string",
                    "description": "Destination path"
                }
            },
            "required": ["from", "to"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let from = input["from"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing from field"))?;
        let to = input["to"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing to field"))?;

        fs::rename(from, to).await?;
        Ok(format!("Moved {} to {}", from, to))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct FileDeleteTool;

#[async_trait]
impl Tool for FileDeleteTool {
    fn name(&self) -> &str {
        "file_delete"
    }

    fn description(&self) -> &str {
        "Delete a file or directory. Use with caution!"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to delete"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "Delete directory recursively"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;
        let recursive = input["recursive"].as_bool().unwrap_or(false);

        let path_obj = Path::new(path);
        
        if path_obj.is_dir() {
            if recursive {
                fs::remove_dir_all(path).await?;
            } else {
                fs::remove_dir(path).await?;
            }
        } else {
            fs::remove_file(path).await?;
        }

        Ok(format!("Deleted {}", path))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct FileCopyTool;

#[async_trait]
impl Tool for FileCopyTool {
    fn name(&self) -> &str {
        "file_copy"
    }

    fn description(&self) -> &str {
        "Copy a file to another location."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "from": {
                    "type": "string",
                    "description": "Source file path"
                },
                "to": {
                    "type": "string",
                    "description": "Destination file path"
                }
            },
            "required": ["from", "to"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let from = input["from"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing from field"))?;
        let to = input["to"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing to field"))?;

        fs::copy(from, to).await?;
        Ok(format!("Copied {} to {}", from, to))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct FileInfoTool;

#[async_trait]
impl Tool for FileInfoTool {
    fn name(&self) -> &str {
        "file_info"
    }

    fn description(&self) -> &str {
        "Get detailed information about a file or directory."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File or directory path"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;

        let metadata = fs::metadata(path).await?;
        
        let file_type = if metadata.is_dir() {
            "Directory"
        } else if metadata.is_file() {
            "File"
        } else {
            "Other"
        };

        let permissions = if metadata.permissions().readonly() {
            "Read-only"
        } else {
            "Read-write"
        };

        Ok(format!(
            "Path: {}\nType: {}\nSize: {} bytes\nPermissions: {}\nModified: {:?}",
            path,
            file_type,
            metadata.len(),
            permissions,
            metadata.modified()?
        ))
    }
}