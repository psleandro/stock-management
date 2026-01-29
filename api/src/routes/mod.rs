use std::sync::Arc;
use axum::Router;
use crate::app::AppState;

pub mod auth;
pub mod products;

pub fn app_routes() -> Router<Arc<AppState>> {
	Router::new()
		.nest("/auth", auth::auth_routes())
		.nest("/products", products::products_routes())
}	