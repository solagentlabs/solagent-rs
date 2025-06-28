use async_trait::async_trait;

// Memory backend trait for pluggable storage
#[async_trait]
pub trait MemoryBackend {
    async fn store(&self, key: &str, value: &str);
    async fn retrieve(&self, key: &str) -> Option<String>;
}

// Long-term memory structure
pub struct LongTermMemory {
    backend: Box<dyn MemoryBackend + Send + Sync>,
}

impl LongTermMemory {
    pub fn new() -> Self {
        LongTermMemory {
            backend: Box::new(SqliteBackend::new()),
        }
    }
}

// SQLite backend for long-term memory
pub struct SqliteBackend {
    db: rusqlite::Connection,
}

impl SqliteBackend {
    pub fn new() -> Self {
        SqliteBackend {
            db: rusqlite::Connection::open("solagent.db").unwrap(),
        }
    }
}