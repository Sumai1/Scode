pub mod base;
pub mod bash;
pub mod file;
pub mod search;
pub mod git;
pub mod web;
pub mod filesystem;

use std::collections::HashMap;
use std::sync::Arc;
use self::base::Tool;

pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        
        // Register all tools
        registry.register(Arc::new(bash::BashTool));
        registry.register(Arc::new(file::FileReadTool));
        registry.register(Arc::new(file::FileWriteTool));
        registry.register(Arc::new(file::FileEditTool));
        registry.register(Arc::new(search::GlobTool));
        registry.register(Arc::new(search::GrepTool));
        registry.register(Arc::new(git::GitStatusTool));
        registry.register(Arc::new(git::GitDiffTool));
        registry.register(Arc::new(git::GitCommitTool));
        registry.register(Arc::new(git::GitAddTool));
        registry.register(Arc::new(git::GitLogTool));
        registry.register(Arc::new(git::GitBranchTool));
        registry.register(Arc::new(git::GitPullTool));
        registry.register(Arc::new(git::GitPushTool));
        registry.register(Arc::new(web::WebFetchTool::new()));
        registry.register(Arc::new(web::WebSearchTool::new()));
        registry.register(Arc::new(web::HttpRequestTool::new()));
        registry.register(Arc::new(filesystem::FileListTool));
        registry.register(Arc::new(filesystem::FileMoveTool));
        registry.register(Arc::new(filesystem::FileDeleteTool));
        registry.register(Arc::new(filesystem::FileCopyTool));
        registry.register(Arc::new(filesystem::FileInfoTool));
        
        registry
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub fn list(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.values().cloned().collect()
    }

    pub fn to_api_tools(&self) -> Vec<crate::api::types::Tool> {
        self.tools
            .values()
            .map(|t| crate::api::types::Tool {
                name: t.name().to_string(),
                description: t.description().to_string(),
                input_schema: t.input_schema(),
            })
            .collect()
    }
}
