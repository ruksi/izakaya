use axum::http::header::AUTHORIZATION;
use axum::http::HeaderValue;
use serde_json::{json, Value};

use crate::endpoints::api::sessions::list;
use crate::endpoints::verify::VerifyOut;
use crate::prelude::*;
use crate::test_utils::{login, mock_server};

fn bearer_auth_header(token: &str) -> HeaderValue {
    HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap()
}

#[sqlx::test]
async fn bearer_authentication_flow(db: sqlx::PgPool) -> Result<()> {
    let mut server = mock_server(&db).await;
    login::as_normal_user(&db, &server).await?;

    let response1 = server
        .post("/api/sessions")
        .json(&json!({"password": "bobIsBest"}))
        .await;
    response1.assert_status_ok();
    let json1 = response1.json::<Value>();
    let token1 = json1.get("access_token").unwrap().as_str().unwrap();
    assert_eq!(token1.len(), 64);

    let response2 = server
        .post("/api/sessions")
        .json(&json!({"password": "bobIsBest"}))
        .await;
    response2.assert_status_ok();
    let json2 = response2.json::<Value>();
    let token2 = json2.get("access_token").unwrap().as_str().unwrap();
    assert_eq!(token2.len(), 64);

    assert_ne!(token1, token2);

    // clear cookies now that we have the tokens
    let verification = server.get("/verify").await.json::<VerifyOut>();
    assert!(verification.is_authenticated);
    server.clear_cookies();
    let verification = server.get("/verify").await.json::<VerifyOut>();
    assert!(!verification.is_authenticated);

    // shows both all sessions (cookie[deleted], token1, token2)
    let sessions = server
        .get("/api/sessions")
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await
        .json::<Vec<list::ListSessionOut>>();
    assert_eq!(sessions.len(), 3);
    assert!(sessions
        .iter()
        .all(|session| session.access_token_prefix.len() == 8));

    // you can delete any of the sessions (token1 here)
    let verification = server
        .get("/verify")
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await
        .json::<VerifyOut>();
    assert!(verification.is_authenticated);
    server
        .delete(format!("/api/sessions/{}", token1).as_str()) // full token instead of prefix 🤷
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await
        .assert_status_ok();
    let verification = server
        .get("/verify")
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await
        .json::<VerifyOut>();
    assert!(!verification.is_authenticated);

    // the token1 session is gone
    let sessions = server
        .get("/api/sessions")
        .add_header(AUTHORIZATION, bearer_auth_header(token2))
        .await
        .json::<Vec<list::ListSessionOut>>();
    assert_eq!(sessions.len(), 2);

    Ok(())
}
