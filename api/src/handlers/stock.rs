use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::AppState, extractors::authenticated_user::AuthenticatedUser,
    models::dto::product_dto::ListProductsParams, services::stock_service::StockService,
};

pub async fn list_products_with_stock(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Query(params): Query<ListProductsParams>,
) -> Response {
    let stock_service = StockService::new(state.db_pool.clone());

    let response = stock_service
        .list_products_with_stock(user.workspace_id, params)
        .await;

    match response {
        Ok(products) => (StatusCode::CREATED, Json(products)).into_response(),
        Err(error) => error.into_response(),
    }
}
