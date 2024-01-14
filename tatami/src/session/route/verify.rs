use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::Visitor;

pub async fn verify(
    Extension(visitor): Extension<Visitor>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let Some(user_id) = visitor.user_id else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"reason": "unauthorized"})),
        ));
    };
    Ok(Json(json!({"userId": user_id})))
}
