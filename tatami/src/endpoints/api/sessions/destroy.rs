use axum::extract::{Path, State};
use axum::{Extension, Json};
use redis::AsyncCommands;

use crate::auth::{revoke_access_token, session_list_key, CurrentUser};
use crate::prelude::*;
use crate::state::AppState;

pub async fn destroy(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(access_token_prefix): Path<String>,
) -> Result<Json<()>> {
    if access_token_prefix.len() < 8 {
        return Err(Error::BadRequest); // TODO: could have a better error message
    }

    let user_id = current_user.user_id;
    let mut redis = state.cache_pool.get().await?;

    // find all access tokens of this user
    let access_tokens: Vec<String> = redis.lrange(session_list_key(user_id), 0, -1).await?;

    // find access tokens with the right prefix, there probably is just one
    // TODO: has a slight "feature" that removes all matching tokens, although matches are unlikely
    let access_tokens: Vec<String> = access_tokens
        .into_iter()
        .filter(|token| token.starts_with(&access_token_prefix))
        .collect();
    if access_tokens.is_empty() {
        return Err(Error::NotFound);
    }

    for access_token in access_tokens {
        revoke_access_token(state.clone(), access_token, user_id).await?;
    }
    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use axum::http::header::AUTHORIZATION;
    use axum::http::HeaderValue;
    use axum_test::TestServer;

    use crate::auth::issue_access_token;
    use crate::endpoints::router;
    use crate::test_utils::mock_state;
    use crate::user;
    use crate::user::UserDeclaration;

    use super::*;

    fn bearer_auth_header(token: &str) -> HeaderValue {
        HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap()
    }

    #[sqlx::test]
    async fn works(db: sqlx::PgPool) -> Result<()> {
        let state = mock_state(&db).await;
        let server = TestServer::new(router(state.clone())).unwrap();

        let declaration = UserDeclaration::new_valid("bob", "bob@example.com", "p4ssw0rd")?;
        let password = declaration.inner_as_ref().password.clone();
        let user = user::create(&db, declaration).await?;

        let expire = Some(time::Duration::seconds(5)); // something short, just in case
        let token1 = issue_access_token(
            &state,
            user.username.clone(),
            password.clone(),
            expire.clone(),
        )
        .await?;
        let token2 = issue_access_token(
            &state,
            user.username.clone(),
            password.clone(),
            expire.clone(),
        )
        .await?;

        // both work
        server
            .get("/verify")
            .add_header(AUTHORIZATION, bearer_auth_header(&token1))
            .await
            .assert_status_ok();
        server
            .get("/verify")
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_ok();

        let prefix1 = token1.chars().take(8).collect::<String>();
        let prefix2 = token2.chars().take(8).collect::<String>();
        assert_ne!(prefix1, prefix2); // I mean, it's unlikely, but still

        // empty token is not a valid path
        server
            .delete(format!("/api/sessions/{}", "").as_str())
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_not_found();

        // too short token
        server
            .delete(format!("/api/sessions/{}", "short").as_str())
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_bad_request();

        // if the token is not found, it's 404
        server
            .delete(format!("/api/sessions/{}", "does_not_exist").as_str())
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_not_found();

        // destroy token1 using token2
        server
            .delete(format!("/api/sessions/{}", prefix1).as_str())
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_ok();

        // token1 stops working, token2 still works
        server
            .get("/verify")
            .add_header(AUTHORIZATION, bearer_auth_header(&token1))
            .await
            .assert_status_unauthorized();
        server
            .get("/verify")
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_ok();

        // can't double revoke token1
        server
            .delete(format!("/api/sessions/{}", prefix1).as_str())
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_not_found();

        // use token2 to destroy itself
        server
            .delete(format!("/api/sessions/{}", prefix2).as_str())
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_ok();
        server
            .get("/verify")
            .add_header(AUTHORIZATION, bearer_auth_header(&token2))
            .await
            .assert_status_unauthorized();

        Ok(())
    }
}
