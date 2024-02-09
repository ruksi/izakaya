use uuid::Uuid;

use crate::auth::Visitor;
use crate::prelude::*;

// CurrentUser is a stricter version of a Visitor.
// A current user is always authenticated.

#[derive(Clone)]
pub struct CurrentUser {
    pub access_token: String,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

impl CurrentUser {
    pub fn from_visitor(visitor: Visitor) -> Result<Self> {
        let access_token = visitor.access_token.ok_or(Error::Unauthorized)?;
        let session_id = visitor.session_id.ok_or(Error::Unauthorized)?;
        let user_id = visitor.user_id.ok_or(Error::Unauthorized)?;
        Ok(Self {
            access_token,
            session_id,
            user_id,
        })
    }
}
