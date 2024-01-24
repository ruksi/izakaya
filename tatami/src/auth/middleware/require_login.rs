use axum::Extension;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::auth::{CurrentUser, Visitor};
use crate::prelude::*;

pub async fn require_login(
    Extension(visitor): Extension<Visitor>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let current_user = CurrentUser::from_visitor(visitor)?;
    request.extensions_mut().insert(current_user);
    Ok(next.run(request).await)
}
