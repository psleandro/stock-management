use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{Error, PasswordHash, PasswordHasher, SaltString},
};
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use rand::rngs::OsRng;
use std::env;

use crate::models::dto::user_dto::{SignIn, SignUp};
use crate::models::user::{CreateUser, User};
use crate::models::workspace::CreateWorkspace;
use crate::{errors::ApplicationError, infrastructure::db::user_repository::UserRepository};
use crate::{errors::AuthError, infrastructure::auth::jwt::JwtService};
use crate::{
    errors::InfrastructureError, infrastructure::db::workspace_repository::WorkspaceRepository,
};
use crate::{
    infrastructure::db::transaction::TransactionRunner, models::dto::user_dto::SignInResponse,
};

#[derive(Clone)]
pub struct AuthService {
    user_repository: UserRepository,
    transaction_runner: TransactionRunner,
}

impl AuthService {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        let user_repository = UserRepository::new(pool.clone());
        let transaction_runner = TransactionRunner::new(pool.clone());
        Self {
            user_repository,
            transaction_runner,
        }
    }

    pub async fn signup(&self, payload: SignUp) -> Result<User, ApplicationError> {
        let password_hash = self
            .hash_password(&payload.password)
            .map_err(|e| InfrastructureError::Hashing(e.to_string()))?;

        let new_user_payload = CreateUser {
            name: payload.name,
            email: payload.email,
            password: password_hash,
        };

        let created_user = self
            .transaction_runner
            .run(|conn| {
                let created_user = UserRepository::create_user_in_tx(conn, new_user_payload)?;

                let new_workspace_payload = CreateWorkspace {
                    name: None,
                    owner_id: created_user.id,
                };

                WorkspaceRepository::create_workspace_in_tx(conn, new_workspace_payload)?;

                Ok(created_user)
            })
            .await?;

        Ok(created_user)
    }

    pub async fn signin(&self, payload: SignIn) -> Result<SignInResponse, ApplicationError> {
        let auth_user = self
            .user_repository
            .get_user_by_email(payload.email)
            .await?;

        if let Some(auth_user) = auth_user {
            let is_valid_password =
                self.verify_password(&payload.password, &auth_user.password_hash);

            if is_valid_password {
                let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let token_service = JwtService::new(jwt_secret);

                let access_token = token_service
                    .generate_token(&auth_user)
                    .map_err(|_| AuthError::TokenCreation)?;

                let user: User = auth_user.into();

                let sign_in_response = SignInResponse { user, access_token };

                return Ok(sign_in_response);
            }
        }

        return Err(AuthError::WrongCredentials)?;
    }

    fn hash_password(&self, password: &str) -> Result<String, Error> {
        let pepper: String = env::var("PASSWORD_PEPPER").expect("PASSWORD_PEPPER must be set");

        let mut password_peppered = String::with_capacity(password.len() + pepper.len());
        password_peppered.push_str(password);
        password_peppered.push_str(&pepper);

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(password_peppered.as_bytes(), &salt)?
            .to_string();

        Ok(hashed_password)
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        let pepper: String = env::var("PASSWORD_PEPPER").expect("PASSWORD_PEPPER must be set");

        let mut password_peppered = String::with_capacity(password.len() + pepper.len());
        password_peppered.push_str(password);
        password_peppered.push_str(&pepper);

        let parsed_hash = match PasswordHash::new(password_hash) {
            Ok(h) => h,
            Err(_) => return false,
        };

        let argon2 = Argon2::default();

        argon2
            .verify_password(password_peppered.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
