use axum::routing::get;
use axum::Router;

use amend::*;
use create::*;
use describe::*;
use describe_myself::*;
use destroy::*;
use list::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;
use crate::user::User;

mod amend;
mod create;
mod describe;
mod describe_myself;
mod destroy;
mod list;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:user_id", get(describe).patch(amend).delete(destroy))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::require_admin,
        ))
        .route("/me", get(describe_myself))
        .with_state(state)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserOut {
    pub user_id: Uuid,
    pub username: String,
}

impl From<User> for UserOut {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
        }
    }
}
