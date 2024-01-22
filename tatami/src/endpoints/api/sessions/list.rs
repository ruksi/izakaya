use axum::extract::State;
use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::{access_token_list_key, Visitor};
use crate::prelude::*;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
) -> Result<Json<Value>> {
    let user_id = visitor.get_user_id_or_respond_unauthorized()?;

    let mut cache_conn = state.cache_pool.get().await?;

    let tokens: Vec<String> = deadpool_redis::redis::cmd("LRANGE")
        .arg(&[access_token_list_key(user_id), "0".into(), "-1".into()])
        .query_async(&mut cache_conn)
        .await?;

    // we don't want to expose the full tokens
    let tokens: Vec<String> = tokens
        .into_iter()
        .map(|token| token[..8].to_string())
        .collect();

    Ok(Json(json!({"sessions": tokens})))
}
