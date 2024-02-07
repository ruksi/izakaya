use axum::http::header::AUTHORIZATION;
use axum::http::HeaderValue;
use serde_json::{json, Value};

use crate::endpoints::api::sessions::list;
use crate::prelude::*;
use crate::test_utils::mock_server;
use crate::user;
use crate::user::UserDeclaration;

fn bearer_auth_header(token: &str) -> HeaderValue {
    HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap()
}

#[sqlx::test]
async fn bearer_authentication_flow(db: sqlx::PgPool) -> Result<()> {
    let mut server = mock_server(&db).await;

    // create and login with a new user, this saves credentials to the cookies
    let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "bobIsBest")?;
    user::create(&db, declaration).await?;
    server
        .post("/log-in")
        .json(&json!({"username_or_email": "bob@example.com", "password": "bobIsBest"}))
        .await
        .assert_status_ok();

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
    server.get("/verify").await.assert_status_ok();
    server.clear_cookies();
    let response = server.get("/verify").await;
    let response_json = response.json::<Value>();
    assert!(!response_json
        .get("is_authenticated")
        .unwrap()
        .as_bool()
        .unwrap());

    // shows both all sessions (cookie[deleted], token1, token2)
    let sessions = server
        .get("/api/sessions")
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await
        .json::<Vec<list::PublicSession>>();
    assert_eq!(sessions.len(), 3);
    assert!(sessions
        .iter()
        .all(|session| session.access_token_prefix.len() == 8));

    // you can delete any of the sessions (token1 here)
    let response = server
        .get("/verify")
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await;
    let response_json = response.json::<Value>();
    assert!(response_json
        .get("is_authenticated")
        .unwrap()
        .as_bool()
        .unwrap());
    server
        .delete(format!("/api/sessions/{}", token1).as_str()) // full token instead of prefix 🤷
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await
        .assert_status_ok();
    let response = server
        .get("/verify")
        .add_header(AUTHORIZATION, bearer_auth_header(token1))
        .await;
    let response_json = response.json::<Value>();
    assert!(!response_json
        .get("is_authenticated")
        .unwrap()
        .as_bool()
        .unwrap());

    // the token1 session is gone
    let sessions = server
        .get("/api/sessions")
        .add_header(AUTHORIZATION, bearer_auth_header(token2))
        .await
        .json::<Vec<list::PublicSession>>();
    assert_eq!(sessions.len(), 2);

    Ok(())
}
