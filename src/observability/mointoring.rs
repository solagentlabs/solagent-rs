// Monitoring configuration structure
pub struct Monitoring {
    metrics_endpoint: String,
}

impl Monitoring {
    pub fn new() -> Self {
        Monitoring {
            metrics_endpoint: String::from("http://127.0.0.1:9090"),
        }
    }
}