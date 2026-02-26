use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::models::dto::user_dto::SignIn;
use crate::{app::AppState, extractors::ValidatedJson};

pub async fn signin(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<SignIn>,
) -> Response {
    let sign_in_response = state.auth_service.signin(payload).await;

    match sign_in_response {
        Ok(sign_in_response) => (StatusCode::OK, Json(sign_in_response)).into_response(),
        Err(error) => error.into_response(),
    }
}
