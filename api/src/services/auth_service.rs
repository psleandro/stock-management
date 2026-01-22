use std::env;
use argon2::{
    Argon2, password_hash::{Error, PasswordHasher, SaltString}};
use rand::rngs::OsRng;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::infrastructure::user_repository::UserRepository;
use crate::models::dto::user_dto::SignUp;
use crate::models::user::{User, CreateUser};

pub struct AuthService {
    user_repository: UserRepository,
}

impl AuthService {

    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        let user_repository = UserRepository::new(pool);
        Self { user_repository }
    }

    pub async fn signup(&self, payload: SignUp) -> Result<User, Error> {

        let password_hash = AuthService::hash_password(&payload.password)?;

        let new_user_payload = CreateUser {
            name: payload.name,
            email: payload.email,
            password: password_hash,
        };

        let created_user = self.user_repository.create(new_user_payload).await;

        Ok(created_user)
    }


    fn hash_password(password: &str) -> Result<String, Error> {
        let pepper: String = env::var("PASSWORD_PEPPER").expect("PASSWORD_PEPPER must be set");

        let mut password_peppered = String::with_capacity(password.len() + pepper.len());
        password_peppered.push_str(password);
        password_peppered.push_str(&pepper);

        let salt =  SaltString::generate(&mut OsRng);


        let argon2 = Argon2::default();

        let hashed_password = argon2.hash_password(password_peppered.as_bytes(), &salt)?.to_string();

        Ok(hashed_password)
    }
}