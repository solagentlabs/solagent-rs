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
use solagent_core::{
    solana_client::rpc_request::TokenAccountsFilter,
    solana_sdk::{instruction::Instruction, pubkey::Pubkey, keypair::Keypair, client::Client,transaction::Transaction},
    SolAgent,
};
use spl_token::instruction::close_account;
use solagent_wallet_solana::wallet::SolAgentWallet;

pub const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[derive(serde::Deserialize)]
pub struct Parsed {
    pub info: SplToken,
}

#[derive(serde::Deserialize)]
pub struct SplToken {
    pub mint: String,
    #[serde(rename(deserialize = "tokenAmount"))]
    pub token_amount: Amount,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct Amount {
    pub amount: String,
    #[serde(rename(deserialize = "uiAmountString"))]
    ui_amount_string: String,
    #[serde(rename(deserialize = "uiAmount"))]
    pub ui_amount: f64,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CloseEmptyTokenAccountsData {
    pub signature: String,
    pub closed_size: usize,
}

impl CloseEmptyTokenAccountsData {
    pub fn new(signature: String, closed_size: usize) -> Self {
        CloseEmptyTokenAccountsData {
            signature,
            closed_size,
        }
    }
}

/// Close Empty SPL Token accounts of the agent.
///
/// # Parameters
///
/// - `wallet`: An instance of `SolAgentWallet`.
///
/// # Returns
///
/// Transaction signature and total number of accounts closed or an error if the account doesn't exist.
pub async fn close_empty_token_accounts(
    wallet: &SolAgentWallet,
) -> Result<CloseEmptyTokenAccountsData, Box<dyn std::error::Error>> {
    let max_instructions = 40_u32;
    let mut transaction: Vec<Instruction> = vec![];
    let mut closed_size = 0;
    let token_programs = vec![spl_token::ID, spl_token_2022::ID];

    let rpc_url = wallet.rpc_url.clone();
    let client = Client::new(rpc_url)?;

    for token_program in token_programs {
        let accounts = client
            .get_token_accounts_by_owner(
                &wallet.pubkey,
                TokenAccountsFilter::ProgramId(token_program.to_owned()),
            )
            .expect("get_token_accounts_by_owner");

        closed_size += accounts.len();

        for account in accounts {
            if transaction.len() >= max_instructions as usize {
                break;
            }

            if let solana_account_decoder::UiAccountData::Json(d) = &account.account.data {
                if let Ok(parsed) = serde_json::from_value::<Parsed>(d.parsed.clone()) {
                    if parsed
                        .info
                        .token_amount
                        .amount
                        .parse::<u32>()
                        .unwrap_or_default()
                        == 0_u32
                        && parsed.info.mint != USDC
                    {
                        let account_pubkey = Pubkey::from_str_const(&account.pubkey);
                        if let Ok(instruct) = close_account(
                            &token_program,
                            &account_pubkey,
                            &wallet.pubkey,
                            &wallet.pubkey,
                            &[&wallet.pubkey],
                        ) {
                            transaction.push(instruct);
                        }
                    }
                }
            }
        }
    }

    if transaction.is_empty() {
        return Ok(CloseEmptyTokenAccountsData::default());
    }

    // Create and send transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &transaction,
        Some(&wallet.pubkey),
        &[&wallet.keypair],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)?;
    let data = CloseEmptyTokenAccountsData::new(signature.to_string(), closed_size);
    Ok(data)
}
