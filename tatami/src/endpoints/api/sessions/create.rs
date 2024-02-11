use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::auth::CurrentUser;
use crate::prelude::*;
use crate::session;
use crate::state::AppState;
use crate::user;

#[derive(Deserialize, Debug)]
pub struct CreateSessionIn {
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateSessionOut {
    access_token: String,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(inbound): Json<CreateSessionIn>,
) -> Result<Json<CreateSessionOut>> {
    let user_id = current_user.user_id;
    let user = user::describe(&state.db_pool, user_id).await?;
    let Some(user) = user else {
        tracing::error!("CurrentUser {} could not find itself", user_id);
        return Err(Error::NotFound);
    };

    let (access_token, _session_id) = session::create(
        &state,
        user.username,
        inbound.password,
        None, // "API Tokens" never expire for now
    )
    .await?;

    let outbound = CreateSessionOut { access_token };
    Ok(Json(outbound))
}
