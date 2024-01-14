use crate::auth::{access_token_key, access_token_list_key};
use crate::state::AppState;
use crate::{crypto, error};
use axum::http::StatusCode;
use rand::Rng;

pub async fn issue_access_token(
    state: &AppState,
    username_or_email: String,
    password: String,
) -> Result<String, (StatusCode, String)> {
    let result = sqlx::query!(
        // language=SQL
        r#"select user_id, password_hash
               from "user"
               left join user_email using (user_id)
               where username = $1
               or address = $1;"#,
        username_or_email,
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(error::internal)?;

    let Some(record) = result else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password.".into(),
        ));
    };

    let Some(password_hash) = record.password_hash else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password.".into(),
        ));
    };

    let verification = crypto::verify_password(password_hash, password).await;
    if verification.is_err() {
        // probably "invalid password"
        return Err((
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password.".into(),
        ));
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

    let mut cache_conn = state.cache_pool.get().await.map_err(error::internal)?;

    deadpool_redis::redis::cmd("HSET")
        .arg(&[
            access_token_key(token.clone()),
            "user_id".into(),
            record.user_id.into(),
        ])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    deadpool_redis::redis::cmd("RPUSH")
        .arg(&[access_token_list_key(record.user_id), token.clone()])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    Ok(token)
}
