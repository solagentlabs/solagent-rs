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
use rig::{
        completion::ToolDefinition,
        tool::{Tool, ToolEmbedding},
    };

use solagent_core::SolAgent;
use solagent_plugin_solana::{close_empty_token_accounts, CloseEmptyTokenAccountsData};
use anyhow::Result;
use rig_tool_macro::tool;
use solagent_wallet_solana::SolAgentWallet;
use solana_sdk::keypair::Keypair;

// #[derive(Debug, Deserialize)]
// pub struct CloseEmptyTokenAccountsArgs {}

#[derive(Deserialize, Serialize)]
pub struct CloseEmptyTokenAccountsOutput {
    pub data: CloseEmptyTokenAccountsData,
}

// #[derive(Debug, thiserror::Error)]
// #[error("CloseEmptyTokenAccounts error")]
// pub struct CloseEmptyTokenAccountsError;

// pub struct CloseEmptyTokenAccounts {
//     agent: Arc<SolAgent>,
// }

// impl CloseEmptyTokenAccounts {
//     pub fn new(agent: Arc<SolAgent>) -> Self {
//         CloseEmptyTokenAccounts { agent }
//     }
// }

// impl Tool for CloseEmptyTokenAccounts {
//     const NAME: &'static str = "close_empty_token_accounts";

//     type Error = CloseEmptyTokenAccountsError;
//     type Args = CloseEmptyTokenAccountsArgs;
//     type Output = CloseEmptyTokenAccountsOutput;

//     async fn definition(&self, _prompt: String) -> ToolDefinition {
//         ToolDefinition {
//             name: "close_empty_token_accounts".to_string(),
//             description: r#"
//             Close empty SPL Token accounts associated with your wallet to reclaim rent. 
//             This action will close both regular SPL Token accounts and Token-2022 accounts that have zero balance. 

//             examples: [
//                 [
//                     {
//                         input: {},
//                         output: {
//                             status: "success",
//                             signature:
//                                 "3KmPyiZvJQk8CfBVVaz8nf3c2crb6iqjQVDqNxknnusyb1FTFpXqD8zVSCBAd1X3rUcD8WiG1bdSjFbeHsmcYGXY",
//                             accountsClosed: 10,
//                         },
//                         explanation: "Closed 10 empty token accounts successfully.",
//                     },
//                 ],
//                 [
//                     {
//                         input: {},
//                         output: {
//                             status: "success",
//                             signature: "",
//                             accountsClosed: 0,
//                         },
//                         explanation: "No empty token accounts were found to close.",
//                     },
//                 ],
//             ]

// "#.to_string(),
//             parameters: serde_json::Value::Null,
//         }
//     }

//     async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
//         let data = close_empty_token_accounts(&self.agent)
//             .await
//             .expect("close_empty_token_accounts");

//         Ok(CloseEmptyTokenAccountsOutput { data })
//     }
// }

// #[derive(Debug, thiserror::Error)]
// #[error("Init error")]
// pub struct InitError;

// impl ToolEmbedding for CloseEmptyTokenAccounts {
//     type InitError = InitError;
//     type Context = ();
//     type State = Arc<SolAgent>;

//     fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
//         Ok(CloseEmptyTokenAccounts { agent: state })
//     }

//     fn embedding_docs(&self) -> Vec<String> {
//         vec!["Close empty SPL Token accounts associated with your wallet to reclaim rent.".into()]
//     }

//     fn context(&self) -> Self::Context {}
// }

#[tool(
    description = "
    Close empty SPL Token accounts associated with your wallet to reclaim rent. 
            This action will close both regular SPL Token accounts and Token-2022 accounts that have zero balance. 
")]
pub async fn close_empty_token_accounts_tool(wallet: SolAgentWallet) -> Result<CloseEmptyTokenAccountsOutput> {
    let data = close_empty_token_accounts(wallet)
    .await
    .expect("close_empty_token_accounts");

    Ok(CloseEmptyTokenAccountsOutput { data })
}