use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use uuid::Uuid;

use crate::api::types::{ContentBlock, Message};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionEntry {
    pub timestamp: DateTime<Utc>,
    pub message: Message,
}

pub struct SessionManager {
    sessions_dir: PathBuf,
    current_session_id: Option<String>,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        let sessions_dir = home.join(".scode").join("sessions");
        
        Ok(Self {
            sessions_dir,
            current_session_id: None,
        })
    }

    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.sessions_dir).await?;
        Ok(())
    }

    pub fn new_session(&mut self) -> String {
        let id = Uuid::new_v4().to_string();
        self.current_session_id = Some(id.clone());
        id
    }

    pub fn set_session(&mut self, id: String) {
        self.current_session_id = Some(id);
    }

    fn session_path(&self, session_id: &str) -> PathBuf {
        self.sessions_dir.join(format!("{}.jsonl", session_id))
    }

    pub async fn save_message(&self, message: Message) -> Result<()> {
        if let Some(session_id) = &self.current_session_id {
            let path = self.session_path(session_id);
            
            let entry = SessionEntry {
                timestamp: Utc::now(),
                message,
            };
            
            let json = serde_json::to_string(&entry)?;
            
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .await?;
            
            file.write_all(json.as_bytes()).await?;
            file.write_all(b"\n").await?;
            file.flush().await?;
        }
        
        Ok(())
    }

    pub async fn load_session(&self, session_id: &str) -> Result<Vec<Message>> {
        let path = self.session_path(session_id);
        
        if !path.exists() {
            anyhow::bail!("Session not found: {}", session_id);
        }
        
        let file = fs::File::open(&path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        
        let mut messages = Vec::new();
        
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }
            
            let entry: SessionEntry = serde_json::from_str(&line)?;
            messages.push(entry.message);
        }
        
        Ok(messages)
    }

    pub async fn list_sessions(&self) -> Result<Vec<SessionMetadata>> {
        let mut sessions = Vec::new();
        
        if !self.sessions_dir.exists() {
            return Ok(sessions);
        }
        
        let mut entries = fs::read_dir(&self.sessions_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
                continue;
            }
            
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(metadata) = self.get_session_metadata(stem).await {
                    sessions.push(metadata);
                }
            }
        }
        
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        
        Ok(sessions)
    }

    async fn get_session_metadata(&self, session_id: &str) -> Result<SessionMetadata> {
        let path = self.session_path(session_id);
        let file = fs::File::open(&path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        
        let mut created_at = None;
        let mut updated_at = None;
        let mut message_count = 0;
        
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }
            
            if let Ok(entry) = serde_json::from_str::<SessionEntry>(&line) {
                if created_at.is_none() {
                    created_at = Some(entry.timestamp);
                }
                updated_at = Some(entry.timestamp);
                message_count += 1;
            }
        }
        
        Ok(SessionMetadata {
            id: session_id.to_string(),
            created_at: created_at.unwrap_or_else(Utc::now),
            updated_at: updated_at.unwrap_or_else(Utc::now),
            message_count,
        })
    }

    pub fn current_session_id(&self) -> Option<&str> {
        self.current_session_id.as_deref()
    }
}
