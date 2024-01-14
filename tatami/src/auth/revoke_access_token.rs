use crate::auth::{access_token_key, access_token_list_key};
use crate::error;
use crate::state::AppState;
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn revoke_access_token(
    state: AppState,
    access_token: String,
    user_id: Uuid,
) -> Result<(), (StatusCode, String)> {
    let mut cache_conn = state.cache_pool.get().await.map_err(error::internal)?;

    deadpool_redis::redis::cmd("DEL")
        .arg(&[access_token_key(access_token.clone())])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    deadpool_redis::redis::cmd("LREM")
        .arg(&[
            access_token_list_key(user_id),
            "0".into(),
            access_token.clone(),
        ])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    Ok(())
}
