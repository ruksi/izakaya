use crate::auth::cookie;
use crate::auth::csrf::create_csrf_token;
pub use domain::cookie_domain_from;
use rand::rngs::StdRng;
use rand::Rng;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Key};

mod domain;

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

// To be able to decrypt our private cookies after a server reboot, we must be able to attain the same key.
// Thus, we use a random string from environment variables to seed the key generation.
pub fn cookie_secret_from_seed(seed: &str) -> Key {
    let namespaced_seed = format!("{}.cookie", seed);
    let rng: StdRng = rand_seeder::Seeder::from(namespaced_seed).make_rng(); // probably ChaCha12

    // `axum-extra` cookie encryption key wants at least 64 bytes, so let's give it exactly that
    let seeded_random_bytes: [u8; 64] = rng
        .sample_iter(rand::distributions::Standard)
        .take(64)
        .collect::<Vec<u8>>()
        .try_into()
        .expect("failed to create cookie key from secret key (convert Vec<u8> to [u8; 64])");

    Key::from(&seeded_random_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_key_generation_works() -> Result<(), String> {
        let key_lol_1 = cookie_secret_from_seed("lol");
        let key_lol_2 = cookie_secret_from_seed("lol");
        assert_eq!(key_lol_1, key_lol_2);

        // borderline acceptable, I guess
        // would hate to fail now; previous validations should have caught this
        let key_empty_1 = cookie_secret_from_seed("");
        let key_empty_2 = cookie_secret_from_seed("");
        assert_eq!(key_empty_1, key_empty_2);

        // sure, why not
        let key_wtf_1 = cookie_secret_from_seed("🔐🙈");
        let key_wtf_2 = cookie_secret_from_seed("🔐🙈");
        assert_eq!(key_wtf_1, key_wtf_2);

        // something that at least looks the part, not really used anywhere
        let seed = "yCIAKtN9qRpP1pky46vmV3ycbBC8zwKxAFkFmJH7UgZbRh41qkMIawCuC12Afs4g";
        let key_good_1 = cookie_secret_from_seed(seed);
        let key_good_2 = cookie_secret_from_seed(seed);
        assert_eq!(key_good_1, key_good_2);

        // and finally, check that all key pairs are different
        assert_ne!(key_lol_1, key_empty_1);
        assert_ne!(key_lol_1, key_wtf_1);
        assert_ne!(key_lol_1, key_good_1);
        assert_ne!(key_empty_1, key_wtf_1);
        assert_ne!(key_empty_1, key_good_1);
        assert_ne!(key_wtf_1, key_good_1);

        Ok(())
    }
}
