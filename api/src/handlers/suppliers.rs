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
    models::dto::supplier_dto::{CreateSupplierDto, ListSuppliersParams, UpdateSupplierDto},
    services::suppliers_service::SuppliersService,
};

pub async fn list_suppliers(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Query(params): Query<ListSuppliersParams>,
) -> Response {
    let suppliers_service = SuppliersService::new(state.db_pool.clone());
    let response = suppliers_service
        .list_suppliers(user.workspace_id, params)
        .await;

    match response {
        Ok(suppliers_list) => (StatusCode::OK, Json(suppliers_list)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn get_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    user: AuthenticatedUser,
) -> Response {
    let suppliers_service = SuppliersService::new(state.db_pool.clone());
    let response = suppliers_service.get_supplier(user.workspace_id, id).await;

    match response {
        Ok(supplier) => (StatusCode::OK, Json(supplier)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn create_supplier(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<CreateSupplierDto>,
) -> Response {
    let suppliers_service = SuppliersService::new(state.db_pool.clone());
    let response = suppliers_service
        .create_supplier(user.workspace_id, payload)
        .await;

    match response {
        Ok(created_supplier) => (StatusCode::CREATED, Json(created_supplier)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn update_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<UpdateSupplierDto>,
) -> Response {
    let suppliers_service = SuppliersService::new(state.db_pool.clone());
    let response = suppliers_service
        .update_supplier(user.workspace_id, id, payload)
        .await;

    match response {
        Ok(updated_supplier) => (StatusCode::OK, Json(updated_supplier)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn delete_supplier(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    user: AuthenticatedUser,
) -> Response {
    let suppliers_service = SuppliersService::new(state.db_pool.clone());
    let response = suppliers_service
        .delete_supplier(user.workspace_id, id as i32)
        .await;

    match response {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(error) => error.into_response(),
    }
}
