use axum::Router;

use crate::state::AppState;
use crate::user;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/users", user::routes::router(state.clone()))
}
