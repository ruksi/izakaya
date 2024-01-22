use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::{async_trait, Json};
use validator::Validate;

pub use password::password;
pub use username::username;

use crate::prelude::*;

mod password;
mod username;

// a newtype that marks that the inner, JSON deserializable value
// has been validated, and the only way to consume the thing is to
// unwrap the validated marker

#[derive(Debug, Clone, Copy, Default)]
pub struct Valid<T: serde::de::DeserializeOwned + Validate>(T);

impl<T: serde::de::DeserializeOwned + Validate> Valid<T> {
    pub fn new(value: T) -> Result<Self> {
        value.validate()?;
        Ok(Self(value))
    }
    pub fn inner_as_ref(&self) -> &T {
        &self.0 // "aka. 'I just want to have a peek.'"
    }
    pub fn into_inner(self) -> T {
        self.0 // "aka. 'I want to consume this thing now, please.'"
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for Valid<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = Error;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self> {
        let Json(thing_from_json) = Json::<T>::from_request(req, state).await?;
        let valid_thing_from_json = Valid::new(thing_from_json)?;
        Ok(valid_thing_from_json)
    }
}
