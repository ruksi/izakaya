use serde_json::{json, Value};

use crate::prelude::*;
use crate::test_utils::mock_server;

#[sqlx::test]
async fn browser_authentication_flow(db: sqlx::PgPool) -> Result<()> {
    let server = mock_server(&db).await;

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
    assert!(uuid::Uuid::parse_str(response_json.get("userId").unwrap().as_str().unwrap()).is_ok());

    // multiple log-out is fine
    server.post("/log-out").await.assert_status_ok();
    server.post("/log-out").await.assert_status_ok();
    server.post("/log-out").await.assert_status_ok();
    server.get("/verify").await.assert_status_unauthorized();

    Ok(())
}
