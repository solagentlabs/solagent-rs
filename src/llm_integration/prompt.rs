// Prompt configuration structure
pub struct PromptConfig {
    template: String,
}

impl PromptConfig {
    pub fn new() -> Self {
        PromptConfig {
            template: String::new(),
        }
    }
}