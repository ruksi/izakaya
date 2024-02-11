use axum::extract::State;
use axum::{Extension, Json};

use crate::auth::CurrentUser;
use crate::endpoints::api::users::UserOut;
use crate::error::Error;
use crate::prelude::*;
use crate::state::AppState;
use crate::user;

pub async fn describe_myself(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<UserOut>> {
    let user_id = current_user.user_id;
    let user = user::describe(&state.db_pool, user_id).await?;
    let Some(user) = user else {
        tracing::error!("User {} could not find itself", user_id);
        return Err(Error::NotFound);
    };
    Ok(Json(user.into()))
}
