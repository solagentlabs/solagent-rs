use anyhow::Result;
use rig::agent::Agent;
use rig::completion::Prompt;
use rig::providers::gemini::completion::CompletionModel;
use rig::providers::{ollama, anthropic, cohere, gemini, openai, perplexity};
use rig::tool::ToolSet;

use crate::tool::SolAgentTool;

/// Represents the model types supported by SolAgentCompletionModel.
/// The `String` field specifies the model name, such as "gpt-4" or "gemini-1.0".
#[non_exhaustive]
pub enum SolAgentModel {
    /// Ollama model, e.g., "llama3.2".
    Ollama(String),
    /// OpenAI model, e.g., "gpt-4", "gpt-3.5-turbo".
    OpenAI(String),
    /// Gemini model, e.g., "gemini-1.0".
    Gemini(String),
    /// Anthropic model, e.g., "claude-3".
    Anthropic(String),
    /// Cohere model, e.g., "cohere-1.0".
    Cohere(String),
    /// Perplexity model, e.g., "perplexity-1.0".
    Perplexity(String),
}

/// Represents the different types of agents that can be created.
#[non_exhaustive]
pub enum SolAgentCompletionModel {
    Ollama(Agent<ollama::CompletionModel>),
    OpenAI(Agent<openai::CompletionModel>),
    Gemini(Agent<CompletionModel>),
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    Cohere(Agent<cohere::CompletionModel>),
    Perplexity(Agent<perplexity::CompletionModel>),
}

impl SolAgentModel {
    /// Dynamically creates an `Agent` based on the model type.
    ///
    /// # Arguments
    ///
    /// * `tools` - A list of tools to be used by the agent.
    ///
    /// # Returns
    ///
    /// * `Result<SolAgentCompletionModel>` - The dynamically created agent wrapped in the `SolAgentCompletionModel` enum.
    pub fn create_agent(
        &self,
        tools: SolAgentTool,
    ) -> Result<SolAgentCompletionModel> {
        match self {
            SolAgentModel::Ollama(model_name) => {
                let client = ollama::Client::new();
                let mut agent = client.agent(model_name).tool_names(tools.tool_names).build();
                agent.tools = tools.toolset;
                Ok(SolAgentCompletionModel::Ollama(agent))
            },
            SolAgentModel::OpenAI(model_name) => {
                let client = openai::Client::from_env();
                let mut agent = client.agent(model_name).tool_names(tools.tool_names).build();
                agent.tools = tools.toolset;
                Ok(SolAgentCompletionModel::OpenAI(agent))
            }
            SolAgentModel::Gemini(model_name) => {
                let client = gemini::Client::from_env();
                let mut agent = client.agent(model_name).tool_names(tools.tool_names).build();
                agent.tools = tools.toolset;
                Ok(SolAgentCompletionModel::Gemini(agent))
            }
            SolAgentModel::Anthropic(model_name) => {
                let client = anthropic::Client::from_env();
                let mut agent = client.agent(model_name).tool_names(tools.tool_names).build();
                agent.tools = tools.toolset;
                Ok(SolAgentCompletionModel::Anthropic(agent))
            }
            SolAgentModel::Cohere(model_name) => {
                let client = cohere::Client::from_env();
                let mut agent = client.agent(model_name).tool_names(tools.tool_names).build();
                agent.tools = tools.toolset;
                Ok(SolAgentCompletionModel::Cohere(agent))
            }
            SolAgentModel::Perplexity(model_name) => {
                let client = perplexity::Client::from_env();
                let mut agent = client.agent(model_name).tool_names(tools.tool_names).build();
                agent.tools = tools.toolset;
                Ok(SolAgentCompletionModel::Perplexity(agent))
            }
        }
    }
}

impl SolAgentCompletionModel {
    /// Executes the given prompt using the appropriate agent.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The input prompt to process.
    ///
    /// # Returns
    ///
    /// * `Result<String>` - The result of processing the prompt.
    pub async fn prompt(&self, prompt: &str) -> Result<String> {
        match self {
            SolAgentCompletionModel::Ollama(agent) => {
                println!("contains > {}", agent.tools.contains("get_tps"));
                let response = agent.prompt(prompt).await?;
                Ok(response)
            },
            SolAgentCompletionModel::OpenAI(agent) => {
                let response = agent.prompt(prompt).await?;
                Ok(response)
            }
            SolAgentCompletionModel::Gemini(agent) => {
                let response = agent.prompt(prompt).await?;
                Ok(response)
            }
            SolAgentCompletionModel::Anthropic(agent) => {
                let response = agent.prompt(prompt).await?;
                Ok(response)
            }
            SolAgentCompletionModel::Cohere(agent) => {
                let response = agent.prompt(prompt).await?;
                Ok(response)
            }
            SolAgentCompletionModel::Perplexity(agent) => {
                let response = agent.prompt(prompt).await?;
                Ok(response)
            }
        }
    }
}
