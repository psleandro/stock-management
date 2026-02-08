use axum::Router;
use axum::routing::post;
use std::sync::Arc;

use crate::app::AppState;
use crate::handlers::stock_movements::{create_stock_entry, create_stock_exit};

pub fn stock_movements_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/entry", post(create_stock_entry))
        .route("/exit", post(create_stock_exit))
}
