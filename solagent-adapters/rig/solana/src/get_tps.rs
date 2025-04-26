// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use solagent_plugin_solana::get_tps;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
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
            description: r#"
            
            Get the current transactions per second (TPS) of the Solana network
            
            examples: [
                [
                    {
                        input: {},
                        output: {
                            status: "success",
                            tps: 3500,
                            message: "Current network TPS: 3500",
                        },
                        explanation: "Get the current TPS of the Solana network",
                    },
                ],
            ]
            
            "#
            .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tps = get_tps(&self.wallet).await.expect("get_tps");

        Ok(GetTpsOutput { tps })
    }
}

// #[tool(
//     description = "
//     Get the current transactions per second (TPS) of the Solana network
// ")]
// pub async fn get_tps_tool(wallet: &SolAgentWallet) -> Result<GetTpsOutput> {
//     let tps = get_tps(keypair)
//     .await
//     .expect("get_tps");

//     Ok(GetTpsOutput { tps })
// }