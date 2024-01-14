use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::{access_token_list_key, Visitor};
use crate::error;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let Some(user_id) = visitor.user_id else {
        return Err((StatusCode::UNAUTHORIZED, "You are not logged in.".into()));
    };

    let mut cache_conn = state.cache_pool.get().await.map_err(error::internal)?;

    let tokens: Vec<String> = deadpool_redis::redis::cmd("LRANGE")
        .arg(&[access_token_list_key(user_id), "0".into(), "-1".into()])
        .query_async(&mut cache_conn)
        .await
        .map_err(error::internal)?;

    // we don't want to expose the full tokens
    let tokens: Vec<String> = tokens
        .into_iter()
        .map(|token| token[..8].to_string())
        .collect();

    Ok(Json(json!({"sessions": tokens})))
}
