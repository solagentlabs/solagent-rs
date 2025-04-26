use {
    serde::{Deserialize, Serialize},
    solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        transaction::Transaction,
    },
    solagent_core::{SolAgent, solana_client::rpc_request::TokenAccountsFilter},
    spl_token::instruction::close_account,
    anyhow::Result,
};

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
/// - `solagent`: An instance of `SolAgent`.
///
/// # Returns
///
/// Transaction signature and total number of accounts closed or an error if the account doesn't exist.
pub async fn close_empty_token_accounts(
    solagent: &SolAgent,
) -> Result<CloseEmptyTokenAccountsData> {
    let max_instructions = 40_u32;
    let mut transaction: Vec<Instruction> = vec![];
    let mut closed_size = 0;
    let token_programs = vec![spl_token::ID, spl_token_2022::ID];

    for token_program in token_programs {
        let accounts = solagent.rpc_client
            .get_token_accounts_by_owner(
                &solagent.wallet.pubkey,
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
                            &solagent.wallet.pubkey,
                            &solagent.wallet.pubkey,
                            &[&solagent.wallet.pubkey],
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
    let recent_blockhash = solagent.rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &transaction,
        Some(&solagent.wallet.pubkey),
        &[&solagent.wallet.keypair],
        recent_blockhash,
    );

    let signature = solagent.rpc_client
        .send_and_confirm_transaction(&transaction)?;
    let data = CloseEmptyTokenAccountsData::new(signature.to_string(), closed_size);
    Ok(data)
}
