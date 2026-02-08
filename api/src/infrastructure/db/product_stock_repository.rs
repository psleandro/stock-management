use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::dsl::sum;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::schema::stock_movements;

pub struct ProductStockRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl ProductStockRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn get_stock_by_product_id(
        &self,
        product_id: i32,
    ) -> Result<i64, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let product_stock: Option<i64> = connection
            .interact(move |conn| {
                stock_movements::table
                    .select(
                        sum(stock_movements::quantity)
                            .aggregate_filter(stock_movements::product_id.eq(product_id)),
                    )
                    .first(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(product_stock.unwrap_or(0))
    }
}
