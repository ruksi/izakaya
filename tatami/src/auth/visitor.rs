use uuid::Uuid;

#[derive(Clone)]
pub struct Visitor {
    pub user_id: Option<Uuid>,
    pub access_token: Option<String>,
}

impl Visitor {
    pub fn is_anonymous(&self) -> bool {
        self.user_id.is_none()
    }
}
