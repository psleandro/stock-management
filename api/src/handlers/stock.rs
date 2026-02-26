use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::AppState, extractors::authenticated_user::AuthenticatedUser,
    models::dto::product_dto::ListProductsParams,
};

pub async fn list_products_with_stock(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Query(params): Query<ListProductsParams>,
) -> Response {
    let response = state
        .stock_service
        .list_products_with_stock(user.workspace_id, params)
        .await;

    match response {
        Ok(products) => (StatusCode::CREATED, Json(products)).into_response(),
        Err(error) => error.into_response(),
    }
}
