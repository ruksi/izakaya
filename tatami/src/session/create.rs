use rand::Rng;
use uuid::Uuid;

use crate::auth::{session_key, session_set_key, verify_password};
use crate::prelude::*;
use crate::state::AppState;

pub async fn create(
    state: &AppState,
    username_or_email: String,
    password: String,
    expire: Option<time::Duration>,
) -> Result<(String, Uuid)> {
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

    let verification = verify_password(record.password_hash, password);
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
    let session_list_key = session_set_key(record.user_id);
    let session_id = Uuid::new_v4();

    let mut commands = redis::pipe()
        .hset(session_key.clone(), "session_id", session_id.to_string())
        .hset(session_key.clone(), "access_token", access_token.clone()) // duplicate info vs. the key but 🤷
        .hset(session_key.clone(), "user_id", record.user_id.to_string())
        .sadd(session_list_key, session_key.clone())
        .to_owned();

    if let Some(expire) = expire {
        commands = commands
            .expire(session_key.clone(), expire.whole_seconds())
            .to_owned();
    }

    commands.query_async(&mut redis).await?;

    Ok((access_token, session_id))
}
