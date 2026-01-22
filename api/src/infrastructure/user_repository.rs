use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::infrastructure::models::CreateUserRow;
use crate::infrastructure::models::UserRow;
use crate::infrastructure::schema::users;

use crate::models::user::{ User, CreateUser };

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
}