use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

use crate::auth::{cookie, revoke_access_token, CurrentUser, Visitor};
use crate::prelude::*;
use crate::state::AppState;

pub async fn log_out(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
    cookies: Cookies,
) -> Result<impl IntoResponse> {
    // if authenticated user, revoke their access token
    let current_user = CurrentUser::from_visitor(visitor);
    if let Ok(current_user) = current_user {
        let access_token = current_user.access_token.clone();
        let user_id = current_user.user_id;
        revoke_access_token(&state, access_token, user_id).await?;
    }

    // always remove the access token cookie
    cookies.remove(Cookie::from(cookie::ACCESS_TOKEN));

    Ok(Json(json!({})))
}
