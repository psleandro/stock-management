use chrono::Utc;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::{CreateSupplierRow, SupplierRow, UpdateSupplierRow};
use crate::infrastructure::db::schema::suppliers;
use crate::models::ids::WorkspaceId;
use crate::models::supplier::{CreateSupplier, Supplier, UpdateSupplier};

pub struct SuppliersRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl SuppliersRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_suppliers(
        &self,
        workspace_id: WorkspaceId,
        search: &str,
    ) -> Result<Vec<Supplier>, InfrastructureError> {
        let search = search.to_string();

        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let supplier_list: Vec<SupplierRow> = connection
            .interact(move |conn| {
                let search_like = format!("%{}%", search);

                let mut suppliers_query = suppliers::table
                    .filter(suppliers::deleted_at.is_null())
                    .filter(suppliers::workspace_id.eq(workspace_id.value()))
                    .into_boxed();

                let filter_expression = suppliers::name.ilike(&search_like);

                if let Ok(search_number) = search.parse::<i32>() {
                    suppliers_query = suppliers_query
                        .filter(filter_expression.or(suppliers::id.eq(search_number)));
                } else {
                    suppliers_query = suppliers_query.filter(filter_expression);
                }

                suppliers_query.load::<SupplierRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        let prods = supplier_list
            .into_iter()
            .map(|supplier| supplier.into())
            .collect::<Vec<Supplier>>();

        Ok(prods)
    }

    pub async fn get_supplier_by_id(
        &self,
        workspace_id: WorkspaceId,
        supplier_id: i32,
    ) -> Result<Option<Supplier>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let supplier = connection
            .interact(move |conn| {
                suppliers::table
                    .filter(suppliers::deleted_at.is_null())
                    .filter(suppliers::workspace_id.eq(workspace_id.value()))
                    .find(supplier_id)
                    .first::<SupplierRow>(conn)
                    .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(supplier.map(|p| p.into()))
    }

    pub async fn create_supplier(
        &self,
        new_supplier: CreateSupplier,
    ) -> Result<Supplier, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let create_supplier_row = CreateSupplierRow {
            workspace_id: new_supplier.workspace_id.value(),
            name: new_supplier.name,
        };

        let created_supplier = connection
            .interact(move |conn| {
                diesel::insert_into(suppliers::table)
                    .values(create_supplier_row)
                    .returning(SupplierRow::as_returning())
                    .get_result::<SupplierRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(created_supplier.into())
    }

    pub async fn update_supplier(
        &self,
        workspace_id: WorkspaceId,
        supplier_id: i32,
        supplier: UpdateSupplier,
    ) -> Result<Option<Supplier>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let update_supplier_row = UpdateSupplierRow {
            name: supplier.name,
        };

        let now = Utc::now().naive_utc();

        let updated_supplier = connection
            .interact(move |conn| {
                diesel::update(
                    suppliers::table
                        .filter(suppliers::deleted_at.is_null())
                        .filter(suppliers::workspace_id.eq(workspace_id.value()))
                        .find(supplier_id),
                )
                .set((&update_supplier_row, suppliers::updated_at.eq(now)))
                .returning(SupplierRow::as_returning())
                .get_result(conn)
                .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(updated_supplier.map(|p| p.into()))
    }

    pub async fn delete_supplier(
        &self,
        workspace_id: WorkspaceId,
        supplier_id: i32,
    ) -> Result<Option<Supplier>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let now = Utc::now().naive_utc();

        let deleted_supplier = connection
            .interact(move |conn| {
                diesel::update(
                    suppliers::table
                        .filter(suppliers::workspace_id.eq(workspace_id.value()))
                        .find(supplier_id),
                )
                .set(suppliers::deleted_at.eq(Some(now)))
                .returning(SupplierRow::as_returning())
                .get_result(conn)
                .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(deleted_supplier.map(|p| p.into()))
    }
}
