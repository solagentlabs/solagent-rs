use crate::get_tps::tool::{GetTps, GetTpsArgs, GetTpsError, GetTpsOutput};
use rig::{completion::ToolDefinition, tool::Tool};
use solagent_plugin_solana::get_tps;

impl Tool for GetTps {
    const NAME: &'static str = "get_tps";

    type Error = GetTpsError;
    type Args = GetTpsArgs;
    type Output = GetTpsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_tps".to_string(),
            description: "Get the current transactions per second (TPS) of the Solana network"
                .to_string(),
            parameters: serde_json::json!(null),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tps = get_tps(&self.solagent).await.expect("get_tps");
        Ok(GetTpsOutput { tps })
    }
}
