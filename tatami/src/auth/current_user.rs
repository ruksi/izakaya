use uuid::Uuid;

use crate::auth::Visitor;
use crate::prelude::*;

// CurrentUser is a stricter version of a Visitor.
// A current user is always authenticated.

#[derive(Clone)]
pub struct CurrentUser {
    pub user_id: Uuid,
    pub access_token: String,
}

impl CurrentUser {
    pub fn from_visitor(visitor: Visitor) -> Result<Self> {
        let user_id = visitor.user_id.ok_or(Error::Unauthorized)?;
        let access_token = visitor.access_token.ok_or(Error::Unauthorized)?;
        Ok(Self {
            user_id,
            access_token,
        })
    }
}
