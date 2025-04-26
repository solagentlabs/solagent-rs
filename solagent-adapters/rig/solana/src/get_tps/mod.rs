pub(crate) mod tool;
pub(crate) mod tool_impl;

use solagent_core::{tool::SolAgentTool, SolAgent};
use std::sync::Arc;
pub use tool::{GetTps, GetTpsOutput};
// use rig::tool::{ToolSetBuilder, ToolSet};

// pub fn get_tools(solagent: Arc<SolAgent>) -> ToolSet {
//     let tps = tool::GetTps::new(solagent);
    
//     let toolset = ToolSetBuilder::static_tool(tps).build();
//     toolset
// }
