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
    pub bearer: Option<String>,
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
    let bearer_token = request.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let mut visitor = Visitor { user_id: None, bearer: None };

    if let Some(bearer_token) = bearer_token {
        if let Ok(mut conn) = state.cache_pool.get().await {
            let session = deadpool_redis::redis::cmd("HGETALL")
                .arg(bearer_key(bearer_token))
                .query_async::<_, HashMap<String, String>>(&mut conn)
                .await
                .unwrap_or_default();
            if let Some(user_id) = session.get("user_id") {
                // TODO: log an error if fails to parse?
                if let Ok(user_id) = Uuid::parse_str(user_id) {
                    visitor.user_id = Some(user_id);
                    visitor.bearer = Some(bearer_token.to_string());
                }
            };
        }
    }

    request.extensions_mut().insert(visitor);
    next.run(request).await
}

pub fn bearer_key<T: Into<String>>(token: T) -> String {
    format!("tatami:bearer:{}", token.into())
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

    use crate::state::AppState;

    use super::*;

    async fn test_cache_pool() -> deadpool_redis::Pool {
        deadpool_redis::Config::from_url("redis://localhost:6379/9")
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect("Failed to create cache pool for tests")
    }

    async fn mock_endpoint(
        Extension(visitor): Extension<Visitor>,
    ) -> Result<Json<Value>, (StatusCode, String)> {
        Ok(Json(json!({"is_anonymous": visitor.is_anonymous()})))
    }

    #[sqlx::test]
    async fn auth_middleware_work(pool: sqlx::PgPool) {
        let state = AppState { db_pool: pool, cache_pool: test_cache_pool().await };
        let app = Router::new()
            .route("/closed", get(mock_endpoint))
            .route_layer(axum::middleware::from_fn(require_login))
            .route("/open", get(mock_endpoint))
            .layer(axum::middleware::from_fn_with_state(state.clone(), record_visit))
            .with_state(state.clone());
        let server = TestServer::new(app).unwrap();

        let mut cache_conn = state.cache_pool.get().await.unwrap();
        deadpool_redis::redis::cmd("DEL")
            .arg(&[bearer_key("test")])
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
                bearer_key("test"),
                "user_id".into(),
                "00000000-0000-0000-0000-000000000000".into(),
            ])
            .query_async::<_, ()>(&mut cache_conn)
            .await
            .unwrap();
        deadpool_redis::redis::cmd("EXPIRE")
            .arg(&[bearer_key("test"), "10".into()])
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
            .arg(&[bearer_key("test")])
            .query_async::<_, ()>(&mut cache_conn)
            .await
            .unwrap();
    }
}
