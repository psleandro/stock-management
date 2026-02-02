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
    models::dto::place_dto::{CreatePlaceDto, ListPlacesParams, UpdatePlaceDto},
    services::places_service::PlacesService,
};

pub async fn list_places(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    Query(params): Query<ListPlacesParams>,
) -> Response {
    let places_service = PlacesService::new(state.db_pool.clone());
    let response = places_service.list_places(user.workspace_id, params).await;

    match response {
        Ok(places_list) => (StatusCode::OK, Json(places_list)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn get_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    user: AuthenticatedUser,
) -> Response {
    let places_service = PlacesService::new(state.db_pool.clone());
    let response = places_service.get_place(user.workspace_id, id).await;

    match response {
        Ok(place) => (StatusCode::OK, Json(place)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn create_place(
    State(state): State<Arc<AppState>>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<CreatePlaceDto>,
) -> Response {
    let places_service = PlacesService::new(state.db_pool.clone());
    let response = places_service
        .create_place(user.workspace_id, payload)
        .await;

    match response {
        Ok(created_place) => (StatusCode::CREATED, Json(created_place)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn update_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    user: AuthenticatedUser,
    ValidatedJson(payload): ValidatedJson<UpdatePlaceDto>,
) -> Response {
    let places_service = PlacesService::new(state.db_pool.clone());
    let response = places_service
        .update_place(user.workspace_id, id, payload)
        .await;

    match response {
        Ok(updated_place) => (StatusCode::OK, Json(updated_place)).into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn delete_place(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    user: AuthenticatedUser,
) -> Response {
    let places_service = PlacesService::new(state.db_pool.clone());
    let response = places_service
        .delete_place(user.workspace_id, id as i32)
        .await;

    match response {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(error) => error.into_response(),
    }
}
