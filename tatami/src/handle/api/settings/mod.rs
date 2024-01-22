use axum::routing::get;
use axum::Router;

use describe_myself::describe_myself;

use crate::state::AppState;

mod describe_myself;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/profile", get(describe_myself))
        .with_state(state)
}
