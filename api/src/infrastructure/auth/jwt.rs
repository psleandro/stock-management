use chrono::{Duration, Utc};
use jsonwebtoken::{ DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error };
use serde::{Deserialize, Serialize};

use crate::models::user::User;

pub struct JwtService {
    jwt_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    sub: String,
    iat: i64,
    exp: i64,
}

impl JwtService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn generate_token(&self, user: &User) -> Result<String, Error> {

        let now = Utc::now();

        let issued_at = now.timestamp();
        let expiration = (now + Duration::hours(1)).timestamp();

        let claims = JwtClaims {
            sub: user.id.to_string(),
            iat: issued_at,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )?;

        Ok(token)
    }

    pub fn get_claims_from_token(&self, access_token: &str) -> Result<JwtClaims, Error> {
        let token_data = decode::<JwtClaims>(
            &access_token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}