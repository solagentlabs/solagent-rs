use tracing::Subscriber;

// Logger configuration structure
pub struct Logger {
    subscriber: Box<dyn Subscriber + Send + Sync>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            subscriber: Box::new(tracing::subscriber::default()),
        }
    }
}