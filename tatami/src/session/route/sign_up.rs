use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::PrivateCookieJar;
use serde_json::{json, Value};

use crate::auth::issue_access_token;
use crate::session::cookie;
use crate::state::AppState;
use crate::user::model::UserDeclaration;

#[derive(serde::Deserialize, Debug)]
pub struct SignUpBody {
    username: String,
    email: String,
    password: String,
}

pub async fn sign_up(
    State(state): State<AppState>,
    mut jar: PrivateCookieJar,
    Json(body): Json<SignUpBody>,
) -> Result<(PrivateCookieJar, Json<Value>), (StatusCode, String)> {
    let declaration = UserDeclaration::new(body.username, body.email, body.password.clone());
    let user = crate::user::model::create(&state.db_pool, declaration).await?;

    let access_token = issue_access_token(&state, user.username, body.password).await?;
    let cookie = cookie::bake(cookie::ACCESS_TOKEN, access_token, time::Duration::days(14));
    jar = jar.add(cookie);

    Ok((jar, Json(json!({"status": "ok"}))))
}
