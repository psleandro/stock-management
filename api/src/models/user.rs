use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i32,
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