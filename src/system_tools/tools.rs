use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::llm_integration::LLMProvider;

pub mod account;
pub mod defi;
pub mod misc;
pub mod nft;
pub mod staking;
pub mod transaction;

// Tool metadata for versioning, dependencies, and LLM compatibility
#[derive(Serialize, Deserialize, Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub aliases: Vec<String>,
    pub version: String,
    pub llm_type: String,
    pub schema: serde_json::Value,
}

// Tool input structure
#[derive(Serialize, Deserialize, Clone)]
pub struct ToolInput {
    pub params: serde_json::Value,
}

// Solana and LLM tool trait
#[async_trait]
pub trait SolanaTool: Send + Sync {
    async fn execute(
        &self,
        input: ToolInput,
        llm: &dyn LLMProvider,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

// Tool registry for managing tools
pub struct ToolRegistry {
    tools: tokio::sync::RwLock<
        std::collections::HashMap<String, (ToolMetadata, Arc<dyn SolanaTool>)>,
    >,
}

impl ToolRegistry {
    pub fn new() -> Self {
        ToolRegistry {
            tools: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    // Registers a tool with metadata
    pub async fn register(
        &self,
        metadata: ToolMetadata,
        tool: Arc<dyn SolanaTool>,
    ) {
        let mut tools = self.tools.write().await;
        tools.insert(metadata.name.clone(), (metadata, tool));
    }

    // Retrieves a tool by name or alias
    pub async fn get(
        &self,
        name: &str,
    ) -> Option<(ToolMetadata, Arc<dyn SolanaTool>)> {
        let tools = self.tools.read().await;
        tools
            .iter()
            .find(|(key, (meta, _))| key == name || meta.aliases.contains(&name.to_string()))
            .map(|(_, v)| v.clone())
    }

    // Registers specified tools
    pub async fn register_tools(&self, tool_names: &[String], rpc_url: &str) {
        for name in tool_names {
            match name.as_str() {
                "get_balance" => account::register_account_tools(self, rpc_url).await,
                "stake_sol" => staking::register_staking_tools(self, rpc_url).await,
                // Add other tools (defi, nft, transaction, misc)
                _ => println!("Warning: Tool '{}' not found", name),
            }
        }
    }
}