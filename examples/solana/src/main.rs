use anyhow::Result;
use solagent_core::*;
use solagent_rig_solana::get_tps::*;
use solagent_wallet_solana::SolAgentWallet;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let wallet = SolAgentWallet::new("https://api.mainnet-beta.solana.com");
    let wallet = Arc::new(wallet);

    let tps = GetTps::new(wallet.clone());
    let tool = tool::SolAgentTool::new(tps);

    let wallet = SolAgentWallet::new("https://api.mainnet-beta.solana.com");
    let solagent = SolAgent::new(wallet);
    let result = solagent
        .prompt(
            // model::SolAgentModel::Gemini("gemini-2.0-flash".to_string()),
            model::SolAgentModel::Ollama("llama3.2".to_string()),
            vec![tool],
            "get solana tps",
        )
        .await?;

    println!("Result: {:?}", result);

    Ok(())
}
