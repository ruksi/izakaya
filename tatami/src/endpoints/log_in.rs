use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use tower_cookies::Cookies;

use crate::auth::{cookie, create_session};
use crate::prelude::*;
use crate::state::AppState;

#[derive(serde::Deserialize, Debug)]
pub struct LogInBody {
    username_or_email: String,
    password: String,
}

pub async fn log_in(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(body): Json<LogInBody>,
) -> Result<Json<Value>> {
    let (access_token, session_id) = create_session(
        &state,
        body.username_or_email,
        body.password,
        Some(time::Duration::days(14) + time::Duration::minutes(1)),
    )
    .await?;

    let private_cookies = cookies.private(&state.config.cookie_secret);
    private_cookies.add(cookie::bake_access(&state.config, access_token));

    // the pre-session CSRF cookie won't work with authenticated requests
    // because of the session ID check, so we need to replace it
    cookies.add(cookie::bake_csrf(&state.config, Some(session_id)));

    Ok(Json(json!({"status": "ok"})))
}
