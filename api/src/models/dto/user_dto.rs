use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SignUp {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: String,

    #[validate(email(message = "invalid format"))]
    pub email: String,

    #[validate(length(min = 8, message = "must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}