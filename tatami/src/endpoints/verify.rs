use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::Visitor;
use crate::prelude::*;

// Bare minimum endpoint to verify that the user credentials are valid.

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyOut {
    pub is_authenticated: bool,
    pub user_id: Option<Uuid>,
}

pub async fn verify(Extension(visitor): Extension<Visitor>) -> Result<Json<VerifyOut>> {
    Ok(Json(VerifyOut {
        is_authenticated: visitor.user_id.is_some(),
        user_id: visitor.user_id,
    }))
}
