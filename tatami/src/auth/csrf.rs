use base64::prelude::*;
use hmac::{Hmac, Mac};
use rand::{distributions::Alphanumeric, Rng};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn create_csrf_token(csrf_secret: &str, session_id: Option<uuid::Uuid>) -> String {
    // `ThreadRng` is cryptographically secure "as far as anyone knows"
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    create_csrf_token_with_salt(csrf_secret, session_id, &salt)
}

pub fn is_valid_csrf_token(
    csrf_secret: &str,
    session_id: Option<uuid::Uuid>,
    csrf_token_in: &str,
) -> bool {
    let session_id = session_id.unwrap_or(uuid::Uuid::nil());

    let mut token_parts_in = csrf_token_in.split('.');
    let Some(hash_base64_in) = token_parts_in.next() else {
        return false; // invalid token string
    };
    let Some(salt_in) = token_parts_in.next() else {
        return false; // invalid token string
    };

    let Ok(hash_bytes_in) = BASE64_URL_SAFE_NO_PAD.decode(hash_base64_in.as_bytes()) else {
        return false; // invalid base64
    };

    let hash_output = HmacSha256::new_from_slice(csrf_secret.as_bytes())
        .expect("HMAC can take key of any size")
        .chain_update(session_id.as_bytes())
        .chain_update(salt_in.as_bytes());

    hash_output.verify_slice(&hash_bytes_in[..]).is_ok()
}

fn create_csrf_token_with_salt(
    csrf_secret: &str,
    session_id: Option<uuid::Uuid>,
    salt: &str,
) -> String {
    // not the best way to go about it, but it's a start as anonymous users have no session
    // well, at least I don't need to worry about making sure sessions are reconstructed after login 😅
    let session_id = session_id.unwrap_or(uuid::Uuid::nil());

    let hash_output = HmacSha256::new_from_slice(csrf_secret.as_bytes())
        .expect("HMAC can take key of any size")
        .chain_update(session_id.as_bytes())
        .chain_update(salt.as_bytes())
        .finalize();

    let hash_bytes = hash_output.into_bytes();
    let hash_base64 = BASE64_URL_SAFE_NO_PAD.encode(hash_bytes);
    let csrf_token = format!("{}.{}", hash_base64, salt);
    csrf_token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn basics_work() {
        let csrf_secret = "yes, very secret";
        let me = uuid::Uuid::new_v4();
        let other = uuid::Uuid::new_v4();
        let token = create_csrf_token(csrf_secret, Some(me));
        assert!(is_valid_csrf_token(csrf_secret, Some(me), &token));
        assert!(!is_valid_csrf_token(csrf_secret, Some(other), &token));
        assert!(!is_valid_csrf_token(csrf_secret, None, &token));
    }

    #[tokio::test]
    async fn internals_work() {
        let cheat = "s_R0xw28in0Bhxury9LCCCp8EXLi8GHh29jRVyman5w.random-public-salt";
        let wrong = "s_R0xw28in0Bhxury9LCCCp8EXLi8GHh29jRVyman5w.random-public-pepper";

        let csrf_secret = "my secret key only the server knows";
        let salt = "random-public-salt";
        let token = create_csrf_token_with_salt(csrf_secret, None, salt);
        assert_eq!(token, cheat);

        assert!(is_valid_csrf_token(csrf_secret, None, cheat));
        assert!(!is_valid_csrf_token(csrf_secret, None, wrong));
    }

    #[tokio::test]
    async fn salt_generation_works() {
        let csrf_secret = "yes, very secret";

        let token1 = create_csrf_token(csrf_secret, None);
        let token2 = create_csrf_token(csrf_secret, None);
        assert_ne!(token1, token2);

        let uuid = uuid::Uuid::new_v4();
        let token1 = create_csrf_token(csrf_secret, Some(uuid));
        let token2 = create_csrf_token(csrf_secret, Some(uuid));
        assert_ne!(token1, token2);
    }

    #[tokio::test]
    async fn bad_tokens_dont_panic() {
        let cases = ["", ".", "...", "not!a*base64.why-so-salty"];
        for case in cases.into_iter() {
            assert!(!is_valid_csrf_token("secret", None, case));
        }
    }
}
