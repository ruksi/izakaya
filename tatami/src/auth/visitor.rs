use uuid::Uuid;

// Visitor is a less restrictive concept than a CurrentUser.
// A visitor is not necessarily authenticated; and this collects
// the information that we can get from a visitor.

#[derive(Clone)]
pub struct Visitor {
    pub user_id: Option<Uuid>,
    pub access_token: Option<String>,
}
