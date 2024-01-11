pub struct Config {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub cache_url: String,
    pub frontend_url: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string());

        let rust_log = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "tatami=debug".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let cache_url = std::env::var("CACHE_URL")
            .expect("CACHE_URL must be set");

        let frontend_url = std::env::var("FRONTEND_URL")
            .ok()
            .map(|url| url.trim_end_matches('/').to_string());

        Self { port, rust_log, database_url, cache_url, frontend_url }
    }

    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
