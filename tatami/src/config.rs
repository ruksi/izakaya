pub struct Config {
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        // populate environment variables from `.env` file without overriding
        dotenvy::dotenv().ok();

        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

        Self { port }
    }
}