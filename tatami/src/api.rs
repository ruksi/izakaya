use axum::routing::get;
use axum::Router;

use crate::state::AppState;
use crate::verify::verify;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/verify", get(verify))
        .nest("/settings", crate::settings::route::router(state.clone()))
        .nest("/users", crate::user::route::router(state.clone()))
        // everything under /api requires authentication (defined above)
        .route_layer(axum::middleware::from_fn(crate::auth::require_login))
}
