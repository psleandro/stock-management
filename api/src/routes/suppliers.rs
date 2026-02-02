use axum::Router;
use axum::routing::{delete, get, patch, post};
use std::sync::Arc;

use crate::app::AppState;
use crate::handlers::suppliers::{
    create_supplier, delete_supplier, get_supplier, list_suppliers, update_supplier,
};

pub fn suppliers_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list_suppliers))
        .route("/{supplier_id}", get(get_supplier))
        .route("/", post(create_supplier))
        .route("/{supplier_id}", patch(update_supplier))
        .route("/{supplier_id}", delete(delete_supplier))
}
