use axum::extract::FromRequestParts;

use crate::{
    errors::{ApplicationError, AuthError},
    infrastructure::auth::jwt::JwtClaims,
    models::ids::{UserId, WorkspaceId},
};

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub workspace_id: WorkspaceId,
}

impl AuthenticatedUser {
    fn try_from_claims(claims: JwtClaims) -> Result<Self, ApplicationError> {
        let user_id = claims
            .sub
            .parse::<i32>()
            .map_err(|_| AuthError::InvalidToken)?;

        let workspace_id = claims
            .workspace_id
            .parse::<i32>()
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(Self {
            user_id: UserId(user_id),
            workspace_id: WorkspaceId(workspace_id),
        })
    }
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

        Self::try_from_claims(jwt_claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_authenticated_user_from_jwt_claims() {
        let jwt_claims = get_mocked_jwt_claims("123", "456");

        let authenticated_user =
            AuthenticatedUser::try_from_claims(jwt_claims).expect("Extractor should succeed");

        assert_eq!(authenticated_user.user_id.value(), 123);
        assert_eq!(authenticated_user.workspace_id.value(), 456);
    }

    #[test]
    fn fails_when_sub_is_not_an_integer() {
        let jwt_claims = get_mocked_jwt_claims("abc", "456");

        let extract_response = AuthenticatedUser::try_from_claims(jwt_claims).unwrap_err();

        assert!(matches!(
            extract_response,
            ApplicationError::Auth(AuthError::InvalidToken)
        ));
    }

    #[test]
    fn fails_when_workspace_id_is_not_an_integer() {
        let jwt_claims = get_mocked_jwt_claims("123", "def");

        let error = AuthenticatedUser::try_from_claims(jwt_claims).unwrap_err();

        assert!(matches!(
            error,
            ApplicationError::Auth(AuthError::InvalidToken)
        ));
    }

    fn get_mocked_jwt_claims(sub: &str, workspace_id: &str) -> JwtClaims {
        JwtClaims {
            sub: sub.to_string(),
            workspace_id: workspace_id.to_string(),
            exp: 1,
            iat: 1,
        }
    }
}
