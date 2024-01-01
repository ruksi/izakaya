pub struct Config {
    pub port: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self { port, database_url }
    }

    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
