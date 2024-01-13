use std::collections::HashMap;

use axum::Extension;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Clone)]
pub struct Visitor {
    pub user_id: Option<Uuid>,
    pub access_token: Option<String>,
}

impl Visitor {
    pub fn is_anonymous(&self) -> bool {
        self.user_id.is_none()
    }
}

pub async fn record_visit(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let bearer = request.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let mut visitor = Visitor { user_id: None, access_token: None };

    if let Some(bearer) = bearer {
        if let Ok(mut conn) = state.cache_pool.get().await {
            let session = deadpool_redis::redis::cmd("HGETALL")
                .arg(access_token_key(bearer))
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

pub fn access_token_key<T: Into<String>>(token: T) -> String {
    format!("tatami:access-token:{}", token.into())
}

pub fn access_token_list_key(user_id: Uuid) -> String {
    format!("tatami:access-tokens:{}", user_id.to_string())
}

pub async fn require_login(
    Extension(visitor): Extension<Visitor>,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    if visitor.is_anonymous() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use axum::{Extension, Json, Router};
    use axum::http::{HeaderValue, StatusCode};
    use axum::routing::get;
    use axum_test::TestServer;
    use serde_json::{json, Value};

    use crate::test_utils::mock_state;

    use super::*;

    async fn mock_endpoint(
        Extension(visitor): Extension<Visitor>,
    ) -> Result<Json<Value>, (StatusCode, String)> {
        Ok(Json(json!({"is_anonymous": visitor.is_anonymous()})))
    }

    #[sqlx::test]
    async fn auth_middleware_work(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let app = Router::new()
            .route("/closed", get(mock_endpoint))
            .route_layer(axum::middleware::from_fn(require_login))
            .route("/open", get(mock_endpoint))
            .layer(axum::middleware::from_fn_with_state(state.clone(), record_visit))
            .with_state(state.clone());
        let server = TestServer::new(app).unwrap();

        let mut cache_conn = state.cache_pool.get().await.unwrap();
        deadpool_redis::redis::cmd("DEL")
            .arg(&[access_token_key("test")])
            .query_async::<_, ()>(&mut cache_conn)
            .await
            .unwrap();

        server.get("/open")
            .await
            .assert_json(&json!({"is_anonymous": true}));
        server.get("/open")
            .add_header(AUTHORIZATION, HeaderValue::from_static("Bearer test"))
            .await
            .assert_json(&json!({"is_anonymous": true}));

        server.get("/closed")
            .await
            .assert_status_unauthorized();
        server.get("/closed")
            .add_header(AUTHORIZATION, HeaderValue::from_static("Bearer test"))
            .await
            .assert_status_unauthorized();

        deadpool_redis::redis::cmd("HSET")
            .arg(&[
                access_token_key("test"),
                "user_id".into(),
                "00000000-0000-0000-0000-000000000000".into(),
            ])
            .query_async::<_, ()>(&mut cache_conn)
            .await
            .unwrap();
        deadpool_redis::redis::cmd("EXPIRE")
            .arg(&[access_token_key("test"), "10".into()])
            .query_async::<_, ()>(&mut cache_conn)
            .await
            .unwrap();

        server.get("/closed")
            .await
            .assert_status_unauthorized();
        server.get("/closed")
            .add_header(AUTHORIZATION, HeaderValue::from_static("Bearer test"))
            .await
            .assert_json(&json!({"is_anonymous": false}));

        deadpool_redis::redis::cmd("DEL")
            .arg(&[access_token_key("test")])
            .query_async::<_, ()>(&mut cache_conn)
            .await
            .unwrap();
    }
}
