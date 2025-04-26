use rig::{completion::ToolDefinition, tool::ToolSet};
use serde_json::Value;
use std::error::Error;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// A wrapper for `Tool` implementations.
pub struct SolAgentTool {
    pub tool_names: Vec<String>,
    pub toolset: ToolSet,
}

impl SolAgentTool {
    pub fn new(tool_names: Vec<String>, toolset: ToolSet) -> Self {
        Self { tool_names, toolset }
    }
}
