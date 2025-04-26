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