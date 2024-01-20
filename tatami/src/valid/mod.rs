use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::{async_trait, Json};
use validator::Validate;

pub use password::password;
pub use username::username;

use crate::prelude::*;

mod password;
mod username;

#[derive(Debug, Clone, Copy, Default)]
pub struct Valid<T: serde::de::DeserializeOwned + Validate>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Valid<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(Valid(value))
    }
}
