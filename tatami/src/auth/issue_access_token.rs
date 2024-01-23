use rand::Rng;
use redis;

use crate::auth::{crypto, session_key, session_list_key};
use crate::prelude::*;
use crate::state::AppState;

pub async fn issue_access_token(
    state: &AppState,
    username_or_email: String,
    password: String,
    expire: Option<time::Duration>,
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
    let access_token: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let mut redis = state.cache_pool.get().await?;

    let session_key = session_key(access_token.clone());
    let session_list_key = session_list_key(record.user_id);

    let mut commands = redis::pipe()
        .hset(session_key.clone(), "user_id", record.user_id.to_string())
        .rpush(session_list_key, access_token.clone())
        .to_owned();

    if let Some(expire) = expire {
        commands = commands
            .expire(session_key.clone(), expire.whole_seconds())
            .to_owned();
    }

    commands.query_async(&mut redis).await?;

    Ok(access_token)
}
