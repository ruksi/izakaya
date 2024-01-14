use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use axum_test::{TestServer, TestServerConfig};
use serde_json::{json, Value};

use crate::test_utils::mock_state;

use super::*;

#[sqlx::test]
async fn browser_authentication_flow(pool: sqlx::PgPool) -> Result<(), (StatusCode, String)> {
    let state = mock_state(pool).await;
    let mock_app = Router::new()
        .route("/sign-up", post(sign_up))
        .route("/log-in", post(log_in))
        .route("/log-out", post(log_out))
        .route("/verify", get(verify))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            crate::auth::record_visit,
        ))
        .with_state(state.clone());
    let config = TestServerConfig::builder().save_cookies().build(); // <- automatically use cookies
    let server = TestServer::new_with_config(mock_app, config).unwrap();

    // you start unauthorized
    server.get("/verify").await.assert_status_unauthorized();

    // you can sign up and get automatically logged in
    server
        .post("/sign-up")
        .json(&json!({"username": "bob", "email": "bob@example.com", "password": "bobIsBest"}))
        .await
        .assert_status_ok();
    let response = server.get("/verify").await;
    response.assert_status_ok();
    let response_json = response.json::<Value>();
    assert_eq!(response_json.get("status").unwrap(), "ok");
    assert!(uuid::Uuid::parse_str(response_json.get("userId").unwrap().as_str().unwrap()).is_ok());

    // you can log out
    server.post("/log-out").await.assert_status_ok();
    server.get("/verify").await.assert_status_unauthorized();

    // you can log in
    server
        .post("/log-in")
        .json(&json!({"username_or_email": "bob", "password": "bobIsBest"}))
        .await
        .assert_status_ok();
    let response = server.get("/verify").await;
    response.assert_status_ok();
    let response_json = response.json::<Value>();
    assert_eq!(response_json.get("status").unwrap(), "ok");
    assert!(uuid::Uuid::parse_str(response_json.get("userId").unwrap().as_str().unwrap()).is_ok());

    // multiple log-out is fine
    server.post("/log-out").await.assert_status_ok();
    server.post("/log-out").await.assert_status_ok();
    server.post("/log-out").await.assert_status_ok();
    server.get("/verify").await.assert_status_unauthorized();

    Ok(())
}
