use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::{CurrentUser, Visitor};
use crate::prelude::*;

// Bare minimum endpoint to verify that the user credentials are valid.

pub async fn verify(Extension(visitor): Extension<Visitor>) -> Result<Json<Value>> {
    let current_user = CurrentUser::from_visitor(visitor)?;
    Ok(Json(json!({"userId": current_user.user_id})))
}
