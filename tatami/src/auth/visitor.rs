use uuid::Uuid;

use crate::prelude::*;

#[derive(Clone)]
pub struct Visitor {
    pub user_id: Option<Uuid>,
    pub access_token: Option<String>,
}

impl Visitor {
    pub fn is_anonymous(&self) -> bool {
        self.user_id.is_none()
    }
    pub fn get_user_id_or_respond_unauthorized(&self) -> Result<Uuid> {
        self.user_id.ok_or(Error::Unauthorized)
    }
}
