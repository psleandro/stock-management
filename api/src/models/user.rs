use serde::Serialize;

use crate::models::ids::{UserId, WorkspaceId};

#[derive(Serialize)]
pub struct User {
    pub id: UserId,
    pub name: Option<String>,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct AuthUser {
    pub id: UserId,
    pub name: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub workspace_id: WorkspaceId,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<AuthUser> for User {
    fn from(auth_user: AuthUser) -> Self {
        User {
            id: auth_user.id,
            name: auth_user.name,
            email: auth_user.email,
            created_at: auth_user.created_at.to_string(),
            updated_at: auth_user.updated_at.to_string(),
            deleted_at: auth_user.deleted_at.map(|d| d.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl AuthUser {
        pub fn mock() -> Self {
            AuthUser {
                id: UserId(1),
                name: Some("John Doe".to_string()),
                email: "teste@example.com".to_string(),
                password_hash: "$2b$12$examplehashforpassword1234567890".to_string(),
                workspace_id: WorkspaceId(1),
                created_at: "2026-01-31T12:00:00Z".to_string(),
                updated_at: "2026-01-31T12:00:00Z".to_string(),
                deleted_at: None,
            }
        }
    }
}
