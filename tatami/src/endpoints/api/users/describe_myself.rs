use axum::extract::State;
use axum::{Extension, Json};

use crate::auth::CurrentUser;
use crate::error::Error;
use crate::state::AppState;
use crate::user;
use crate::user::User;

pub async fn describe_myself(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> crate::error::Result<Json<User>> {
    let user_id = current_user.user_id;
    let user = user::describe(&state.db_pool, user_id).await?;
    let Some(user) = user else {
        tracing::error!("User {} could not find itself", user_id);
        return Err(Error::NotFound);
    };
    Ok(Json(user))
}
