pub struct Config {
    pub port: String,
    pub rust_log: String,
    pub database_url: String, // aka. PostgreSQL
    pub cache_url: String,    // aka. Redis
    pub secret_key: String, // a generic seed (64+ character string) used for hashes, salts, and the like
    pub frontend_urls: Option<Vec<String>>,
}

impl Config {
    pub fn load() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "tatami=debug".to_string());

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let cache_url = std::env::var("CACHE_URL").expect("CACHE_URL must be set");

        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        let frontend_urls = std::env::var("FRONTEND_URL").ok().map(split_urls);

        Self {
            port,
            rust_log,
            database_url,
            cache_url,
            secret_key,
            frontend_urls,
        }
    }

    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

pub fn split_urls<T: Into<String>>(text: T) -> Vec<String> {
    let text = text.into();
    text.split(',')
        .map(|url| url.trim().trim_end_matches('/').to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontend_url_parsing_works() {
        let cases = [
            ("http://localhost:3000", vec!["http://localhost:3000"]),
            ("http://localhost:3000/", vec!["http://localhost:3000"]),
            ("https://a.com/,b.com", vec!["https://a.com", "b.com"]),
            ("/a.com ,  b.com//", vec!["/a.com", "b.com"]),
        ];
        for (case, expected) in cases.into_iter() {
            let result = split_urls(case);
            assert_eq!(result, expected);
        }
    }
}
