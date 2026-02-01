use std::env;

use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    errors::{ApplicationError, AuthError},
    infrastructure::auth::jwt::{JwtClaims, JwtService},
};

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token_service = JwtService::new(jwt_secret);

        let jwt_claims = token_service
            .get_claims_from_token(bearer.token())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(jwt_claims)
    }
}

#[cfg(test)]
mod tests {
    use axum::http::Request;

    use crate::models::user::AuthUser;

    use super::*;

    #[tokio::test]
    async fn extracts_claims_from_valid_bearer() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        let authenticated_user = AuthUser::mock();

        let token_service = JwtService::new("secret".to_string());

        let bearer_token = token_service
            .generate_token(&authenticated_user)
            .expect("Token should be generated");

        let (mut parts, state) = get_mocked_parts_and_state(&bearer_token);

        let result = JwtClaims::from_request_parts(&mut parts, &state).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn fails_when_authorization_header_is_missing() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        let request = Request::builder().body(()).unwrap();

        let mut parts = request.into_parts().0;
        let state = ();

        let error = JwtClaims::from_request_parts(&mut parts, &state)
            .await
            .unwrap_err();

        assert!(matches!(
            error,
            ApplicationError::Auth(AuthError::MissingToken)
        ));
    }

    #[tokio::test]
    async fn fails_when_token_is_invalid() {
        unsafe {
            std::env::set_var("JWT_SECRET", "secret");
        }

        let (mut parts, state) = get_mocked_parts_and_state("Invalid Token");

        let error = JwtClaims::from_request_parts(&mut parts, &state)
            .await
            .unwrap_err();

        assert!(matches!(
            error,
            ApplicationError::Auth(AuthError::InvalidToken)
        ));
    }

    fn get_mocked_parts_and_state(bearer_token: &str) -> (Parts, ()) {
        let request = Request::builder()
            .header("Authorization", format!("Bearer {}", bearer_token))
            .body(())
            .unwrap();

        let parts = request.into_parts().0;
        let state = ();

        (parts, state)
    }
}
