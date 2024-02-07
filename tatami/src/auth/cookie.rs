use axum::extract::FromRef;
use axum_extra::extract::cookie::{Cookie, Key, SameSite};
use rand::rngs::StdRng;
use rand::Rng;
use url::Host;

use crate::state::AppState;

pub const ACCESS_TOKEN: &str = "Access-Token";

// tell `axum-extra` private cookies where to get their encryption key
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_secret.clone()
    }
}

// 🍫 + 🥣 + 🌡️ = 🍪
pub fn bake<'a>(name: &'static str, value: String, max_age: time::Duration) -> Cookie<'a> {
    let mut builder = Cookie::build((name, value))
        // these flags are for the _browser_ to enforce
        .same_site(SameSite::Strict) // use inside the same registrable domain, excluding https://publicsuffix.org/
        .path("/") // use on all paths (not just the current one)
        .http_only(true) // forbid reading cookies with JavaScript
        .secure(true) // forbid sending the cookie over plain HTTP
        .max_age(max_age); // automatically delete the cookie after this duration

    // TODO: feeling too lazy passing the config down here for now 🦥
    // as our API server and frontend are on different subdomains, we want to assign cookie
    // domain to the registrable domain (e.g. "example.com") instead of the current subdomain
    // so we can share the cookie.
    // note that this _does_ make the cookie insecure on shared domains like "onrender.com",
    // domain-scoped cookies are only secure if you control all subdomains of the domain.
    let frontend_urls = std::env::var("FRONTEND_URL")
        .ok()
        .map(crate::config::split_urls);
    if let Some(frontend_urls) = frontend_urls {
        let frontend_url = frontend_urls.first();
        if let Some(frontend_url) = frontend_url {
            // TODO: this wrongly assumes that all frontend URLs are on the same domain,
            //       fix with panic in config parse
            if let Ok(Some(domain)) = cookie_domain_from(frontend_url) {
                builder = builder.domain(domain);
            }
        }
    }

    builder.build()
}

// To be able to decrypt our private cookies after a server reboot, we must be able to attain the same key.
// Thus, we use a random string from environment variables to seed the key generation.
pub fn cookie_secret_from_seed(seed: String) -> Key {
    let rng: StdRng = rand_seeder::Seeder::from(seed).make_rng(); // probably ChaCha12

    // `axum-extra` cookie encryption key wants at least 64 bytes, so let's give it exactly that
    let seeded_random_bytes: [u8; 64] = rng
        .sample_iter(rand::distributions::Standard)
        .take(64)
        .collect::<Vec<u8>>()
        .try_into()
        .expect("failed to create cookie key from secret key (convert Vec<u8> to [u8; 64])");

    Key::from(&seeded_random_bytes)
}

fn cookie_domain_from(text: &str) -> Result<Option<String>, url::ParseError> {
    let url = url::Url::parse(text)?;
    let domain = match url.host() {
        Some(Host::Domain(domain)) => domain.to_string(),
        Some(Host::Ipv4(ip)) => return Ok(Some(ip.to_string())),
        Some(Host::Ipv6(ip)) => return Ok(Some(ip.to_string())),
        None => return Ok(None),
    };
    let segments: Vec<&str> = domain
        .split('.')
        .filter(|segment| !segment.is_empty())
        .collect();

    if segments.len() <= 2 {
        // probably "localhost" or already a registrable domain
        return Ok(segments.join(".").into());
    }

    // this doesn't properly handle domains like "co.uk" or "com.au", but it's good enough for me
    let cookie_domain = segments
        .iter()
        .rev()
        .take(2)
        .rev()
        .cloned()
        .collect::<Vec<_>>()
        .join(".");

    Ok(Some(cookie_domain))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_domain_generation_works() -> Result<(), String> {
        let cases = [
            ("http://127.0.0.1", "127.0.0.1"),
            ("http://localhost:5173", "localhost"),
            ("https://example.com", "example.com"),
            ("https://sub.example.com/", "example.com"),
        ];
        for (url, expected) in cases.iter() {
            let result = cookie_domain_from(url).unwrap().unwrap();
            assert_eq!(result, *expected);
        }
        Ok(())
    }

    #[test]
    fn cookie_key_generation_works() -> Result<(), String> {
        let key_lol_1 = cookie_secret_from_seed("lol".into());
        let key_lol_2 = cookie_secret_from_seed("lol".into());
        assert_eq!(key_lol_1, key_lol_2);

        // borderline acceptable, I guess
        // would hate to fail now; previous validations should have caught this
        let key_empty_1 = cookie_secret_from_seed("".into());
        let key_empty_2 = cookie_secret_from_seed("".into());
        assert_eq!(key_empty_1, key_empty_2);

        // sure, why not
        let key_wtf_1 = cookie_secret_from_seed("🔐🙈".into());
        let key_wtf_2 = cookie_secret_from_seed("🔐🙈".into());
        assert_eq!(key_wtf_1, key_wtf_2);

        // something that at least looks the part, not really used anywhere
        let seed = "yCIAKtN9qRpP1pky46vmV3ycbBC8zwKxAFkFmJH7UgZbRh41qkMIawCuC12Afs4g";
        let key_good_1 = cookie_secret_from_seed(seed.into());
        let key_good_2 = cookie_secret_from_seed(seed.into());
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
