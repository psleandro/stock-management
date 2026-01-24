use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::infrastructure::db::schema::{users, workspaces};

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

#[derive(Queryable, Selectable)]
#[diesel(table_name=workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct WorkspaceRow {
    pub id: i32,
    pub name: Option<String>,
    pub owner_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=workspaces)]
pub struct CreateWorkspaceRow {
    pub name: Option<String>,
    pub owner_id: i32,
}