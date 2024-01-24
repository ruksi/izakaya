use axum::extract::State;
use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::{issue_access_token, CurrentUser};
use crate::prelude::*;
use crate::state::AppState;
use crate::user;

#[derive(serde::Deserialize, Debug)]
pub struct CreateSessionBody {
    password: String,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<CreateSessionBody>,
) -> Result<Json<Value>> {
    let user_id = current_user.user_id;
    let user = user::describe(&state.db_pool, user_id).await?;
    let Some(user) = user else {
        tracing::error!("CurrentUser {} could not find itself", user_id);
        return Err(Error::NotFound);
    };

    let access_token = issue_access_token(
        &state,
        user.username,
        body.password,
        None, // "API Tokens" never expire for now
    )
    .await?;
    Ok(Json(json!({"access_token": access_token})))
}
