use serde::{Deserialize, Serialize};
use solagent_plugin_solana::get_tps;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use solagent_wallet_solana::SolAgentWallet;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetTpsArgs {}

#[derive(Deserialize, Serialize)]
pub struct GetTpsOutput {
    pub tps: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTps error")]
pub struct GetTpsError;

#[derive(Debug, Clone)]
pub struct GetTps {
    wallet: Arc<SolAgentWallet>,
}

impl GetTps {
    pub fn new(wallet: Arc<SolAgentWallet>) -> Self {
        GetTps { wallet }
    }
}

impl Tool for GetTps {
    const NAME: &'static str = "get_tps";

    type Error = GetTpsError;
    type Args = GetTpsArgs;
    type Output = GetTpsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_tps".to_string(),
            description: "Get the current transactions per second (TPS) of the Solana network".to_string(),
            parameters: serde_json::json!(null),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tps = get_tps(&self.wallet).await.expect("get_tps");
        Ok(GetTpsOutput { tps })
    }
}