use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::errors::{ApplicationError, DomainError};
use crate::infrastructure::db::places_repository::PlacesRepository;
use crate::infrastructure::db::product_stock_repository::ProductStockRepository;
use crate::infrastructure::db::products_repository::ProductsRepository;
use crate::infrastructure::db::stock_movements_repository::StockMovementsRepository;
use crate::infrastructure::db::suppliers_repository::SuppliersRepository;
use crate::models::dto::stock_movement_dto::{StockMovementEntryDto, StockMovementExitDto};
use crate::models::ids::WorkspaceId;
use crate::models::stock_movement::{StockMovement, StockMovementEntry, StockMovementExit};

pub struct StockMovementsService {
    pub products_repository: ProductsRepository,
    pub stock_movements_repository: StockMovementsRepository,
    pub product_stock_repository: ProductStockRepository,
    pub suppliers_repository: SuppliersRepository,
    pub places_repository: PlacesRepository,
}

impl StockMovementsService {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        let products_repository = ProductsRepository::new(pool.clone());
        let stock_movements_repository = StockMovementsRepository::new(pool.clone());
        let product_stock_repository = ProductStockRepository::new(pool.clone());
        let suppliers_repository = SuppliersRepository::new(pool.clone());
        let places_repository = PlacesRepository::new(pool.clone());
        Self {
            products_repository,
            stock_movements_repository,
            product_stock_repository,
            suppliers_repository,
            places_repository,
        }
    }

    pub async fn create_stock_entry(
        &self,
        workspace_id: WorkspaceId,
        payload: StockMovementEntryDto,
    ) -> Result<StockMovement, ApplicationError> {
        let product_id = self
            .products_repository
            .get_product_by_id(workspace_id, payload.product_id)
            .await?
            .ok_or(DomainError::ProductNotFound)?
            .id;

        let supplier_id = self
            .suppliers_repository
            .get_supplier_by_id(workspace_id, payload.supplier_id)
            .await?
            .ok_or(DomainError::SupplierNotFound)?
            .id;

        let create_stock_entry_data = StockMovementEntry {
            movement_date: payload.movement_date,
            product_id,
            supplier_id,
            quantity: payload.quantity,
            unit_cost_in_cents: payload.unit_cost_in_cents,
            invoice_number: payload.invoice_number,
            notes: payload.notes,
        };

        let created_stock_movement = self
            .stock_movements_repository
            .create_stock_entry(create_stock_entry_data)
            .await?;

        Ok(created_stock_movement)
    }

    pub async fn create_stock_exit(
        &self,
        workspace_id: WorkspaceId,
        payload: StockMovementExitDto,
    ) -> Result<StockMovement, ApplicationError> {
        let product_id = self
            .products_repository
            .get_product_by_id(workspace_id, payload.product_id)
            .await?
            .ok_or(DomainError::ProductNotFound)?
            .id;

        let place_id = self
            .places_repository
            .get_place_by_id(workspace_id, payload.place_id)
            .await?
            .ok_or(DomainError::PlaceNotFound)?
            .id;

        let product_stock_quantity = self
            .product_stock_repository
            .get_stock_by_product_id(product_id)
            .await?;

        if payload.quantity as i64 > product_stock_quantity {
            return Err(DomainError::NotEnoughStock)?;
        }

        let create_stock_exit_data = StockMovementExit {
            movement_date: payload.movement_date,
            product_id,
            place_id,
            quantity: payload.quantity * -1,
            notes: payload.notes,
        };

        let created_stock_movement = self
            .stock_movements_repository
            .create_stock_exit(create_stock_exit_data)
            .await?;

        Ok(created_stock_movement)
    }
}
