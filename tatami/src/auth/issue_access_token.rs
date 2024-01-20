use rand::Rng;

use crate::auth::{access_token_key, access_token_list_key};
use crate::crypto;
use crate::prelude::*;
use crate::state::AppState;

pub async fn issue_access_token(
    state: &AppState,
    username_or_email: String,
    password: String,
) -> Result<String> {
    let result = sqlx::query!(
        // language=SQL
        r#"select user_id, password_hash
               from "user"
               left join user_email using (user_id)
               where username = $1
               or email = $1;"#,
        username_or_email,
    )
    .fetch_optional(&state.db_pool)
    .await?;

    let Some(record) = result else {
        return Err(Error::Unauthorized);
    };

    let verification = crypto::verify_password(record.password_hash, password).await;
    if verification.is_err() {
        return Err(Error::Unauthorized);
    }

    // UUIDs have 16 bytes of randomness, and that is considered enough
    // for session identifiers if the random generator is secure enough.
    // Let's generate 64 bytes of randomness, just to be sure as I'm
    // not 100% sure what is used to generate the numbers on the server. 🤷
    let token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let mut cache_conn = state.cache_pool.get().await?;

    deadpool_redis::redis::cmd("HSET")
        .arg(&[
            access_token_key(token.clone()),
            "user_id".into(),
            record.user_id.into(),
        ])
        .query_async::<_, ()>(&mut cache_conn)
        .await?;

    deadpool_redis::redis::cmd("RPUSH")
        .arg(&[access_token_list_key(record.user_id), token.clone()])
        .query_async::<_, ()>(&mut cache_conn)
        .await?;

    Ok(token)
}
