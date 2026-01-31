use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::CreateUserRow;
use crate::infrastructure::db::models::UserRow;
use crate::infrastructure::db::schema::users;

use crate::infrastructure::db::schema::workspaces;
use crate::models::ids::{UserId, WorkspaceId};
use crate::models::user::{AuthUser, CreateUser, User};

pub struct UserRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl UserRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user_payload: CreateUser) -> Result<User, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let created_user = connection
            .interact(|conn| Self::insert_user(conn, user_payload))
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(Self::to_domain(created_user))
    }

    pub fn create_user_in_tx(
        conn: &mut PgConnection,
        user_payload: CreateUser,
    ) -> Result<User, diesel::result::Error> {
        let created_user = Self::insert_user(conn, user_payload)?;
        Ok(Self::to_domain(created_user))
    }

    pub async fn get_user_by_email(
        &self,
        user_email: String,
    ) -> Result<Option<AuthUser>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let user = connection
            .interact(|conn| {
                users::table
                    .inner_join(workspaces::table.on(workspaces::owner_id.eq(users::id)))
                    .filter(users::email.eq(user_email))
                    .select((users::all_columns, workspaces::id))
                    .first::<(UserRow, i32)>(conn)
                    .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(user.map(|(u, workspace_id)| AuthUser {
            id: UserId(u.id),
            name: u.name,
            password_hash: u.password,
            workspace_id: WorkspaceId(workspace_id),
            email: u.email,
            created_at: u.created_at.to_string(),
            updated_at: u.updated_at.to_string(),
            deleted_at: u.deleted_at.map(|d| d.to_string()),
        }))
    }

    fn insert_user(
        conn: &mut PgConnection,
        user_payload: CreateUser,
    ) -> Result<UserRow, diesel::result::Error> {
        let create_user_row = CreateUserRow {
            name: user_payload.name,
            email: user_payload.email,
            password: user_payload.password,
        };

        diesel::insert_into(users::table)
            .values(create_user_row)
            .returning(UserRow::as_returning())
            .get_result::<UserRow>(conn)
    }

    fn to_domain(row: UserRow) -> User {
        User {
            id: UserId(row.id),
            name: row.name,
            email: row.email,
            created_at: row.created_at.to_string(),
            updated_at: row.updated_at.to_string(),
            deleted_at: row.deleted_at.map(|d| d.to_string()),
        }
    }
}
