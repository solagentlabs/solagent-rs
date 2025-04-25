use anyhow::Result;
use rig::agent::Agent;
use rig::completion::Prompt;
use rig::providers::gemini::completion::CompletionModel;
use rig::providers::{anthropic, cohere, gemini, openai, perplexity};
use rig::tool::Tool;

use crate::tool::SolAgentTool;

/// Represents the model types supported by SolAgentCompletionModel.
/// The `String` field specifies the model name, such as "gpt-4" or "gemini-1.0".
#[non_exhaustive]
pub enum SolAgentModel {
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
    pub fn create_agent<T: Tool + 'static + Clone>(
        &self,
        tools: Vec<SolAgentTool<T>>,
    ) -> Result<SolAgentCompletionModel> {
        match self {
            SolAgentModel::OpenAI(model_name) => {
                let client = openai::Client::from_env();
                let tools: Vec<&T> = tools.iter().map(|tool| tool.get_tool()).collect();

                let mut agent_builder = client.agent(model_name);

                for tool in tools {
                    agent_builder = agent_builder.tool(tool.clone());
                }

                let agent = agent_builder.build();
                Ok(SolAgentCompletionModel::OpenAI(agent))
            }
            SolAgentModel::Gemini(model_name) => {
                let client = gemini::Client::from_env();
                let tools: Vec<&T> = tools.iter().map(|tool| tool.get_tool()).collect();

                let mut agent_builder = client.agent(model_name);

                for tool in tools {
                    agent_builder = agent_builder.tool(tool.clone());
                }

                let agent = agent_builder.build();

                Ok(SolAgentCompletionModel::Gemini(agent))
            }
            SolAgentModel::Anthropic(model_name) => {
                let client = anthropic::Client::from_env();
                let tools: Vec<&T> = tools.iter().map(|tool| tool.get_tool()).collect();

                let mut agent_builder = client.agent(model_name);

                for tool in tools {
                    agent_builder = agent_builder.tool(tool.clone());
                }

                let agent = agent_builder.build();

                Ok(SolAgentCompletionModel::Anthropic(agent))
            }
            SolAgentModel::Cohere(model_name) => {
                let client = cohere::Client::from_env();
                let tools: Vec<&T> = tools.iter().map(|tool| tool.get_tool()).collect();

                let mut agent_builder = client.agent(model_name);

                for tool in tools {
                    agent_builder = agent_builder.tool(tool.clone());
                }

                let agent = agent_builder.build();

                Ok(SolAgentCompletionModel::Cohere(agent))
            }
            SolAgentModel::Perplexity(model_name) => {
                let client = perplexity::Client::from_env();
                let tools: Vec<&T> = tools.iter().map(|tool| tool.get_tool()).collect();

                let mut agent_builder = client.agent(model_name);

                for tool in tools {
                    agent_builder = agent_builder.tool(tool.clone());
                }

                let agent = agent_builder.build();

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
