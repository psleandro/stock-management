pub struct Workspace {
    pub id: i32,
    pub name: Option<String>,
    pub owner_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub struct CreateWorkspace {
    pub name: Option<String>,
    pub owner_id: i32,
}
