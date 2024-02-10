use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use tower_cookies::Cookies;

use crate::auth::{cookie, create_session};
use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::UserDeclaration;
use crate::valid::Valid;

pub async fn sign_up(
    State(state): State<AppState>,
    cookies: Cookies,
    declaration: Valid<UserDeclaration>,
) -> Result<Json<Value>> {
    // we need the password after consume to create the access token
    let password = declaration.inner_as_ref().password.clone();
    let user = user::create(&state.db_pool, declaration).await?;
    let (access_token, session_id) = create_session(
        &state,
        user.username,
        password,
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
