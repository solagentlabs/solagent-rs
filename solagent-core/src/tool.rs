use rig::tool::Tool;

/// A wrapper for `Tool` implementations.
pub struct SolAgentTool<T: Tool> {
    tool: T,
}

impl<T: Tool> SolAgentTool<T> {
    pub fn new(tool: T) -> Self {
        Self { tool }
    }

    pub fn get_tool(&self) -> &T {
        &self.tool
    }

    pub fn registe(&mut self, tool: T) {
        self.tool = tool;
    }
}

// Creates a tool from the metadata and wallet client
// fn main() {
//     let tools: SolAgentTool<T> = vec![];
//     let wallet = SolAgentWallet();

//     let solagent = SolAgent::new(wallet);
//     let result = solagent.prompt(
//         SolAgentModel::OpenAI("gpt-4".to_string()),
//         tools: tools,
//         "get balance of account: Sola11111111111111111111111111",
//     );
// }
