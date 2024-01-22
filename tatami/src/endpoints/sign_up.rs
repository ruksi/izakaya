use axum::extract::State;
use axum::Json;
use axum_extra::extract::PrivateCookieJar;
use serde_json::{json, Value};

use crate::auth::{cookie, issue_access_token};
use crate::prelude::*;
use crate::state::AppState;
use crate::user;
use crate::user::UserDeclaration;
use crate::valid::Valid;

pub async fn sign_up(
    State(state): State<AppState>,
    mut jar: PrivateCookieJar,
    declaration: Valid<UserDeclaration>,
) -> Result<(PrivateCookieJar, Json<Value>)> {
    // we need the password after consume to create the access token
    let password = declaration.inner_as_ref().password.clone();

    let user = user::create(&state.db_pool, declaration).await?;

    let access_token = issue_access_token(&state, user.username, password).await?;
    let cookie = cookie::bake(cookie::ACCESS_TOKEN, access_token, time::Duration::days(14));
    jar = jar.add(cookie);

    Ok((jar, Json(json!({"status": "ok"}))))
}
