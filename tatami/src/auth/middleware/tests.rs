use axum::http::{header, HeaderValue};
use axum::routing::get;
use axum::{Extension, Json, Router};
use axum_test::TestServer;
use redis::AsyncCommands;
use serde_json::{json, Value};
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

use crate::auth::cache_keys::session_key;
use crate::auth::middleware::*;
use crate::auth::*;
use crate::prelude::*;
use crate::test_utils::mock_state;

async fn mock_endpoint(Extension(visitor): Extension<Visitor>) -> Result<Json<Value>> {
    Ok(Json(json!({"is_anonymous": visitor.user_id.is_none()})))
}

#[sqlx::test]
async fn auth_middleware_work_with_bearer_header(db: sqlx::PgPool) {
    let state = mock_state(&db).await;
    let app = Router::new()
        .route("/private", get(mock_endpoint))
        .route_layer(axum::middleware::from_fn(require_login))
        .route("/public", get(mock_endpoint))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            record_visit,
        ))
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());
    let server = TestServer::new(app).unwrap();

    let mut redis = state.cache_pool.get().await.unwrap();
    let session_key = session_key("test");
    let bearer_value = HeaderValue::from_static("Bearer test");

    // make sure the test session token is not in Redis
    redis.del::<&str, ()>(&session_key).await.unwrap();

    // we are anonymous by default
    server
        .get("/public")
        .await
        .assert_json(&json!({"is_anonymous": true}));

    // we are anonymous even if we give a non-existent token
    server
        .get("/public")
        .add_header(header::AUTHORIZATION, bearer_value.clone())
        .await
        .assert_json(&json!({"is_anonymous": true}));

    // anonymous can't access private endpoints
    server.get("/private").await.assert_status_unauthorized();

    // not even if we pass a non-existent token
    server
        .get("/private")
        .add_header(header::AUTHORIZATION, bearer_value.clone())
        .await
        .assert_status_unauthorized();

    // make the session token valid
    redis::pipe()
        .hset(&session_key, "user_id", Uuid::nil().to_string()) // #YOLO
        .expire(&session_key, 10)
        .query_async::<_, ()>(&mut redis)
        .await
        .unwrap();

    // still, reject if no token is passed
    server.get("/private").await.assert_status_unauthorized();

    // can access private endpoints if we pass the token,
    // although in reality the user doesn't exist, but that
    // isn't checked to avoid a DB query on each request
    server
        .get("/private")
        .add_header(header::AUTHORIZATION, bearer_value.clone())
        .await
        .assert_json(&json!({"is_anonymous": false}));

    // cleanup
    redis.del::<&str, ()>(&session_key).await.unwrap();
}
