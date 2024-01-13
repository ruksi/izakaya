use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::state::AppState;
use crate::user;
use crate::user::model::UserFilter;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:user_id", get(describe).patch(amend).delete(destroy))
        .with_state(state)
}

#[derive(serde::Deserialize, Debug)]
struct CreateUserBody {
    username: String,
    email: String,
    password: String,
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<Json<user::model::User>, (axum::http::StatusCode, String)> {
    let declaration = user::model::UserDeclaration::new(
        body.username,
        body.email,
        body.password,
    );
    let user = user::model::create(&state.db_pool, declaration).await?;
    Ok(Json(user))
}

async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<user::model::User>>, (axum::http::StatusCode, String)> {
    let users = user::model::list(&state.db_pool, UserFilter::default()).await?;
    Ok(Json(users))
}

async fn describe(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<user::model::User>, (axum::http::StatusCode, String)> {
    let user = user::model::describe(&state.db_pool, user_id).await?;
    match user {
        Some(user) => Ok(Json(user)),
        None => return Err((axum::http::StatusCode::NOT_FOUND, "User not found".into())),
    }
}

#[derive(serde::Deserialize, Debug)]
struct AmendUserBody {
    username: Option<String>,
}

async fn amend(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AmendUserBody>,
) -> Result<Json<user::model::User>, (axum::http::StatusCode, String)> {
    let amendment = user::model::UserAmendment {
        username: Some(body.username.unwrap()),
    };
    let user = user::model::amend(&state.db_pool, user_id, amendment).await?;
    Ok(Json(user))
}

async fn destroy(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    user::model::destroy(&state.db_pool, user_id).await?;
    Ok(Json(json!({"status": "ok"})))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use uuid::Uuid;

    use crate::test_utils::mock_state;
    use crate::user::model::UserDeclaration;

    use super::*;

    #[sqlx::test]
    async fn create_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();
        let response = server
            .post("/")
            .content_type(&"application/json")
            .json(&json!({
                "username": "bob",
                "email": "bob@example.com",
                "password": "bobIsBest",
            }))
            .await;
        let user = response.json::<user::model::User>();
        assert_eq!(user.username, "bob");
        assert_ne!(user.user_id, Uuid::nil());
    }

    #[sqlx::test]
    async fn list_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        server.get("/").await.assert_json(&json!([]));

        user::model::create(&state.db_pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await.unwrap();
        let users = server.get("/").await.json::<Vec<user::model::User>>();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "bob");
    }

    #[sqlx::test]
    async fn describe_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let response = server.get(format!("/{}", Uuid::new_v4()).as_str()).await;
        response.assert_status(axum::http::StatusCode::NOT_FOUND);
        response.assert_text("User not found");

        let user = user::model::create(&state.db_pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await.unwrap();
        let re_user = server.get(format!("/{}", user.user_id).as_str()).await.json::<user::model::User>();
        assert_eq!(user, re_user);
    }

    #[sqlx::test]
    async fn amend_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let user = user::model::create(&state.db_pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await.unwrap();

        let user = server
            .patch(format!("/{}", user.user_id).as_str())
            .content_type(&"application/json")
            .json(&json!({
                "username": "bobby",
            }))
            .await
            .json::<user::model::User>();
        assert_eq!(user.username, "bobby");

        let user = user::model::describe(&state.db_pool, user.user_id).await.unwrap().unwrap();
        assert_eq!(user.username, "bobby");
    }

    #[sqlx::test]
    async fn destroy_works(pool: sqlx::PgPool) {
        let state = mock_state(pool).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let user = user::model::create(&state.db_pool, UserDeclaration::new("bob", "bob@example.com", "pw")).await.unwrap();
        server.delete(format!("/{}", user.user_id).as_str()).await.assert_json(&json!({"status": "ok"}));

        let response = server.get(format!("/{}", user.user_id).as_str()).await;
        response.assert_status(axum::http::StatusCode::NOT_FOUND);
        response.assert_text("User not found");
    }
}
