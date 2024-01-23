use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;

use crate::auth::Visitor;
use crate::prelude::*;

pub async fn require_login(
    Extension(visitor): Extension<Visitor>,
    request: Request,
    next: Next,
) -> Result<Response> {
    if visitor.is_anonymous() {
        return Err(Error::Unauthorized);
    }
    Ok(next.run(request).await)
}
