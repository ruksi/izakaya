pub struct Config {
    pub port: String,
    pub rust_log: String,
    pub database_url: String,
    pub cache_url: String,
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

        Self { port, rust_log, cache_url, database_url }
    }

    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
