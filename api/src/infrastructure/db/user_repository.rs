use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::infrastructure::db::models::CreateUserRow;
use crate::infrastructure::db::models::UserRow;
use crate::infrastructure::db::schema::users;

use crate::infrastructure::db::schema::workspaces;
use crate::models::user::{ User, AuthUser, CreateUser };

pub struct UserRepository {
 	pub pool: Pool<Manager<PgConnection>>,
}

impl UserRepository {

	pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
		Self { pool }
	}

	pub async fn create(&self, user_payload: CreateUser) -> User {
		let connection = self.pool.get().await.unwrap();

        let create_user_row = CreateUserRow {
            name: user_payload.name,
            email: user_payload.email,
            password: user_payload.password,
        };

        let created_user = connection.interact(|conn| {
            diesel::insert_into(users::table)
                .values(create_user_row)
                .returning(UserRow::as_returning())
                .get_result::<UserRow>(conn)
        }).await.unwrap().unwrap();

		User {
			id: created_user.id,
			name: created_user.name,
			email: created_user.email,
			created_at: created_user.created_at.to_string(),
			updated_at: created_user.updated_at.to_string(),
			deleted_at: created_user.deleted_at.map(|d| d.to_string()),
		}
	}

    pub async fn get_user_by_email(&self, user_email: String) -> Option<AuthUser> {
        let connection = self.pool.get().await.unwrap();

        let user = connection.interact(|conn| {
            users::table
                .inner_join(workspaces::table.on(workspaces::owner_id.eq(users::id))) 
                .filter(users::email.eq(user_email))
                .select((users::all_columns, workspaces::id))
                .first::<(UserRow, i32)>(conn)
                .optional()
        }).await.unwrap().unwrap();
        
        user.map(|(u, workspace_id)| AuthUser {
            id: u.id,
			name: u.name,
            password_hash: u.password,
            workspace_id: workspace_id,
			email: u.email,
			created_at: u.created_at.to_string(),
			updated_at: u.updated_at.to_string(),
			deleted_at: u.deleted_at.map(|d| d.to_string()),
        })
    }
}