use crate::app::AppState;
use axum::Router;
use std::sync::Arc;

pub mod auth;
pub mod places;
pub mod products;
pub mod suppliers;

pub fn app_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::auth_routes())
        .nest("/places", places::places_routes())
        .nest("/products", products::products_routes())
        .nest("/suppliers", suppliers::suppliers_routes())
}
