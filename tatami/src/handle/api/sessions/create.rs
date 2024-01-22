use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::auth::issue_access_token;
use crate::prelude::*;
use crate::state::AppState;

#[derive(serde::Deserialize, Debug)]
pub struct CreateSessionBody {
    username_or_email: String,
    password: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateSessionBody>,
) -> Result<Json<Value>> {
    let access_token = issue_access_token(&state, body.username_or_email, body.password).await?;
    Ok(Json(json!({"accessToken": access_token})))
}
