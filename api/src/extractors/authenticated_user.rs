use axum::extract::FromRequestParts;

use crate::{
    errors::{ApplicationError, AuthError},
    infrastructure::auth::jwt::JwtClaims,
};

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub workspace_id: i32,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Sync + Send,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jwt_claims = JwtClaims::from_request_parts(parts, state).await?;

        let workspace_id = jwt_claims
            .workspace_id
            .parse::<i32>()
            .map_err(|_| AuthError::InvalidToken)?;

        let user_id = jwt_claims
            .sub
            .parse::<i32>()
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(Self {
            workspace_id,
            user_id,
        })
    }
}
