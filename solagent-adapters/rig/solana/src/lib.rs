pub mod close_empty_account;
// pub mod deploy_collection;
// pub mod deploy_token;
// pub mod get_balance;
// pub mod get_balance_other;

pub mod get_tps;

// pub mod get_wallet_address;
// pub mod mint_nft;
// pub mod request_faucet_funds;
// pub mod transfer;

use solagent_core::{tool::SolAgentTool, SolAgent};
use std::sync::Arc;
use rig::tool::{ToolSetBuilder, ToolSet};
use crate::close_empty_account::CloseEmptyTokenAccounts;
use crate::get_tps::GetTps;

pub fn get_solana_tools(solagent: Arc<SolAgent>) -> ToolSet {
    let tps = GetTps::new(solagent.clone());
    let close = CloseEmptyTokenAccounts::new(solagent.clone());
    
    let toolset = ToolSet::builder()
    	.static_tool(tps)
    	.static_tool(close)
	    .build();

	toolset
}
