use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::errors::ApplicationError;
use crate::infrastructure::db::suppliers_repository::SuppliersRepository;
use crate::models::dto::supplier_dto::{CreateSupplierDto, ListSuppliersParams, UpdateSupplierDto};
use crate::models::ids::WorkspaceId;
use crate::models::supplier::{CreateSupplier, Supplier, UpdateSupplier};

pub struct SuppliersService {
    pub suppliers_repository: SuppliersRepository,
}

impl SuppliersService {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        let suppliers_repository = SuppliersRepository::new(pool.clone());
        Self {
            suppliers_repository,
        }
    }

    pub async fn list_suppliers(
        &self,
        workspace_id: WorkspaceId,
        params: ListSuppliersParams,
    ) -> Result<Vec<Supplier>, ApplicationError> {
        let search = params.search.unwrap_or_default();

        let suppliers = self
            .suppliers_repository
            .list_suppliers(workspace_id, &search)
            .await?;

        Ok(suppliers)
    }

    pub async fn get_supplier(
        &self,
        workspace_id: WorkspaceId,
        supplier_id: i32,
    ) -> Result<Supplier, ApplicationError> {
        let supplier = self
            .suppliers_repository
            .get_supplier_by_id(workspace_id, supplier_id)
            .await?;

        match supplier {
            Some(p) => Ok(p),
            None => Err(ApplicationError::NotFound),
        }
    }

    pub async fn create_supplier(
        &self,
        workspace_id: WorkspaceId,
        payload: CreateSupplierDto,
    ) -> Result<Supplier, ApplicationError> {
        let create_supplier_data = CreateSupplier {
            workspace_id: workspace_id,
            name: payload.name,
        };

        let created_supplier = self
            .suppliers_repository
            .create_supplier(create_supplier_data)
            .await?;

        Ok(created_supplier)
    }

    pub async fn update_supplier(
        &self,
        workspace_id: WorkspaceId,
        supplier_id: i32,
        payload: UpdateSupplierDto,
    ) -> Result<Supplier, ApplicationError> {
        let update_supplier_data = UpdateSupplier { name: payload.name };

        let updated_supplier = self
            .suppliers_repository
            .update_supplier(workspace_id, supplier_id, update_supplier_data)
            .await?;

        match updated_supplier {
            Some(p) => Ok(p),
            None => Err(ApplicationError::NotFound),
        }
    }

    pub async fn delete_supplier(
        &self,
        workspace_id: WorkspaceId,
        supplier_id: i32,
    ) -> Result<(), ApplicationError> {
        let deleted_supplier = self
            .suppliers_repository
            .delete_supplier(workspace_id, supplier_id)
            .await?;

        match deleted_supplier {
            Some(_) => Ok(()),
            None => Err(ApplicationError::NotFound),
        }
    }
}
