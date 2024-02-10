use crate::auth::cookie;
use crate::auth::csrf::create_csrf_token;
use tower_cookies::cookie::SameSite;
use tower_cookies::Cookie;

pub use domain::cookie_domain_from;
pub use secret::cookie_secret_from_seed;

mod domain;
mod secret;

pub const ACCESS_TOKEN: &str = "Tatami-Access";
pub const CSRF_TOKEN: &str = "Tatami-CSRF";

// 🍫 + 🥣 + 🌡️ = 🍪

pub fn bake_csrf<'a>(config: &crate::config::Config, session_id: Option<uuid::Uuid>) -> Cookie<'a> {
    let csrf_token = create_csrf_token(&config.csrf_secret, session_id);
    let csrf_cookie = cookie::bake_for_frontend(
        CSRF_TOKEN,
        csrf_token,
        config.cookie_domain.clone(),
        time::Duration::days(14),
    );
    csrf_cookie
}

// Cookies that should only be read by the server.
pub fn bake_for_backend<'a>(
    name: &'static str,
    value: String,
    domain: Option<String>,
    max_age: time::Duration,
) -> Cookie<'a> {
    let mut cookie = Cookie::new(name, value);
    cookie.set_same_site(SameSite::Strict); // use inside the same top domain, except https://publicsuffix.org/
    cookie.set_path("/"); // use on all paths (not just the current one)
    cookie.set_http_only(true); // forbid reading cookies with JavaScript
    cookie.set_secure(true); // forbid sending the cookie over plain HTTP
    cookie.set_max_age(max_age); // automatically delete the cookie after this duration
    if let Some(domain) = domain {
        cookie.set_domain(domain);
    }
    cookie
}

pub fn remove_for_backend<'a>(name: &'static str, domain: Option<String>) -> Cookie<'a> {
    bake_for_backend(name, "".to_string(), domain, time::Duration::ZERO)
}

// Cookies that can be read by the browser JavaScript.
pub fn bake_for_frontend<'a>(
    name: &'static str,
    value: String,
    domain: Option<String>,
    max_age: time::Duration,
) -> Cookie<'a> {
    let mut cookie = bake_for_backend(name, value, domain, max_age);
    cookie.set_http_only(false); // allow reading cookies with JavaScript
    cookie
}
