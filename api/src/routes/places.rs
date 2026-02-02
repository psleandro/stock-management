use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, patch, post};

use crate::app::AppState;
use crate::handlers::places::{create_place, delete_place, get_place, list_places, update_place};
pub fn places_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_places))
        .route("/{place_id}", get(get_place))
        .route("/", post(create_place))
        .route("/{place_id}", patch(update_place))
        .route("/{place_id}", delete(delete_place))
}
