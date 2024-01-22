use axum::extract::State;
use axum::{Extension, Json};
use serde_json::{json, Value};

use crate::auth::Visitor;
use crate::error::Error;
use crate::state::AppState;
use crate::user;

pub async fn describe_myself(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
) -> crate::error::Result<Json<Value>> {
    let user_id = visitor.get_user_id_or_respond_unauthorized()?;
    let user = user::describe(&state.db_pool, user_id).await?;
    let Some(user) = user else {
        tracing::error!("User {} could not find itself", user_id);
        return Err(Error::NotFound);
    };
    Ok(Json(json!({
        "userId": user.user_id,
        "username": user.username,
    })))
}
