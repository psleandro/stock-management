use chrono::Utc;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::{CreateProductRow, ProductRow, UpdateProductRow};
use crate::infrastructure::db::schema::products;
use crate::models::ids::WorkspaceId;
use crate::models::product::UpdateProduct;
use crate::models::product::{CreateProduct, Product};

pub struct ProductsRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl ProductsRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_products(
        &self,
        workspace_id: WorkspaceId,
        search: &str,
    ) -> Result<Vec<Product>, InfrastructureError> {
        let search = search.to_string();

        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let product_list: Vec<ProductRow> = connection
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

                products_query.load::<ProductRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        let prods = product_list
            .into_iter()
            .map(|product| product.into())
            .collect::<Vec<Product>>();

        Ok(prods)
    }

    pub async fn get_product_by_id(
        &self,
        workspace_id: WorkspaceId,
        product_id: i32,
    ) -> Result<Option<Product>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let product = connection
            .interact(move |conn| {
                products::table
                    .filter(products::deleted_at.is_null())
                    .filter(products::workspace_id.eq(workspace_id.value()))
                    .find(product_id)
                    .first::<ProductRow>(conn)
                    .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(product.map(|p| p.into()))
    }

    pub async fn create_product(
        &self,
        new_product: CreateProduct,
    ) -> Result<Product, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let create_product_row = CreateProductRow {
            workspace_id: new_product.workspace_id.value(),
            name: new_product.name,
            unit: new_product.unit,
            brand: new_product.brand,
            min_stock: new_product.min_stock,
            observation: new_product.observation,
        };

        let created_product = connection
            .interact(move |conn| {
                diesel::insert_into(products::table)
                    .values(create_product_row)
                    .returning(ProductRow::as_returning())
                    .get_result::<ProductRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(created_product.into())
    }

    pub async fn update_product(
        &self,
        workspace_id: WorkspaceId,
        product_id: i32,
        product: UpdateProduct,
    ) -> Result<Option<Product>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let update_product_row = UpdateProductRow {
            name: product.name,
            unit: product.unit,
            brand: product.brand,
            min_stock: product.min_stock,
            observation: product.observation,
        };

        let now = Utc::now().naive_utc();

        let updated_product = connection
            .interact(move |conn| {
                diesel::update(
                    products::table
                        .filter(products::deleted_at.is_null())
                        .filter(products::workspace_id.eq(workspace_id.value()))
                        .find(product_id),
                )
                .set((&update_product_row, products::updated_at.eq(now)))
                .returning(ProductRow::as_returning())
                .get_result(conn)
                .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(updated_product.map(|p| p.into()))
    }

    pub async fn delete_product(
        &self,
        workspace_id: WorkspaceId,
        product_id: i32,
    ) -> Result<Option<Product>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let now = Utc::now().naive_utc();

        let deleted_product = connection
            .interact(move |conn| {
                diesel::update(
                    products::table
                        .filter(products::workspace_id.eq(workspace_id.value()))
                        .find(product_id),
                )
                .set(products::deleted_at.eq(Some(now)))
                .returning(ProductRow::as_returning())
                .get_result(conn)
                .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(deleted_product.map(|p| p.into()))
    }
}
