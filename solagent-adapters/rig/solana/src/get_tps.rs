use serde::{Deserialize, Serialize};
use solagent_plugin_solana::get_tps;
use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use std::sync::Arc;
use solagent_core::{SolAgent, tool::SolAgentTool};

#[derive(Deserialize)]
pub struct GetTpsArgs {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTpsOutput {
    pub tps: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTps error")]
pub struct GetTpsError;

#[derive(Clone)]
pub struct GetTps {
    solagent: Arc<SolAgent>,
}

impl GetTps {
    pub fn new(solagent: Arc<SolAgent>) -> Self {
        GetTps { solagent }
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
        let tps = get_tps(&self.solagent).await.expect("get_tps");
        Ok(GetTpsOutput { tps })
    }
}

pub fn tool(solagent: Arc<SolAgent>) -> SolAgentTool<GetTps> {
    let tps = GetTps::new(solagent);
    let tool = SolAgentTool::new(tps);
    tool
}