/// SolAgent Framework: A Rust AI agent for Solana, integrating blockchain tools and LLMs.
/// 
/// # Example
/// ```
/// use solagent::{SolAgent, SolAgentConfig};
/// use serde_json::json;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = SolAgentConfig {
///         name: "Solana Agent".to_string(),
///         instructions: "Execute Solana tasks.".to_string(),
///         model: "grok3".to_string(),
///         tools: vec!["stake_sol".to_string()],
///     };
///     let solagent = SolAgent::new(config)?;
///     let input = json!({ "amount": 10.0, "validator": "validator_pubkey" });
///     let result = solagent.execute_task("stake_sol", input).await?;
///     println!("Result: {}", result);
///     Ok(())
/// }
/// ```
/// 

use std::sync::Arc;

use agent_controller::AgentController;
use llm_integration::{Grok3Provider, LLMClient, LLMProvider};
use memory_system::{LongTermMemory, ShortTermMemory};
use observability::{Logger, Monitoring};
use security_permission::{ABAC, RBAC};
use solana_integration::{IndexerClient, SolanaRPC};
use tool_system::ToolRegistry;
use user_interface::{ApiServer, CliConfig, WebConfig};
use workflow_engine::WorkflowEngine;

// Configuration for SolAgent initialization
pub struct SolAgentConfig {
    pub name: String,
    pub instructions: String,
    pub model: String,
    pub tools: Vec<String>,
}

// Main SolAgent struct, orchestrating all framework components
pub struct SolAgent {
    pub tool_registry: Arc<ToolRegistry>,
    pub memory_short: Arc<ShortTermMemory>,
    pub memory_long: Arc<LongTermMemory>,
    pub controller: Arc<AgentController>,
    pub workflow: Arc<WorkflowEngine>,
    pub rpc: Arc<SolanaRPC>,
    pub indexer: Arc<IndexerClient>,
    pub rbac: Arc<RBAC>,
    pub abac: Arc<ABAC>,
    pub logger: Arc<Logger>,
    pub monitoring: Arc<Monitoring>,
    pub cli: Arc<CliConfig>,
    pub api: Arc<ApiServer>,
    pub web: Arc<WebConfig>,
    pub llm_client: Arc<LLMClient>,
    pub config: SolAgentConfig,
}

impl SolAgent {
    // Creates a new SolAgent instance with provided configuration
    pub fn new(config: SolAgentConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let rpc_url = std::env::var("SOLANA_RPC_URL").unwrap_or("https://api.devnet.solana.com".to_string());
        let rpc = Arc::new(SolanaRPC::new());
        let indexer = Arc::new(IndexerClient::new());
        let tool_registry = Arc::new(ToolRegistry::new());
        
        // Register specified tools
        tool_registry.register_tools(&config.tools, &rpc_url).await;

        let memory_short = Arc::new(ShortTermMemory::new());
        let memory_long = Arc::new(LongTermMemory::new());
        let rbac = Arc::new(RBAC::new());
        let abac = Arc::new(ABAC::new());
        let logger = Arc::new(Logger::new());
        let monitoring = Arc::new(Monitoring::new());
        let llm_client = Arc::new(LLMClient::new());

        // Initialize LLM provider based on config.model
        match config.model.as_str() {
            "grok3" => {
                let api_key = std::env::var("GROK3_API_KEY")?;
                llm_client.register_provider("grok3", Arc::new(Grok3Provider::new(api_key))).await;
            }
            "gemini" => {
                let api_key = std::env::var("GEMINI_API_KEY")?;
                llm_client.register_provider("gemini", Arc::new(GeminiProvider::new(api_key))).await;
            }
            "openai" => {
                let api_key = std::env::var("OPENAI_API_KEY")?;
                llm_client.register_provider("openai", Arc::new(OpenAIProvider::new(api_key))).await;
            }
            _ => return Err("Unsupported LLM model".into()),
        }

        let controller = Arc::new(AgentController::new(
            tool_registry.clone(),
            memory_short.clone(),
            memory_long.clone(),
            llm_client.clone(),
        ));
        let workflow = Arc::new(WorkflowEngine::new());
        let cli = Arc::new(CliConfig::new());
        let api = Arc::new(ApiServer::new());
        let web = Arc::new(WebConfig::new());

        Ok(SolAgent {
            tool_registry,
            memory_short,
            memory_long,
            controller,
            workflow,
            rpc,
            indexer,
            rbac,
            abac,
            logger,
            monitoring,
            cli,
            api,
            web,
            llm_client,
            config,
        })
    }

    // Executes a task using the AgentController
    pub async fn execute_task(
        &self,
        task: &str,
        input: serde_json::Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.controller.execute_task(task, input).await
    }
}