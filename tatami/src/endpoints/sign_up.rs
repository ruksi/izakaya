use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use tower_cookies::Cookies;

use crate::auth::{cookie, issue_access_token};
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
    let access_token = issue_access_token(
        &state,
        user.username,
        password,
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

    Ok(Json(json!({"status": "ok"})))
}
