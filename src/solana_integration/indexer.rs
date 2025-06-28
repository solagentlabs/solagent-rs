// Indexer client structure
pub struct IndexerClient {
    endpoint: String,
    api_key: String,
}

impl IndexerClient {
    pub fn new() -> Self {
        IndexerClient {
            endpoint: String::from("https://api.helius.xyz"),
            api_key: String::new(),
        }
    }
}