pub mod model;
pub mod tool;
pub mod config;

use {
    anyhow::Result, model::SolAgentModel, rig::tool::{Tool, ToolSet}, solagent_wallet_solana::SolAgentWallet,
    solana_client::rpc_client::RpcClient, crate::config::SolAgentConfig,
};
pub use solana_client;
pub use tool::SolAgentTool;

pub struct SolAgent {
    pub wallet: SolAgentWallet,
    pub config: Option<SolAgentConfig>,
    pub rpc_client: RpcClient,
}

impl SolAgent {
    /// Creates a new `SolAgent` with the given wallet.
    pub fn new(wallet: SolAgentWallet, config: Option<SolAgentConfig>) -> Self {
        let rpc_client = RpcClient::new(&wallet.rpc_url);
        Self { config, wallet, rpc_client }
    }

    /// Dynamically creates an `Agent` based on the provided model and executes the prompt.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to use for creating the agent (e.g., OpenAI, Gemini).
    /// * `tools` - A list of tools to be used by the agent.
    /// * `prompt` - The input prompt to process.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The result of processing the prompt.
    pub async fn prompt(
        &self,
        model: SolAgentModel,
        tools: SolAgentTool,
        prompt: &str,
    ) -> Result<String> {
        // Dynamically create the agent based on the model
        let agent = model.create_agent(tools)?;

        // Execute the prompt using the dynamically created agent
        let response = agent.prompt(prompt).await?;

        Ok(response)
    }
}
