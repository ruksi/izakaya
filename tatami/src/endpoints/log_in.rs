use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::auth::{cookie, issue_access_token};
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
    let access_token = issue_access_token(
        &state,
        body.username_or_email,
        body.password,
        Some(time::Duration::days(14) + time::Duration::minutes(1)),
    )
    .await?;

    let cookie = cookie::bake_for_backend(
        cookie::ACCESS_TOKEN,
        access_token,
        state.config.cookie_domain,
        time::Duration::days(14),
    );
    let private_cookies = cookies.private(&state.config.cookie_secret);
    private_cookies.add(cookie);

    // destroy the pre-session CSRF cookie
    // TODO: would probably be better to generate new CSRF token here
    cookies.remove(Cookie::from(cookie::CSRF_TOKEN));

    Ok(Json(json!({"status": "ok"})))
}
