use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub api: ApiConfig,
    
    #[serde(default)]
    pub agent: AgentConfig,
    
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,
    
    #[serde(default = "default_auto_approve_read_only")]
    pub auto_approve_read_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    #[serde(default = "default_true")]
    pub colors: bool,
    
    #[serde(default = "default_true")]
    pub show_tokens: bool,
    
    #[serde(default = "default_true")]
    pub show_progress: bool,
}

fn default_max_iterations() -> usize {
    20
}

fn default_auto_approve_read_only() -> bool {
    false
}

fn default_true() -> bool {
    true
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: None,
            model: Some("claude-sonnet-4-5-20250929".to_string()),
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: 20,
            auto_approve_read_only: false,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            colors: true,
            show_tokens: true,
            show_progress: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: ApiConfig::default(),
            agent: AgentConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        Ok(home.join(".scode").join("config.toml"))
    }

    pub async fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).await?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub async fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&path, content).await?;
        
        Ok(())
    }

    pub async fn init_default() -> Result<()> {
        let config = Self::default();
        config.save().await?;
        println!("✅ Created default config at: {}", Self::config_path()?.display());
        Ok(())
    }
}
