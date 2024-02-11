use axum::routing::{delete, get};
use axum::Router;

use crate::state::AppState;

use create::create;
use destroy::destroy;
use list::list;
mod create;
mod destroy;
mod list;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:email_id", delete(destroy))
        .with_state(state)
}
