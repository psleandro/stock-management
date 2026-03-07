pub mod state;

use std::sync::Arc;

use axum::Router;
pub use state::AppState;

use crate::{
    infrastructure::db::{
        places_repository::PlacesRepository, products_repository::ProductsRepository,
        stock_movements_repository::StockMovementsRepository,
        stock_repository::ProductStockRepository, suppliers_repository::SuppliersRepository,
        transaction::TransactionRunner, user_repository::UserRepository,
    },
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
    let transaction_runner = TransactionRunner::new(pool.clone());

    let places_repository = PlacesRepository::new(pool.clone());
    let products_repository = ProductsRepository::new(pool.clone());
    let suppliers_repository = SuppliersRepository::new(pool.clone());
    let user_repository = UserRepository::new(pool.clone());

    let product_stock_repository = ProductStockRepository::new(pool.clone());
    let stock_movements_repository = StockMovementsRepository::new(pool.clone());
    let stock_repository = ProductStockRepository::new(pool.clone());

    let auth_service = AuthService::new(user_repository, transaction_runner);
    let places_service = PlacesService::new(places_repository.clone());
    let products_service = ProductsService::new(products_repository.clone());
    let suppliers_service = SuppliersService::new(suppliers_repository.clone());

    let stock_service = StockService::new(stock_repository);
    let stock_movements_service = StockMovementsService::new(
        products_repository,
        stock_movements_repository,
        product_stock_repository,
        suppliers_repository,
        places_repository,
    );

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
