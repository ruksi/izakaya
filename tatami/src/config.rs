use crate::auth::cookie::{cookie_domain_from, cookie_secret_from_seed};

const DEFAULT_PORT: &str = "8080";
const DEFAULT_RUST_LOG: &str = "tatami=debug";

// Config contains all the global, unchanging settings for the application.

#[rustfmt::skip]
#[derive(Clone)]
pub struct Config {
    pub port: String,
    pub rust_log: String,
    pub database_url: String, // aka. PostgreSQL
    pub cache_url: String,    // aka. Redis
    
    pub secret_key: String,                 // used to _seed_ the actual secrets
    pub cookie_secret: tower_cookies::Key,  // used to encrypt "private" cookies
    pub csrf_secret: String,                // used to sign CSRF tokens
    
    pub csrf_enabled: bool,  // configurable for tests 🧪
    
    pub frontend_urls: Vec<String>,
    pub cookie_domain: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| DEFAULT_RUST_LOG.to_string());

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let cache_url = std::env::var("CACHE_URL").expect("CACHE_URL must be set");

        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let cookie_secret = cookie_secret_from_seed(&secret_key);
        let csrf_secret = crate::auth::csrf::csrf_secret_from_seed(&secret_key);

        let csrf_enabled = true;

        let frontend_urls = split_urls(std::env::var("FRONTEND_URL").unwrap_or_default());

        let mut cookie_domain = None;
        if frontend_urls.len() > 0 {
            cookie_domain = cookie_domain_from(&frontend_urls).unwrap_or_else(|_| {
                panic!("FRONTEND_URL contains invalid URLs: {:?}", frontend_urls)
            });
            if cookie_domain.is_none() {
                panic!(
                    "FRONTEND_URL URLs have no common domain suffix: {:?}",
                    frontend_urls
                );
            }
        }

        Self {
            port,
            rust_log,
            database_url,
            cache_url,
            secret_key,
            cookie_secret,
            csrf_secret,
            csrf_enabled,
            frontend_urls,
            cookie_domain,
        }
    }

    pub fn bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }

    #[cfg(test)]
    #[rustfmt::skip]
    pub fn new_for_tests() -> Self {
        let secret_key = "v3ry-s3cr3t-v3ry-s3cr3t-v3ry-s3cr3t-v3ry-s3cr3t-v3ry-s3cr3t-v3ry".to_string();
        let cookie_secret = cookie_secret_from_seed(&secret_key);
        let csrf_secret = crate::auth::csrf::csrf_secret_from_seed(&secret_key);
        let csrf_enabled = false;
        let frontend_urls = vec!["http://localhost:3000".to_string()];
        let cookie_domain = None;
        Self {
            port: DEFAULT_PORT.to_string(),
            rust_log: DEFAULT_RUST_LOG.to_string(),
            database_url: "postgres://yeah-this-wont-work".to_string(),
            cache_url: "redis://yeah-this-wont-work".to_string(),
            secret_key,
            cookie_secret,
            csrf_secret,
            csrf_enabled,
            frontend_urls,
            cookie_domain,
        }
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
