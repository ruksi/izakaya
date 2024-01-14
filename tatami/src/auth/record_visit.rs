use std::collections::HashMap;

use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

use crate::auth;
use crate::auth::Visitor;
use crate::state::AppState;

pub async fn record_visit(
    State(state): State<AppState>,
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
                // TODO: log an error if fails to parse?
                if let Ok(user_id) = Uuid::parse_str(user_id) {
                    visitor.user_id = Some(user_id);
                    visitor.access_token = Some(bearer.to_string());
                }
            };
        }
    }

    request.extensions_mut().insert(visitor);
    next.run(request).await
}
