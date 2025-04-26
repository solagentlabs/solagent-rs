pub mod model;
pub mod tool;

use {
    anyhow::Result, model::SolAgentModel, rig::tool::Tool, solagent_wallet_solana::SolAgentWallet, tool::SolAgentTool,
    solana_client::rpc_client::RpcClient,
};

pub struct SolAgent {
    pub wallet: SolAgentWallet,
    pub rpc_client: RpcClient,
}

impl SolAgent {
    /// Creates a new `SolAgent` with the given wallet.
    pub fn new(wallet: SolAgentWallet) -> Self {
        let rpc_client = RpcClient::new(&wallet.rpc_url);
        Self { wallet, rpc_client }
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
    pub async fn prompt<T: Tool + Clone + 'static>(
        &self,
        model: SolAgentModel,
        tools: Vec<SolAgentTool<T>>,
        prompt: &str,
    ) -> Result<String> {
        // Dynamically create the agent based on the model
        let agent = model.create_agent(tools)?;

        // Execute the prompt using the dynamically created agent
        let response = agent.prompt(prompt).await?;

        Ok(response)
    }
}
