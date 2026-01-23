use std::env;

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{Response, IntoResponse}
};
use axum_extra::{
    headers::{authorization::Bearer,Authorization},
    TypedHeader
};

use crate::infrastructure::auth::jwt::{JwtClaims, JwtService};

impl <S> FromRequestParts<S> for JwtClaims where S: Send + Sync {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidToken)?;

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token_service = JwtService::new(jwt_secret);

        let jwt_claims = token_service.get_claims_from_token(bearer.token())
            .map_err(|_| Error::InvalidToken)?;

        Ok(jwt_claims)
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidToken,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, "Invalid token").into_response()
    }
}