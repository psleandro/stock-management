use crate::models::dto::user_dto::SignUp;
use crate::{app::AppState, extractors::ValidatedJson};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

pub async fn signup(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<SignUp>,
) -> Response {
    let created_user = state.auth_service.signup(payload).await;

    match created_user {
        Ok(created_user) => (StatusCode::CREATED, Json(created_user)).into_response(),
        Err(error) => error.into_response(),
    }
}
