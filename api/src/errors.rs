use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum PayloadError {
    ValidationError(String),
    InvalidJson(String),
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Debug)]
pub enum InfrastructureError {
    Connection(String),
    Query(String),
    Timeout(String),
    Hashing(String),
    Unexpected(String),
}

pub enum ApplicationError {
    PayloadError(PayloadError),
    Auth(AuthError),
    Infrastructure(InfrastructureError),
    NotFound,
}

impl From<PayloadError> for ApplicationError {
    fn from(error: PayloadError) -> Self {
        ApplicationError::PayloadError(error)
    }
}

impl From<AuthError> for ApplicationError {
    fn from(error: AuthError) -> Self {
        ApplicationError::Auth(error)
    }
}

impl From<InfrastructureError> for ApplicationError {
    fn from(error: InfrastructureError) -> Self {
        ApplicationError::Infrastructure(error)
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            ApplicationError::PayloadError(error) => match error {
                PayloadError::ValidationError(e) => (StatusCode::BAD_REQUEST, e).into_response(),
                PayloadError::InvalidJson(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            },
            ApplicationError::Auth(error) => match error {
                AuthError::WrongCredentials => {
                    (StatusCode::UNAUTHORIZED, "Wrong Credentials").into_response()
                }
                AuthError::MissingCredentials => {
                    (StatusCode::BAD_REQUEST, "Missing credentials").into_response()
                }
                AuthError::TokenCreation => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error").into_response()
                }
                AuthError::InvalidToken => {
                    (StatusCode::BAD_REQUEST, "Invalid token").into_response()
                }
            },
            ApplicationError::Infrastructure(error) => {
                let error = match error {
                    InfrastructureError::Connection(e) => e,
                    InfrastructureError::Query(e) => e,
                    InfrastructureError::Timeout(e) => e,
                    InfrastructureError::Hashing(e) => e,
                    InfrastructureError::Unexpected(e) => e,
                };

                let error_message = format!("Infrastructure error: {}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            ApplicationError::NotFound => {
                (StatusCode::NOT_FOUND, "Resource Not Found").into_response()
            }
        }
    }
}
