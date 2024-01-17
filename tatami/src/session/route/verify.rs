use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::Visitor;
use crate::prelude::*;

pub async fn verify(Extension(visitor): Extension<Visitor>) -> Result<Json<Value>> {
    let Some(user_id) = visitor.user_id else {
        return Err(Error::Unauthorized);
    };
    Ok(Json(json!({"userId": user_id})))
}
