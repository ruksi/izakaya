use std::collections::HashMap;

use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::PrivateCookieJar;
use uuid::Uuid;

use crate::auth;
use crate::auth::Visitor;
use crate::session::cookie;
use crate::state::AppState;

pub async fn record_visit(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let bearer = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let mut visitor = Visitor {
        user_id: None,
        access_token: None,
    };

    if let Some(bearer) = bearer {
        if let Ok(mut conn) = state.cache_pool.get().await {
            let session = deadpool_redis::redis::cmd("HGETALL")
                .arg(auth::access_token_key(bearer))
                .query_async::<_, HashMap<String, String>>(&mut conn)
                .await
                .unwrap_or_default();
            if let Some(user_id) = session.get("user_id") {
                if let Ok(user_id) = Uuid::parse_str(user_id) {
                    visitor.user_id = Some(user_id);
                    visitor.access_token = Some(bearer.to_string());
                } else {
                    tracing::error!("failed to parse session user_id: {}", user_id);
                }
            };
        }
    }

    if visitor.user_id.is_none() {
        let access_token = jar
            .get(cookie::ACCESS_TOKEN)
            .map(|cookie| cookie.value().to_owned());
        if let Some(access_token) = access_token {
            if access_token != "<deleted>" {
                if let Ok(mut conn) = state.cache_pool.get().await {
                    let session = deadpool_redis::redis::cmd("HGETALL")
                        .arg(auth::access_token_key(&access_token))
                        .query_async::<_, HashMap<String, String>>(&mut conn)
                        .await
                        .unwrap_or_default();
                    if let Some(user_id) = session.get("user_id") {
                        if let Ok(user_id) = Uuid::parse_str(user_id) {
                            visitor.user_id = Some(user_id);
                            visitor.access_token = Some(access_token.to_string());
                        } else {
                            tracing::error!("failed to parse session user_id: {}", user_id);
                        }
                    };
                }
            }
        }
    }

    // let value: Option<String> = jar.get("test").map(|cookie| cookie.value().to_owned());

    request.extensions_mut().insert(visitor);
    next.run(request).await
}
