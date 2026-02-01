use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};

use crate::models::user::AuthUser;

pub struct JwtService {
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub workspace_id: String,
    pub iat: i64,
    pub exp: i64,
}

impl JwtService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn generate_token(&self, user: &AuthUser) -> Result<String, Error> {
        let now = Utc::now();

        let issued_at = now.timestamp();
        let expiration = (now + Duration::hours(1)).timestamp();

        let claims = JwtClaims {
            sub: user.id.to_string(),
            workspace_id: user.workspace_id.to_string(),
            iat: issued_at,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_token_with_correct_claims() {
        let mocked_user = AuthUser::mock();
        let jwt_secret = String::from("S3CR3T");
        let service = JwtService::new(jwt_secret);

        let token = service
            .generate_token(&mocked_user)
            .expect("Token should be generated");

        let token_claims = service
            .get_claims_from_token(&token)
            .expect("Claims should be decoded");

        assert_eq!(token_claims.sub, mocked_user.id.value().to_string());

        assert_eq!(
            token_claims.workspace_id,
            mocked_user.workspace_id.value().to_string()
        )
    }

    #[test]
    fn fails_to_decode_token_with_different_secret() {
        let mocked_user = AuthUser::mock();
        let jwt_secret = String::from("S3CR3T");

        let service_secret = JwtService::new(jwt_secret);

        let token_secret = service_secret.generate_token(&mocked_user).unwrap();

        let service_with_other_secret = JwtService::new("different_secret".to_string());

        let claims_result = service_with_other_secret.get_claims_from_token(&token_secret);

        assert!(claims_result.is_err());
    }
}
