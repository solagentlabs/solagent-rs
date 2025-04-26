mod tool;
mod tool_impl;

use solagent_core::{tool::SolAgentTool, SolAgent};
use std::sync::Arc;
pub use tool::{CloseEmptyTokenAccounts, CloseEmptyTokenAccountsOutput};

pub fn get_tool(solagent: Arc<SolAgent>) -> SolAgentTool<CloseEmptyTokenAccounts> {
    let tps = tool::CloseEmptyTokenAccounts::new(solagent);
    let tool = SolAgentTool::new(tps);
    tool
}
