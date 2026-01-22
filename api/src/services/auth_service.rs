use std::env;
use argon2::{
    Argon2, PasswordVerifier, password_hash::{Error, PasswordHasher, PasswordHash, SaltString}};
use rand::rngs::OsRng;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::{infrastructure::user_repository::UserRepository, models::dto::user_dto::SignInResponse};
use crate::infrastructure::auth::jwt::JwtService;
use crate::models::dto::user_dto::{SignUp, SignIn};
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

    pub async fn signin(&self, payload: SignIn) -> Result<SignInResponse, Box<dyn std::error::Error>>{
        let auth_user = self.user_repository.get_user_by_email(payload.email).await;

        if let Some(auth_user) = auth_user {
            let is_valid_password = AuthService::verify_password(
                &payload.password, &auth_user.password_hash
            );

            if is_valid_password {
                let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let token_service = JwtService::new(jwt_secret);

                let user: User = auth_user.into();

                let access_token = token_service.generate_token(&user)?;

                let sign_in_response = SignInResponse {
                    user,
                    access_token,
                };

                return Ok(sign_in_response)
            }
        }

        return Err("Invalid email or password".into());
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

    fn verify_password(password: &str, password_hash: &str) -> bool {
        let pepper: String = env::var("PASSWORD_PEPPER").expect("PASSWORD_PEPPER must be set");

        let mut password_peppered = String::with_capacity(password.len() + pepper.len());
        password_peppered.push_str(password);
        password_peppered.push_str(&pepper);

        let parsed_hash = match PasswordHash::new(password_hash) {
            Ok(h) => h,
            Err(_) => return false, 
        };

        let argon2 = Argon2::default();

        argon2.verify_password(password_peppered.as_bytes(), &parsed_hash).is_ok()
    }
}