use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::Visitor;
use crate::prelude::*;

pub async fn verify(Extension(visitor): Extension<Visitor>) -> Result<Json<Value>> {
    let user_id = visitor.get_user_id_or_respond_unauthorized()?;
    Ok(Json(json!({"userId": user_id})))
}
