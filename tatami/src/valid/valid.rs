use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::{async_trait, Json};
use validator::Validate;

use crate::error;
use crate::error::Error;

// Valid a newtype that marks that the inner, JSON deserializable value
// has been validated, and the only way to mutate the inner value is to
// remove the validated marker (no Arc, no RefCell, no Mutex, no RwLock, please 🙏).
//
// You should be using it in the request handlers through the `Valid<T>` type like:
// * `Valid(my_json): Valid<MyJson>`
// * or, if you want to keep the validated marker: `my_json: Valid<MyJson>`
//
// This makes sure that the value both 1) is valid JSON and 2) passes Validator validation

#[derive(Debug, Clone, Copy, Default)]
pub struct Valid<T>(T)
where
    T: serde::de::DeserializeOwned + Validate;

impl<T> Valid<T>
where
    T: serde::de::DeserializeOwned + Validate,
{
    pub fn new(value: T) -> error::Result<Self> {
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

    async fn from_request(req: axum::extract::Request, state: &S) -> error::Result<Self> {
        let Json(thing_from_json) = Json::<T>::from_request(req, state).await?;
        let valid_thing_from_json = Valid::new(thing_from_json)?;
        Ok(valid_thing_from_json)
    }
}
