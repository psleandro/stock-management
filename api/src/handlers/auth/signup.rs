use crate::models::dto::user_dto::SignUp;
use crate::services::auth_service::AuthService;
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
    let auth_service = AuthService::new(state.db_pool.clone());

    let created_user = auth_service.signup(payload).await;

    match created_user {
        Ok(created_user) => (StatusCode::CREATED, Json(created_user)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())).into_response(),
    }
}
