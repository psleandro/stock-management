use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::dsl::sum;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::ProductRow;
use crate::infrastructure::db::schema::{products, stock_movements};
use crate::models::ids::WorkspaceId;
use crate::models::product::{Product, Stock};

pub struct ProductStockRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl ProductStockRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_products_with_stock(
        &self,
        workspace_id: WorkspaceId,
        search: &str,
    ) -> Result<Vec<(Product, Stock)>, InfrastructureError> {
        let search = search.to_string();

        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let products = connection
            .interact(move |conn| {
                let search_like = format!("%{}%", search);

                let mut products_query = products::table
                    .filter(products::deleted_at.is_null())
                    .filter(products::workspace_id.eq(workspace_id.value()))
                    .into_boxed();

                let filter_expression = products::name
                    .ilike(&search_like)
                    .or(products::brand.ilike(&search_like))
                    .or(products::observation.ilike(&search_like))
                    .or(products::unit.ilike(&search_like));

                if let Ok(search_number) = search.parse::<i32>() {
                    products_query = products_query.filter(
                        filter_expression
                            .or(products::id.eq(search_number))
                            .or(products::min_stock.eq(search_number)),
                    );
                } else {
                    products_query = products_query.filter(filter_expression);
                }

                let products_query = products_query
                    .select((
                        ProductRow::as_select(),
                        stock_movements::table
                            .select(
                                sum(stock_movements::quantity)
                                    .aggregate_filter(stock_movements::product_id.eq(products::id)),
                            )
                            .single_value(),
                    ))
                    .order_by(products::id);

                products_query.load::<(ProductRow, Option<i64>)>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        let stock_products = products
            .into_iter()
            .map(|(product, stock)| {
                let min = product.min_stock.clone();
                (
                    product.into(),
                    Stock {
                        current: stock.unwrap_or(0),
                        min,
                    },
                )
            })
            .collect();

        Ok(stock_products)
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
