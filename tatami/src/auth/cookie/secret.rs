use rand::prelude::StdRng;
use rand::Rng;
use tower_cookies::Key;

// To be able to decrypt our private cookies after a server reboot,
// we must be able to attain the same key. Thus, we use a random
// string from environment variables to seed the key generation.

pub fn cookie_secret_from_seed(seed: &str) -> Key {
    let namespaced_seed = format!("{}.cookie", seed);
    let rng: StdRng = rand_seeder::Seeder::from(namespaced_seed).make_rng(); // probably ChaCha12

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
