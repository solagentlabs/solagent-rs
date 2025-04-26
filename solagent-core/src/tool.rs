use rig::{completion::ToolDefinition, tool::ToolDyn};
use serde_json::Value;
use std::error::Error;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// A wrapper for `Tool` implementations.
pub struct SolAgentTool<T: ToolDyn> {
    tool: T,
}

impl<T: ToolDyn> SolAgentTool<T> {
    pub fn new(tool: T) -> Self {
        Self { tool }
    }

    pub fn get_tool(&self) -> &T {
        &self.tool
    }

    pub fn register(&mut self, tool: T) {
        self.tool = tool;
    }
}
