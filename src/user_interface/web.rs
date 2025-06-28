// Web configuration structure
pub struct WebConfig {
    port: u16,
}

impl WebConfig {
    pub fn new() -> Self {
        WebConfig { port: 3000 }
    }
}