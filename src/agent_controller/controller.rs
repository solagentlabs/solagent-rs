use async_std::sync::Arc;
use serde_json::Value;

use crate::{
    llm_integration::LLMClient,
    memory_system::{LongTermMemory, ShortTermMemory},
    tool_system::{ToolInput, ToolMetadata, ToolRegistry, SolanaTool},
};

// Agent Controller structure
pub struct AgentController {
    tool_registry: Arc<ToolRegistry>,
    memory_short: Arc<ShortTermMemory>,
    memory_long: Arc<LongTermMemory>,
    llm_client: Arc<LLMClient>,
}

impl AgentController {
    pub fn new(
        tool_registry: Arc<ToolRegistry>,
        memory_short: Arc<ShortTermMemory>,
        memory_long: Arc<LongTermMemory>,
        llm_client: Arc<LLMClient>,
    ) -> Self {
        AgentController {
            tool_registry,
            memory_short,
            memory_long,
            llm_client,
        }
    }

    // Executes a task by selecting a tool and calling the appropriate LLM
    pub async fn execute_task(
        &self,
        task: &str,
        input: Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Retrieve tool and metadata
        let (metadata, tool) = self
            .tool_registry
            .get(task)
            .await
            .ok_or(format!("Tool '{}' not found", task))?;

        // Get LLM provider
        let llm_provider = self
            .llm_client
            .providers
            .read()
            .await
            .get(&metadata.llm_type)
            .ok_or(format!("LLM provider '{}' not found", metadata.llm_type))?
            .clone();

        // Construct prompt with task and context
        let context = self.memory_short.context.read().await;
        let prompt = format!(
            "Execute task: {}\nInput: {}\nContext: {:?}",
            task, input, context
        );

        // Call LLM
        let llm_response = llm_provider.call(&prompt, vec![metadata.clone()]).await?;

        // Parse LLM response
        let tool_call = parse_llm_response(&llm_response, &metadata.llm_type)?;
        let tool_input = ToolInput {
            params: tool_call.get("arguments").cloned().unwrap_or(input),
        };

        // Execute tool
        let result = tool.execute(tool_input, llm_provider.as_ref()).await?;

        // Store result in memory
        self.memory_short
            .context
            .write()
            .await
            .insert(task.to_string(), result.clone());

        Ok(result)
    }
}

// Helper function to parse LLM response
fn parse_llm_response(response: &Value, llm_type: &str) -> Result<Value, Box<dyn std::error::Error>> {
    match llm_type {
        "grok3" => Ok(response["choices"][0]["message"]["tool_calls"][0].clone()),
        "gemini" => Ok(response["candidates"][0]["content"]["parts"][0]["functionCall"].clone()),
        "openai" => Ok(response["choices"][0]["message"]["tool_calls"][0].clone()),
        _ => Err("Unsupported LLM type".into()),
    }
}