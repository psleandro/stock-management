pub mod state;

use std::sync::Arc;

use axum::Router;
pub use state::AppState;

use crate::{
    routes::app_routes,
    services::{
        auth_service::AuthService, places_service::PlacesService,
        products_service::ProductsService, stock_movements_service::StockMovementsService,
        stock_service::StockService, suppliers_service::SuppliersService,
    },
};

pub fn build_app(
    pool: deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::PgConnection>>,
) -> Router {
    let auth_service = AuthService::new(pool.clone());
    let places_service = PlacesService::new(pool.clone());
    let products_service = ProductsService::new(pool.clone());
    let stock_movements_service = StockMovementsService::new(pool.clone());
    let stock_service = StockService::new(pool.clone());
    let suppliers_service = SuppliersService::new(pool.clone());

    let app_state = Arc::new(AppState::new(
        auth_service,
        places_service,
        products_service,
        stock_movements_service,
        stock_service,
        suppliers_service,
    ));

    app_routes().with_state(app_state)
}
