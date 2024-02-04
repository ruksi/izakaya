use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::Visitor;
use crate::prelude::*;

// Bare minimum endpoint to verify that the user credentials are valid.

pub async fn verify(Extension(visitor): Extension<Visitor>) -> Result<Json<Value>> {
    Ok(Json(json!({
        "is_authenticated": visitor.user_id.is_some(),
        "user_id": visitor.user_id,
    })))
}
