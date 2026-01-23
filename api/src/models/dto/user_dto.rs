use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::models::user::User;

#[derive(Deserialize, Validate)]
pub struct SignUp {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: String,

    #[validate(email(message = "invalid format"))]
    pub email: String,

    #[validate(length(min = 8, message = "must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct SignIn {
    #[validate(email(message = "invalid format"))]
    pub email: String,

    pub password: String,
}

#[derive(Serialize)]
pub struct SignInResponse {
    pub access_token: String,
    pub user: User,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}