use axum::extract::{FromRequest, Json, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::{ApplicationError, PayloadError};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| PayloadError::InvalidJson(e.to_string()))?;

        value
            .validate()
            .map_err(|e| PayloadError::ValidationError(e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use serde::Deserialize;
    use validator::Validate;

    #[derive(Debug, Deserialize, Validate)]
    struct TestData {
        #[validate(length(min = 1, message = "cannot be empty"))]
        name: String,
        #[validate(range(min = 1, message = "must be positive"))]
        age: u8,
    }

    async fn extract_validated_json<T: DeserializeOwned + Validate>(
        json_str: &str,
    ) -> Result<ValidatedJson<T>, ApplicationError> {
        let req = Request::builder()
            .method("POST")
            .uri("/")
            .header("content-type", "application/json")
            .body(Body::from(json_str.to_owned()))
            .unwrap();

        ValidatedJson::<T>::from_request(req, &()).await
    }

    #[tokio::test]
    async fn extracts_valid_json_successfully() {
        let json = r#"{ "name": "John", "age": 30 }"#;
        let result = extract_validated_json::<TestData>(json).await;
        assert!(result.is_ok());
        let validated = result.unwrap();
        assert_eq!(validated.0.name, "John");
        assert_eq!(validated.0.age, 30);
    }

    #[tokio::test]
    async fn fails_on_invalid_json_format() {
        let json = r#"name": "John";"age": "not_a_number"#;
        let result = extract_validated_json::<TestData>(json).await;
        assert!(matches!(result, Err(ApplicationError::PayloadError(_))));
    }

    #[tokio::test]
    async fn fails_when_validation_fails() {
        let json = r#"{ "name": "", "age": 0 }"#;
        let result = extract_validated_json::<TestData>(json).await;
        assert!(matches!(result, Err(ApplicationError::PayloadError(_))));
    }
}
