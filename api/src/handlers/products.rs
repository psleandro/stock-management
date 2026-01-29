use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::AppState,
    extractors::ValidatedJson,
    infrastructure::auth::jwt::JwtClaims,
    models::dto::product_dto::{CreateProductDto, ListProductsParams, UpdateProductDto},
    services::products_service::ProductsService,
};

pub async fn list_products(
    State(state): State<Arc<AppState>>,
    jwt_claims: JwtClaims,
    Query(params): Query<ListProductsParams>,
) -> Response {
    let workspace_id = jwt_claims.workspace_id.parse::<i32>().unwrap();

    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service.list_products(workspace_id, params).await;

    match response {
        Ok(products_list) => (StatusCode::OK, Json(products_list)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())).into_response(),
    }
}

pub async fn get_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    jwt_claims: JwtClaims,
) -> Response {
    let user_id = jwt_claims.sub.parse::<i32>().unwrap();

    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service.get_product(user_id, id).await;

    match response {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    jwt_claims: JwtClaims,
    ValidatedJson(payload): ValidatedJson<CreateProductDto>,
) -> Response {
    let workspace_id = jwt_claims.workspace_id.parse::<i32>().unwrap();

    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service.create_product(workspace_id, payload).await;

    match response {
        Ok(created_product) => (StatusCode::CREATED, Json(created_product)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())).into_response(),
    }
}

pub async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    jwt_claims: JwtClaims,
    ValidatedJson(payload): ValidatedJson<UpdateProductDto>,
) -> Response {
    let user_id = jwt_claims.sub.parse::<i32>().unwrap();

    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service.update_product(user_id, id, payload).await;

    match response {
        Ok(updated_product) => (StatusCode::OK, Json(updated_product)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())).into_response(),
    }
}

pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    jwt_claims: JwtClaims,
) -> Response {
    let user_id = jwt_claims.sub.parse::<i32>().unwrap();

    let products_service = ProductsService::new(state.db_pool.clone());
    let response = products_service.delete_product(user_id, id as i32).await;

    match response {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string())).into_response(),
    }
}
