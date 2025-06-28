use solagent::{SolAgent, SolAgentConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize SolAgent with config
    let config = SolAgentConfig {
        name: "Solana Agent".to_string(),
        instructions: "Execute Solana-related tasks with tools and LLM.".to_string(),
        model: "grok3".to_string(),
        tools: vec!["get_balance".to_string(), "stake_sol".to_string()],
    };
    let solagent = SolAgent::new(config)?;

    // Check if running in CLI mode
    if env::args().len() > 1 {
        solagent.cli.handle_args().await?;
    } else {
        // Start API server
        solagent.api.run().await?;
    }

    Ok(())
}