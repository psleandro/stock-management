use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::infrastructure::db::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
	pub password: String,
	pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct CreateUserRow {
    pub name: String,
    pub email: String,
	pub password: String,
}