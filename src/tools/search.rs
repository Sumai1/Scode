use super::base::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use glob::glob as glob_pattern;

pub struct GlobTool;

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Search for files matching a glob pattern. Returns list of matching file paths."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Glob pattern (e.g., '**/*.rs', 'src/**/*.txt')"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let pattern = input["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing pattern field"))?;

        let mut matches = Vec::new();
        for entry in glob_pattern(pattern)? {
            match entry {
                Ok(path) => matches.push(path.display().to_string()),
                Err(e) => eprintln!("Glob error: {}", e),
            }
        }

        if matches.is_empty() {
            Ok("No files matched the pattern".to_string())
        } else {
            Ok(format!("Found {} files:\n{}", matches.len(), matches.join("\n")))
        }
    }
}

pub struct GrepTool;

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for text patterns in files using regex. Returns matching lines with line numbers."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Regex pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "File or directory to search in"
                }
            },
            "required": ["pattern", "path"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let pattern = input["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing pattern field"))?;
        let path = input["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path field"))?;

        // Use ripgrep if available, otherwise fall back to grep
        let output = tokio::process::Command::new("rg")
            .arg("-n")
            .arg(pattern)
            .arg(path)
            .output()
            .await;

        let result = match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if stdout.is_empty() {
                    "No matches found".to_string()
                } else {
                    stdout.to_string()
                }
            }
            Err(_) => {
                // Fallback to grep
                let out = tokio::process::Command::new("grep")
                    .arg("-rn")
                    .arg(pattern)
                    .arg(path)
                    .output()
                    .await?;
                String::from_utf8_lossy(&out.stdout).to_string()
            }
        };

        Ok(result)
    }
}
