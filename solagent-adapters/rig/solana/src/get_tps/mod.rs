pub(crate) mod tool;
pub(crate) mod tool_impl;

use solagent_core::{tool::SolAgentTool, SolAgent};
use std::sync::Arc;
pub use tool::{GetTps, GetTpsOutput};

pub fn get_tool(solagent: Arc<SolAgent>) -> SolAgentTool<GetTps> {
    let tps = tool::GetTps::new(solagent);
    let tool = SolAgentTool::new(tps);
    tool
}
