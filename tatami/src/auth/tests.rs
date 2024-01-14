use axum::{Extension, Json, Router};
use axum::http::{header, HeaderValue, StatusCode};
use axum::routing::get;
use axum_test::TestServer;
use serde_json::{json, Value};

use crate::test_utils::mock_state;

use crate::auth::*;
use crate::auth::cache_keys::access_token_key;
use crate::auth::{record_visit, require_login};

async fn mock_endpoint(
    Extension(visitor): Extension<Visitor>,
) -> Result<Json<Value>, (StatusCode, String)> {
    Ok(Json(json!({"is_anonymous": visitor.is_anonymous()})))
}

#[sqlx::test]
async fn authentication_flow_works(pool: sqlx::PgPool) {
    let state = mock_state(pool).await;
    let app = Router::new()
        .route("/private", get(mock_endpoint))
        .route_layer(axum::middleware::from_fn(require_login::require_login))
        .route("/public", get(mock_endpoint))
        .layer(axum::middleware::from_fn_with_state(state.clone(), record_visit::record_visit))
        .with_state(state.clone());
    let server = TestServer::new(app).unwrap();

    let mut cache_conn = state.cache_pool.get().await.unwrap();
    deadpool_redis::redis::cmd("DEL")
        .arg(&[access_token_key("test")])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .unwrap();

    server.get("/public")
        .await
        .assert_json(&json!({"is_anonymous": true}));
    server.get("/public")
        .add_header(header::AUTHORIZATION, HeaderValue::from_static("Bearer test"))
        .await
        .assert_json(&json!({"is_anonymous": true}));

    server.get("/private")
        .await
        .assert_status_unauthorized();
    server.get("/private")
        .add_header(header::AUTHORIZATION, HeaderValue::from_static("Bearer test"))
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

    server.get("/private")
        .await
        .assert_status_unauthorized();
    server.get("/private")
        .add_header(header::AUTHORIZATION, HeaderValue::from_static("Bearer test"))
        .await
        .assert_json(&json!({"is_anonymous": false}));

    deadpool_redis::redis::cmd("DEL")
        .arg(&[access_token_key("test")])
        .query_async::<_, ()>(&mut cache_conn)
        .await
        .unwrap();
}
