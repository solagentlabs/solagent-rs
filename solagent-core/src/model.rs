/// Represents the model types supported by SolAgent.
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