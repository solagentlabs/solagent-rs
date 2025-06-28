use async_std::sync::Arc;
use serde_json::json;
use solagent::{SolAgent, SolAgentConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize SolAgent with config
    let config = SolAgentConfig {
        name: "Solana Agent".to_string(),
        instructions: "Execute Solana-related tasks with tools and LLM.".to_string(),
        model: "grok3".to_string(),
        tools: vec!["stake_sol".to_string(), "get_balance".to_string()],
    };
    let solagent = SolAgent::new(config)?;

    // Execute staking task
    let input = json!({ "amount": 10.0, "validator": "validator_pubkey" });
    let result = solagent.execute_task("stake_sol", input).await?;
    println!("Stake Result: {}", result);

    // Test with alias
    let result = solagent.execute_task("stake", input).await?;
    println!("Stake Result (using alias): {}", result);

    Ok(())
}