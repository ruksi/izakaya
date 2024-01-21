pub mod route {
    use axum::extract::State;
    use axum::routing::get;
    use axum::{Extension, Json, Router};
    use serde_json::{json, Value};

    use crate::auth::Visitor;
    use crate::error::Error;
    use crate::state::AppState;

    pub fn router<S>(state: AppState) -> Router<S> {
        Router::new()
            .route("/profile", get(get_profile))
            .with_state(state)
    }

    pub async fn get_profile(
        State(state): State<AppState>,
        Extension(visitor): Extension<Visitor>,
    ) -> crate::error::Result<Json<Value>> {
        let Some(user_id) = visitor.user_id else {
            return Err(Error::Unauthorized);
        };
        let user = crate::user::model::describe(&state.db_pool, user_id).await?;
        let Some(user) = user else {
            tracing::error!("User {} could not find itself", user_id);
            return Err(Error::NotFound);
        };
        Ok(Json(json!({
            "userId": user.user_id,
            "username": user.username,
        })))
    }
}
