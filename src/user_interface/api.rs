use actix_web::App;

// API server configuration
pub struct ApiServer {
    port: u16,
    app: App<()>,
}

impl ApiServer {
    pub fn new() -> Self {
        ApiServer {
            port: 8080,
            app: App::new(),
        }
    }

    // Starts the API server
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}