use anyhow::Result;
use solagent_core::*;
use solagent_rig_solana::get_tps::{self, GetTpsOutput};
use solagent_wallet_solana::SolAgentWallet;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let wallet = SolAgentWallet::new("https://api.mainnet-beta.solana.com");
    let solagent = Arc::new(SolAgent::new(wallet));

    let tps_tool = get_tps::tool(solagent.clone());

    let result = solagent
        .prompt(
            model::SolAgentModel::Ollama("llama3.2".to_string()),
            vec![tps_tool],
            "get solana tps",
        )
        .await?;

    match serde_json::from_str::<GetTpsOutput>(&result) {
        Ok(output) => println!("Result: {:#?}", output),
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    };
    
    Ok(())
}
