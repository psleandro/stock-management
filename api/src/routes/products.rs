use axum::Router;
use axum::routing::{delete, get, patch, post};
use std::sync::Arc;

use crate::app::AppState;
use crate::handlers::products::{
    create_product, delete_product, get_product, list_products, update_product,
};

pub fn products_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_products))
        .route("/{product_id}", get(get_product))
        .route("/", post(create_product))
        .route("/{product_id}", patch(update_product))
        .route("/{product_id}", delete(delete_product))
}
