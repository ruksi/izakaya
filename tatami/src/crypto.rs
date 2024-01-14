use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::StatusCode;

use crate::error;

pub async fn hash_password(password: String) -> Result<String, (StatusCode, String)> {
    // TODO: the error shouldn't be something to be exposed...
    tokio::task::spawn_blocking(move || -> Result<String, (StatusCode, String)> {
        let salt = SaltString::generate(rand::thread_rng());
        let hash = hasher()
            .hash_password(password.as_ref(), &salt)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .to_string();
        Ok(hash)
    })
    .await
    .map_err(error::internal)?
}

pub async fn verify_password(
    hash_string: String,
    password: String,
) -> Result<(), (StatusCode, String)> {
    // TODO: the error shouldn't be something to be exposed...
    let hash = PasswordHash::new(&hash_string)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    hasher()
        .verify_password(password.as_ref(), &hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(())
}

fn hasher<'a>() -> Argon2<'a> {
    Argon2::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn password_hashing_works() -> Result<(), (StatusCode, String)> {
        let hash = hash_password("p4ssw0rd".into()).await?;
        assert!(hash.contains("argon2id")); // hash string contains the algorithm name
        assert!(verify_password(hash.clone(), "p4ssw0rd".into())
            .await
            .is_ok());
        assert!(verify_password(hash.clone(), "p4ssw0rdd".into())
            .await
            .is_err());
        Ok(())
    }
}
