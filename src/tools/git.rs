use super::base::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::process::Command;

pub struct GitStatusTool;

#[async_trait]
impl Tool for GitStatusTool {
    fn name(&self) -> &str {
        "git_status"
    }

    fn description(&self) -> &str {
        "Get the current git repository status, including modified files, staged changes, and branch information."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn execute(&self, _input: Value) -> Result<String> {
        let output = Command::new("git")
            .arg("status")
            .arg("--short")
            .arg("--branch")
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Error: {}", stderr));
        }

        Ok(stdout.to_string())
    }
}

pub struct GitDiffTool;

#[async_trait]
impl Tool for GitDiffTool {
    fn name(&self) -> &str {
        "git_diff"
    }

    fn description(&self) -> &str {
        "Show changes in the working directory or staged changes."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "staged": {
                    "type": "boolean",
                    "description": "Show staged changes instead of working directory changes"
                },
                "file": {
                    "type": "string",
                    "description": "Show diff for a specific file"
                }
            }
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let staged = input["staged"].as_bool().unwrap_or(false);
        let file = input["file"].as_str();

        let mut cmd = Command::new("git");
        cmd.arg("diff");

        if staged {
            cmd.arg("--staged");
        }

        if let Some(f) = file {
            cmd.arg(f);
        }

        let output = cmd.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        Ok(stdout.to_string())
    }
}

pub struct GitCommitTool;

#[async_trait]
impl Tool for GitCommitTool {
    fn name(&self) -> &str {
        "git_commit"
    }

    fn description(&self) -> &str {
        "Create a git commit with the specified message. Files must be staged first."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Commit message"
                }
            },
            "required": ["message"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let message = input["message"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing message field"))?;

        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Error: {}", stderr));
        }

        Ok(format!("{}{}", stdout, stderr))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct GitAddTool;

#[async_trait]
impl Tool for GitAddTool {
    fn name(&self) -> &str {
        "git_add"
    }

    fn description(&self) -> &str {
        "Stage files for commit."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "files": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Files to stage (use '.' for all files)"
                }
            },
            "required": ["files"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let files = input["files"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing files field"))?;

        let mut cmd = Command::new("git");
        cmd.arg("add");

        for file in files {
            if let Some(f) = file.as_str() {
                cmd.arg(f);
            }
        }

        let output = cmd.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Error: {}", stderr));
        }

        Ok(format!("Files staged successfully\n{}{}", stdout, stderr))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct GitLogTool;

#[async_trait]
impl Tool for GitLogTool {
    fn name(&self) -> &str {
        "git_log"
    }

    fn description(&self) -> &str {
        "Show recent commit history."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "number",
                    "description": "Number of commits to show (default: 10)"
                }
            }
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let count = input["count"].as_u64().unwrap_or(10);

        let output = Command::new("git")
            .arg("log")
            .arg(format!("-{}", count))
            .arg("--oneline")
            .arg("--decorate")
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }
}

pub struct GitBranchTool;

#[async_trait]
impl Tool for GitBranchTool {
    fn name(&self) -> &str {
        "git_branch"
    }

    fn description(&self) -> &str {
        "List, create, or switch git branches."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "create", "switch"],
                    "description": "Action to perform"
                },
                "name": {
                    "type": "string",
                    "description": "Branch name (for create/switch)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let action = input["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing action field"))?;

        let mut cmd = Command::new("git");

        match action {
            "list" => {
                cmd.arg("branch").arg("-a");
            }
            "create" => {
                let name = input["name"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing name field"))?;
                cmd.arg("branch").arg(name);
            }
            "switch" => {
                let name = input["name"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing name field"))?;
                cmd.arg("checkout").arg(name);
            }
            _ => anyhow::bail!("Invalid action: {}", action),
        }

        let output = cmd.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Error: {}", stderr));
        }

        Ok(format!("{}{}", stdout, stderr))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct GitPullTool;

#[async_trait]
impl Tool for GitPullTool {
    fn name(&self) -> &str {
        "git_pull"
    }

    fn description(&self) -> &str {
        "Pull changes from remote repository."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "remote": {
                    "type": "string",
                    "description": "Remote name (default: origin)"
                },
                "branch": {
                    "type": "string",
                    "description": "Branch name (optional)"
                }
            }
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let remote = input["remote"].as_str().unwrap_or("origin");
        let branch = input["branch"].as_str();

        let mut cmd = Command::new("git");
        cmd.arg("pull").arg(remote);

        if let Some(b) = branch {
            cmd.arg(b);
        }

        let output = cmd.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Error: {}", stderr));
        }

        Ok(format!("{}{}", stdout, stderr))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}

pub struct GitPushTool;

#[async_trait]
impl Tool for GitPushTool {
    fn name(&self) -> &str {
        "git_push"
    }

    fn description(&self) -> &str {
        "Push changes to remote repository."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "remote": {
                    "type": "string",
                    "description": "Remote name (default: origin)"
                },
                "branch": {
                    "type": "string",
                    "description": "Branch name (optional)"
                },
                "force": {
                    "type": "boolean",
                    "description": "Force push (dangerous!)"
                }
            }
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let remote = input["remote"].as_str().unwrap_or("origin");
        let branch = input["branch"].as_str();
        let force = input["force"].as_bool().unwrap_or(false);

        let mut cmd = Command::new("git");
        cmd.arg("push");

        if force {
            cmd.arg("--force");
        }

        cmd.arg(remote);

        if let Some(b) = branch {
            cmd.arg(b);
        }

        let output = cmd.output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Ok(format!("Error: {}", stderr));
        }

        Ok(format!("{}{}", stdout, stderr))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}
