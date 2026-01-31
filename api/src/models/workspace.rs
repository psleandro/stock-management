use crate::models::ids::{UserId, WorkspaceId};

pub struct Workspace {
    pub id: WorkspaceId,
    pub name: Option<String>,
    pub owner_id: UserId,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub struct CreateWorkspace {
    pub name: Option<String>,
    pub owner_id: UserId,
}
