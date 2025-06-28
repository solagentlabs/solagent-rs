use tokio::sync::RwLock;

// Short-term memory structure
pub struct ShortTermMemory {
    pub context: RwLock<std::collections::HashMap<String, String>>,
}

impl ShortTermMemory {
    pub fn new() -> Self {
        ShortTermMemory {
            context: RwLock::new(std::collections::HashMap::new()),
        }
    }
}