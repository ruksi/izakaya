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
        state.config.cookie_secret.clone()
    }
}

// 🍫 + 🥣 + 🌡️ = 🍪
pub fn bake<'a>(
    name: &'static str,
    value: String,
    domain: Option<String>,
    max_age: time::Duration,
) -> Cookie<'a> {
    let mut builder = Cookie::build((name, value))
        // these flags are for the _browser_ to enforce
        .same_site(SameSite::Strict) // use inside the same registrable domain, excluding https://publicsuffix.org/
        .path("/") // use on all paths (not just the current one)
        .http_only(true) // forbid reading cookies with JavaScript
        .secure(true) // forbid sending the cookie over plain HTTP
        .max_age(max_age); // automatically delete the cookie after this duration

    if let Some(domain) = domain {
        builder = builder.domain(domain);
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

pub fn cookie_domain_from(urls: &Vec<String>) -> Result<Option<String>, url::ParseError> {
    let urls = urls
        .iter()
        .map(|u| url::Url::parse(u))
        .collect::<Result<Vec<_>, _>>()?;

    let domains = urls
        .iter()
        .map(|url| match url.host() {
            Some(Host::Domain(domain)) => domain.to_string(),
            Some(Host::Ipv4(ip)) => ip.to_string(),
            Some(Host::Ipv6(ip)) => ip.to_string(),
            None => "".to_string(),
        })
        .collect::<Vec<_>>();

    let segmented = domains
        .iter()
        .map(|domain| domain.split('.').rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut common = segmented[0].clone();
    for segment in segmented.into_iter().skip(1) {
        common = common
            .into_iter()
            .zip(segment.into_iter())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();
    }

    let prefix = common
        .into_iter()
        .rev()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(".");

    match prefix.is_empty() {
        true => Ok(None),
        false => Ok(Some(prefix)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_domain_from_singles() -> Result<(), String> {
        let cases = [
            (vec!["http://127.0.0.1".into()], "127.0.0.1"),
            (vec!["http://localhost:5173".into()], "localhost"),
            (vec!["https://example.com".into()], "example.com"),
            (vec!["https://sub.example.com/".into()], "sub.example.com"),
        ];
        for (case, expected) in cases.iter() {
            let result = cookie_domain_from(case).unwrap().unwrap();
            assert_eq!(result, *expected);
        }
        Ok(())
    }

    #[test]
    fn cookie_domain_from_pairs() -> Result<(), String> {
        #[rustfmt::skip]
        let cases = [
            (vec!["http://127.0.0.1".into(), "http://localhost".into()], None),
            (vec!["http://localhost:5173".into(), "http://localhost:3000".into()], Some("localhost".into())),
            (vec!["https://alpha.example.com".into(), "https://beta.example.com".into()], Some("example.com".into())),
            (vec!["https://a.b.c.com".into(), "http://z.b.c.com".into()], Some("b.c.com".into())),
        ];
        for (case, expected) in cases.into_iter() {
            let result = cookie_domain_from(&case).unwrap();
            assert_eq!(result, expected);
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
