use std::collections::HashMap;

use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use redis::AsyncCommands;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::auth::{cookie, session_key, Visitor};
use crate::scripts::RedisScripts;
use crate::state::AppState;

pub async fn record_visit(
    State(state): State<AppState>,
    cookies: tower_cookies::Cookies,
    mut request: Request,
    next: Next,
) -> Response {
    // we have an anonymous visitor by default;
    // a stranger on the web, mechanical or organic
    let mut visitor = Visitor {
        access_token: None,
        session_id: None,
        user_id: None,
        is_superuser: false,
    };

    // first check if we can use `Authorization` header to identify them
    let bearer_token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));
    if let Some(found_visitor) = get_visitor(&state, bearer_token).await {
        visitor = found_visitor;
    }

    // second, check if we can use cookies to identify them
    if visitor.user_id.is_none() {
        let private_cookies = cookies.private(&state.config.cookie_secret);
        let cookie_token = private_cookies
            .get(cookie::ACCESS_TOKEN)
            .map(|cookie| cookie.value().to_owned());
        if let Some(found_visitor) = get_visitor(&state, cookie_token).await {
            visitor = found_visitor;
        }
    }

    if visitor.user_id.is_some() {
        // update the last usage time of the session in the background
        let move_session_key = session_key(visitor.access_token.as_ref().unwrap());
        let move_pool = state.cache_pool.clone();
        tokio::spawn(async move {
            let Ok(mut redis) = move_pool.get().await else {
                return;
            };
            let utc_now = time::OffsetDateTime::now_utc();
            let Ok(now_text) = utc_now.format(&Rfc3339) else {
                return;
            };
            let _ = redis.hset_x(move_session_key, "used_at", now_text).await;
        });
    }

    request.extensions_mut().insert(visitor);
    next.run(request).await
}

async fn get_visitor<T: Into<String>>(
    state: &AppState,
    access_token: Option<T>,
) -> Option<Visitor> {
    let Some(access_token) = access_token else {
        return None;
    };
    let access_token = access_token.into();

    let Ok(mut redis) = state.cache_pool.get().await else {
        return None;
    };

    let session: HashMap<String, String> = redis
        .hgetall(session_key(&access_token))
        .await
        .unwrap_or_default();

    let Some(session_id) = session.get("session_id") else {
        return None;
    };
    let Ok(session_id) = Uuid::parse_str(session_id) else {
        tracing::error!("failed to parse session session_id: {}", session_id);
        return None;
    };

    let Some(user_id) = session.get("user_id") else {
        return None;
    };
    let Ok(user_id) = Uuid::parse_str(user_id) else {
        tracing::error!("failed to parse session user_id: {}", user_id);
        return None;
    };

    let visitor = Visitor {
        access_token: Some(access_token),
        session_id: Some(session_id),
        user_id: Some(user_id),
        is_superuser: session.get("is_superuser").map_or(false, |v| v == "true"),
    };
    Some(visitor)
}
