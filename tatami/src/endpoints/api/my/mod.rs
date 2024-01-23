use axum::routing::get;
use axum::Router;

use describe_user::describe_user;
use list_sessions::list_sessions;

use crate::state::AppState;

mod describe_user;
mod list_sessions;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/user", get(describe_user))
        .route("/sessions", get(list_sessions))
        .with_state(state)
}
