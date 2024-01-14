use axum::routing::get;
use axum::Router;

use amend::*;
use create::*;
use describe::*;
use destroy::*;
use list::*;

use crate::state::AppState;

mod amend;
mod create;
mod describe;
mod destroy;
mod list;

pub fn router<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:user_id", get(describe).patch(amend).delete(destroy))
        .with_state(state)
}
