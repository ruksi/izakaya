use axum::extract::State;
use axum::Json;
use axum_extra::extract::cookie::PrivateCookieJar;
use serde_json::{json, Value};

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
    mut jar: PrivateCookieJar,
    Json(body): Json<LogInBody>,
) -> Result<(PrivateCookieJar, Json<Value>)> {
    let access_token = issue_access_token(&state, body.username_or_email, body.password).await?;
    let cookie = cookie::bake(cookie::ACCESS_TOKEN, access_token, time::Duration::days(14));
    jar = jar.add(cookie);

    Ok((jar, Json(json!({"status": "ok"}))))
}
