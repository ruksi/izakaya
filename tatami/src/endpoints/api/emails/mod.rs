use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

use create::create;
use list::list;
mod create;
mod list;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        // .route("/:access_token_prefix", delete(destroy))
        .with_state(state)
}
