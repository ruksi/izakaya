use axum::routing::get;
use axum::Router;

use describe_user::describe_user;

use crate::state::AppState;

mod describe_user;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/user", get(describe_user))
        .with_state(state)
}
