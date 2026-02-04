use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::{StockMovementEntryRow, StockMovementRow};
use crate::infrastructure::db::schema::stock_movements;
use crate::models::stock_movement::{StockMovement, StockMovementEntry};

pub struct StockMovementsRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl StockMovementsRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create_stock_entry(
        &self,
        new_stock_movement_entry: StockMovementEntry,
    ) -> Result<StockMovement, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let create_stock_movement_row = StockMovementEntryRow {
            movement_date: new_stock_movement_entry.movement_date,
            product_id: new_stock_movement_entry.product_id,
            supplier_id: Some(new_stock_movement_entry.supplier_id),
            quantity: new_stock_movement_entry.quantity,
            unit_cost_in_cents: Some(new_stock_movement_entry.unit_cost_in_cents),
            invoice_number: Some(new_stock_movement_entry.invoice_number),
            notes: new_stock_movement_entry.notes,
        };

        let created_stock_movement = connection
            .interact(move |conn| {
                diesel::insert_into(stock_movements::table)
                    .values(create_stock_movement_row)
                    .returning(StockMovementRow::as_returning())
                    .get_result::<StockMovementRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(StockMovement {
            id: created_stock_movement.id,
            movement_date: created_stock_movement.movement_date,
            product_id: created_stock_movement.product_id,
            supplier_id: created_stock_movement.supplier_id,
            place_id: created_stock_movement.place_id,
            quantity: created_stock_movement.quantity,
            unit_cost_in_cents: created_stock_movement.unit_cost_in_cents,
            invoice_number: created_stock_movement.invoice_number,
            notes: created_stock_movement.notes,
            created_at: created_stock_movement.created_at,
            deleted_at: created_stock_movement.deleted_at,
        })
    }
}
