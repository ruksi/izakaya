pub struct Config {
    pub port: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        // populate environment variables from `.env` file without overriding
        dotenvy::dotenv().ok();

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self { port, database_url }
    }
}