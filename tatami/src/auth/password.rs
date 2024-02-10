use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::borrow::Cow;

use crate::prelude::*;

pub async fn hash_password(password: impl Into<Cow<'_, str>>) -> Result<String> {
    let password = password.into().into_owned();
    // CPU-intensive work should be done in a blocking task
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        let hash = hasher()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hash)
    })
    .await?
}

pub async fn verify_password<'a, 'b>(
    hash_string: impl Into<Cow<'a, str>>,
    password: impl Into<Cow<'b, str>>,
) -> Result<()> {
    let hash_string = hash_string.into().into_owned();
    let password = password.into().into_owned();
    // CPU-intensive work should be done in a blocking task
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(hash_string.as_ref())?;
        hasher().verify_password(password.as_bytes(), &hash)?;
        Ok(())
    })
    .await??;
    Ok(())
}

fn hasher<'a>() -> Argon2<'a> {
    Argon2::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn password_hashing_works() -> Result<()> {
        let hash = hash_password("p4ssw0rd").await?;
        assert!(hash.contains("argon2id")); // hash string contains the algorithm name
        assert!(verify_password(&hash, "p4ssw0rd").await.is_ok());
        assert!(verify_password(&hash, "p4ssw0rdd").await.is_err());
        Ok(())
    }
}
