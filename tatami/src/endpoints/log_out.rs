use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_extra::extract::PrivateCookieJar;
use serde_json::json;

use crate::auth::{cookie, revoke_access_token, Visitor};
use crate::prelude::*;
use crate::state::AppState;

pub async fn log_out(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
    mut jar: PrivateCookieJar,
) -> Result<impl IntoResponse> {
    let Some(access_token) = visitor.access_token else {
        return Ok((jar, Json(json!({}))));
    };
    let Some(user_id) = visitor.user_id else {
        return Ok((jar, Json(json!({}))));
    };

    revoke_access_token(&state, access_token.clone(), user_id).await?;
    let cookie = cookie::bake_for_backend(
        cookie::ACCESS_TOKEN,
        access_token,
        state.config.cookie_domain,
        time::Duration::ZERO, // i.e. delete it
    );
    jar = jar.add(cookie);

    Ok((jar, Json(json!({}))))
}
