use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use tower_cookies::Cookie;

use crate::auth::csrf::{create_csrf_token, is_valid_csrf_token};
use crate::auth::{cookie, Visitor};
use crate::prelude::*;
use crate::state::AppState;

pub async fn csrf_manager(
    State(state): State<AppState>,
    Extension(visitor): Extension<Visitor>,
    cookies: tower_cookies::Cookies,
    request: Request,
    next: Next,
) -> Result<Response> {
    if should_skip_csrf() {
        return Ok(next.run(request).await);
    }

    // TODO: skip CSRF manager if authenticated with a Bearer token 🐻

    // if the request is not "safe" (GET | HEAD | OPTIONS | TRACE),
    // require a CSRF token
    if !request.method().is_safe() {
        let Some(csrf_cookie) = cookies.get(cookie::CSRF_TOKEN) else {
            tracing::debug!("CSRF cookie missing");
            return Err(Error::Unauthorized);
        };
        let csrf_cookie_token = csrf_cookie.value();

        let csrf_header_token = request
            .headers()
            .get("CSRF-Token")
            .and_then(|value| value.to_str().ok());
        let Some(csrf_header_token) = csrf_header_token else {
            tracing::debug!("CSRF header missing");
            return Err(Error::Unauthorized);
        };

        if csrf_cookie_token != csrf_header_token {
            tracing::debug!("CSRF token mismatch");
            return Err(Error::Unauthorized);
        }

        if !is_valid_csrf_token(
            &state.config.csrf_secret,
            visitor.session_id,
            csrf_cookie_token,
        ) {
            tracing::debug!("CSRF token invalid");
            // Chrome doesn't apply cookies coming from a 401 response
            // if we just do a `cookies.remove()` 🤷
            let cookie = cookie::bake_for_frontend(
                cookie::CSRF_TOKEN,
                "".to_string(),
                state.config.cookie_domain,
                time::Duration::ZERO,
            );
            cookies.add(cookie);
            return Err(Error::Unauthorized);
        }
    }

    // we _could_ move this after "next" but the auth state (`visitor`) might be incorrect
    let csrf_cookie = cookies.get(cookie::CSRF_TOKEN);
    if csrf_cookie.is_none() {
        let csrf_token = create_csrf_token(&state.config.csrf_secret, visitor.session_id);
        let csrf_cookie = cookie::bake_for_frontend(
            cookie::CSRF_TOKEN,
            csrf_token,
            state.config.cookie_domain,
            time::Duration::days(14),
        );
        cookies.add(csrf_cookie);
    }

    Ok(next.run(request).await)
}

fn should_skip_csrf() -> bool {
    // having this function allows us to skip CSRF checks in tests,
    // but having this inline will mess my IDE :(
    // TODO: move to Config so we can override in a few tests
    #[cfg(test)]
    {
        return true;
    }
    false
}
