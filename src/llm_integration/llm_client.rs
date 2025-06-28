use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

use crate::tool_system::ToolMetadata;

// LLM provider trait for pluggable LLM APIs
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn call(
        &self,
        prompt: &str,
        tools: Vec<ToolMetadata>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
}

// Grok 3 provider
pub struct Grok3Provider {
    client: Client,
    api_key: String,
    endpoint: String,
}

impl Grok3Provider {
    pub fn new(api_key: String) -> Self {
        Grok3Provider {
            client: Client::new(),
            api_key,
            endpoint: "https://api.x.ai/v1".to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for Grok3Provider {
    async fn call(
        &self,
        prompt: &str,
        tools: Vec<ToolMetadata>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&format!("{}/chat/completions", self.endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "grok-3",
                "messages": [{"role": "user", "content": prompt}],
                "tools": tools.iter().map(|t| &t.schema).collect::<Vec<_>>(),
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

// Gemini provider
pub struct GeminiProvider {
    client: Client,
    api_key: String,
    endpoint: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        GeminiProvider {
            client: Client::new(),
            api_key,
            endpoint: "https://generativelanguage.googleapis.com/v1".to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for GeminiProvider {
    async fn call(
        &self,
        prompt: &str,
        tools: Vec<ToolMetadata>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&format!("{}/models/gemini-1.5-pro:generateContent", self.endpoint))
            .query(&[("key", &self.api_key)])
            .json(&serde_json::json!({
                "contents": [{"parts": [{"text": prompt}]}],
                "tools": [{"functionDeclarations": tools.iter().map(|t| &t.schema).collect::<Vec<_>>()}],
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

// OpenAI provider
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    endpoint: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        OpenAIProvider {
            client: Client::new(),
            api_key,
            endpoint: "https://api.openai.com/v1".to_string(),
        }
    }
}

#[async_trait]
impl LLMProvider for OpenAIProvider {
    async fn call(
        &self,
        prompt: &str,
        tools: Vec<ToolMetadata>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&format!("{}/chat/completions", self.endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "gpt-4o",
                "messages": [{"role": "user", "content": prompt}],
                "tools": tools.iter().map(|t| &t.schema).collect::<Vec<_>>(),
            }))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

// LLM client managing multiple providers
pub struct LLMClient {
    providers: tokio::sync::RwLock<std::collections::HashMap<String, Arc<dyn LLMProvider>>>,
}

impl LLMClient {
    pub fn new() -> Self {
        LLMClient {
            providers: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    // Registers an LLM provider
    pub async fn register_provider(&self, name: &str, provider: Arc<dyn LLMProvider>) {
        self.providers.write().await.insert(name.to_string(), provider);
    }

    // Calls an LLM provider
    pub async fn call(
        &self,
        provider_name: &str,
        prompt: &str,
        tools: Vec<ToolMetadata>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let provider = self
            .providers
            .read()
            .await
            .get(provider_name)
            .ok_or("Provider not found")?
            .clone();
        provider.call(prompt, tools).await
    }
}