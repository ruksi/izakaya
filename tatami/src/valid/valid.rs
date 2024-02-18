use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::{async_trait, Json};
use validator::Validate;

use crate::prelude::*;

// Valid a typestate that marks that the contained struct has been validated, and
// the only way to mutate the inner value is to remove the validated marker
// (don't use else with multiple ownership 🙏).
//
// You can use it in Axum request handlers through `Valid<T>` like:
// * `Valid(person): Valid<Person>`
// * or, if you want to keep the validated marker: `person: Valid<Person>`

pub struct Valid<T: Validate> {
    inner: ValidInner<T>,
}

impl<T: Validate + std::fmt::Debug> std::fmt::Debug for Valid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Valid").field(self.inner_as_ref()).finish()
    }
}

impl<T: Validate> Valid<T> {
    pub fn new(value: T) -> Result<Self> {
        value.validate()?;
        Ok(Self {
            inner: ValidInner(value),
        })
    }
    pub fn inner_as_ref(&self) -> &T {
        &self.inner.0 // "aka. 'I just want to have a peek.'"
    }
    pub fn into_inner(self) -> T {
        self.inner.0 // "aka. 'I want to consume this thing now, please.'"
    }
}

// private scope, so that the only way to create a `Valid<T>` is through `Valid::new`
struct ValidInner<T: Validate>(T);

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
