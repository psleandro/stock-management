use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::AppState,
    extractors::{ValidatedJson, authenticated_user::AuthenticatedUser},
    models::dto::product_dto::{CreateProductDto, ListProductsParams, UpdateProductDto},
    services::products_service::ProductsService,
};

pub async fn list_products(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Query(params): Query<ListProductsParams>,
) -> Response {
    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service
        .list_products(user.workspace_id, params)
        .await;

    match response {
        Ok(products_list) => (StatusCode::OK, Json(products_list)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    user: AuthenticatedUser,
) -> Response {
    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service.get_product(user.workspace_id, id).await;

    match response {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<CreateProductDto>,
) -> Response {
    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service
        .create_product(user.workspace_id, payload)
        .await;

    match response {
        Ok(created_product) => (StatusCode::CREATED, Json(created_product)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<UpdateProductDto>,
) -> Response {
    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service
        .update_product(user.workspace_id, id, payload)
        .await;

    match response {
        Ok(updated_product) => (StatusCode::OK, Json(updated_product)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    user: AuthenticatedUser,
) -> Response {
    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service
        .delete_product(user.workspace_id, id as i32)
        .await;

    match response {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(error) => error.into_response(),
    }
}
