use axum::Router;

use crate::{auth, user};
use crate::state::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/users", user::route::router(state.clone()))
        // everything under API requires authentication
        .route_layer(axum::middleware::from_fn(auth::require_login))
}
