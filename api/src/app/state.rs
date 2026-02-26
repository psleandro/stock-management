use crate::services::{
    auth_service::AuthService, places_service::PlacesService, products_service::ProductsService,
    stock_movements_service::StockMovementsService, stock_service::StockService,
    suppliers_service::SuppliersService,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub places_service: PlacesService,
    pub products_service: ProductsService,
    pub stock_movements_service: StockMovementsService,
    pub stock_service: StockService,
    pub suppliers_service: SuppliersService,
}

impl AppState {
    pub fn new(
        auth_service: AuthService,
        places_service: PlacesService,
        products_service: ProductsService,
        stock_movements_service: StockMovementsService,
        stock_service: StockService,
        suppliers_service: SuppliersService,
    ) -> AppState {
        AppState {
            auth_service,
            places_service,
            products_service,
            stock_movements_service,
            stock_service,
            suppliers_service,
        }
    }
}
