use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

use crate::auth::{access_token_key, access_token_list_key, Visitor};
use crate::error;
use crate::state::AppState;

pub async fn destroy(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
) -> Result<Json<()>, (StatusCode, String)> {
    let Some(access_token) = visitor.access_token else {
        return Ok(Json(()));
    };
    let Some(user_id) = visitor.user_id else {
        return Ok(Json(()));
    };

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

    Ok(Json(()))
}
