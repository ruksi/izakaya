use axum::http::header::AUTHORIZATION;
use axum::http::HeaderValue;
use axum_test::TestServer;
use serde_json::{json, Value};

use crate::prelude::*;
use crate::test_utils::mock_state;
use crate::user;
use crate::user::UserDeclaration;

use super::*;

#[sqlx::test]
async fn bearer_authentication_flow(pool: sqlx::PgPool) -> Result<()> {
    let state = mock_state(pool).await;
    let db = &state.db_pool;
    let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "bobIsBest")?;
    user::create(db, declaration).await?;

    let server = TestServer::new(router(state.clone()).layer(
        axum::middleware::from_fn_with_state(state.clone(), crate::auth::record_visit),
    ))
    .unwrap();

    // wrong password
    server
        .post("/")
        .json(&json!({"username_or_email": "bob", "password": "bobIsBes"}))
        .await
        .assert_status_unauthorized();

    // wrong username
    server
        .post("/")
        .json(&json!({"username_or_email": "bobby", "password": "bobIsBest"}))
        .await
        .assert_status_unauthorized();

    // works with username
    let response1 = server
        .post("/")
        .json(&json!({"username_or_email": "bob", "password": "bobIsBest"}))
        .await;
    response1.assert_status_ok();
    let json1 = response1.json::<Value>();
    let token1 = json1.get("accessToken").unwrap().as_str().unwrap();
    assert_eq!(token1.len(), 64);

    // works with email
    let response2 = server
        .post("/")
        .json(&json!({"username_or_email": "bob@example.com", "password": "bobIsBest"}))
        .await;
    response2.assert_status_ok();
    let json2 = response2.json::<Value>();
    let token2 = json2.get("accessToken").unwrap().as_str().unwrap();
    assert_eq!(token2.len(), 64);

    // double log-in is fine, you get two separate tokens
    assert_ne!(token1, token2);

    // logout with token is fine as ever
    server.delete("/").await.assert_status_ok();

    // logout with an unknown token is fine too
    server
        .delete("/")
        .add_header(AUTHORIZATION, HeaderValue::from_static("Bearer unknown"))
        .await
        .assert_status_ok();

    // shows both sessions
    let sessions_json = server
        .get("/")
        .add_header(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token2).as_str()).unwrap(),
        )
        .await
        .json::<Value>();
    let sessions = sessions_json.get("sessions").unwrap().as_array().unwrap();
    assert_eq!(sessions.len(), 2);
    assert!(sessions
        .iter()
        .all(|session| session.as_str().unwrap().len() == 8));

    // logout with a valid token is the same
    server
        .delete("/")
        .add_header(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token1).as_str()).unwrap(),
        )
        .await
        .assert_status_ok();

    // the session is gone
    let sessions_json = server
        .get("/")
        .add_header(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token2).as_str()).unwrap(),
        )
        .await
        .json::<Value>();
    let sessions = sessions_json.get("sessions").unwrap().as_array().unwrap();
    assert_eq!(sessions.len(), 1);

    // again is fine and responds the same
    server
        .delete("/")
        .add_header(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token1).as_str()).unwrap(),
        )
        .await
        .assert_status_ok();

    // and again, just as fine
    server
        .delete("/")
        .add_header(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token2).as_str()).unwrap(),
        )
        .await
        .assert_status_ok();

    Ok(())
}
