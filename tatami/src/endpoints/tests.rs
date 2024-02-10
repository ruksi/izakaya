use crate::endpoints::verify::Verification;
use serde_json::json;

use crate::prelude::*;
use crate::test_utils::mock_server;

#[sqlx::test]
async fn browser_authentication_flow(db: sqlx::PgPool) -> Result<()> {
    let server = mock_server(&db).await;

    // you start unauthorized
    let verification = server.get("/verify").await.json::<Verification>();
    assert!(!verification.is_authenticated);

    // you can sign up and get automatically logged in
    server
        .post("/sign-up")
        .json(&json!({"username": "bob", "email": "bob@example.com", "password": "bobIsBest"}))
        .await
        .assert_status_ok();
    let verification = server.get("/verify").await.json::<Verification>();
    assert!(verification.is_authenticated);

    // you can log out
    server.post("/log-out").await.assert_status_ok();
    let verification = server.get("/verify").await.json::<Verification>();
    assert!(!verification.is_authenticated);

    // you can log in
    server
        .post("/log-in")
        .json(&json!({"username_or_email": "bob", "password": "bobIsBest"}))
        .await
        .assert_status_ok();
    let verification = server.get("/verify").await.json::<Verification>();
    assert!(verification.is_authenticated);

    // multiple log-out is fine
    server.post("/log-out").await.assert_status_ok();
    server.post("/log-out").await.assert_status_ok();
    server.post("/log-out").await.assert_status_ok();

    let verification = server.get("/verify").await.json::<Verification>();
    assert!(!verification.is_authenticated);

    Ok(())
}
