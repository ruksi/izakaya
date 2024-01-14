use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

use crate::auth::{revoke_access_token, Visitor};
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
    revoke_access_token(state, access_token, user_id).await?;
    Ok(Json(()))
}
