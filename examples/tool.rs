use async_std::sync::Arc;
use serde_json::json;
use solagent::{
    llm_integration::{Grok3Provider, LLMClient, OpenAIProvider, GeminiProvider, LLMProvider},
    tool_system::{ToolInput, ToolMetadata, SolanaTool},
    SolAgent, SolAgentConfig,
};

// Example Solana balance tool compatible with LLMs
struct BalanceTool;

#[async_trait::async_trait]
impl SolanaTool for BalanceTool {
    async fn execute(
        &self,
        input: ToolInput,
        llm: &dyn LLMProvider,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let pubkey = input.params["pubkey"]
            .as_str()
            .ok_or("Missing pubkey")?;
        let prompt = format!("Get the balance for Solana address: {}", pubkey);
        let response = llm.call(&prompt, vec![]).await?;
        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
            .to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize SolAgent with config
    let config = SolAgentConfig {
        name: "Solana Agent".to_string(),
        instructions: "Execute Solana-related tasks with tools and LLM.".to_string(),
        model: "grok3".to_string(),
        tools: vec!["get_balance".to_string()],
    };
    let solagent = SolAgent::new(config)?;

    // Register balance tool for Grok 3
    let grok3_tool_metadata = ToolMetadata {
        name: "get_balance".to_string(),
        aliases: vec!["balance".to_string()],
        version: "1.0".to_string(),
        llm_type: "grok3".to_string(),
        schema: json!({
            "name": "get_balance",
            "description": "Get Solana account balance",
            "parameters": {
                "type": "object",
                "properties": {
                    "pubkey": {"type": "string"}
                }
            }
        }),
    };
    solagent
        .tool_registry
        .register(grok3_tool_metadata, Arc::new(BalanceTool))
        .await;

    // Execute task with Grok 3
    let input = json!({ "pubkey": "abc123" });
    let result = solagent.execute_task("get_balance", input).await?;
    println!("Grok 3 Balance Result: {}", result);

    Ok(())
}