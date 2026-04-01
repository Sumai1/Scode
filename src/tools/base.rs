use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    async fn execute(&self, input: Value) -> Result<String>;
    
    fn requires_permission(&self) -> bool {
        false
    }
    
    fn is_destructive(&self) -> bool {
        false
    }
}
