use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::AppState,
    extractors::{ValidatedJson, authenticated_user::AuthenticatedUser},
    models::dto::stock_movement_dto::{StockMovementEntryDto, StockMovementExitDto},
};

pub async fn create_stock_entry(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<StockMovementEntryDto>,
) -> Response {
    let response = state
        .stock_movements_service
        .create_stock_entry(user.workspace_id, payload)
        .await;

    match response {
        Ok(created_stock_movement) => {
            (StatusCode::CREATED, Json(created_stock_movement)).into_response()
        }
        Err(error) => error.into_response(),
    }
}

pub async fn create_stock_exit(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<StockMovementExitDto>,
) -> Response {
    let response = state
        .stock_movements_service
        .create_stock_exit(user.workspace_id, payload)
        .await;

    match response {
        Ok(created_stock_movement) => {
            (StatusCode::CREATED, Json(created_stock_movement)).into_response()
        }
        Err(error) => error.into_response(),
    }
}
