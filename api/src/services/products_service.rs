use std::sync::Arc;

use crate::contracts::event_bus::{Event, EventBus};
use crate::errors::ApplicationError;
use crate::infrastructure::db::products_repository::ProductsRepository;
use crate::models::dto::product_dto::{CreateProductDto, ListProductsParams, UpdateProductDto};
use crate::models::ids::WorkspaceId;
use crate::models::product::{CreateProduct, Product, UpdateProduct};

#[derive(Clone)]
pub struct ProductsService {
    pub products_repository: ProductsRepository,
    pub event_bus: Arc<dyn EventBus>,
}

impl ProductsService {
    pub fn new(products_repository: ProductsRepository, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            products_repository,
            event_bus,
        }
    }

    pub async fn list_products(
        &self,
        workspace_id: WorkspaceId,
        params: ListProductsParams,
    ) -> Result<Vec<Product>, ApplicationError> {
        let search = params.search.unwrap_or_default();

        let products = self
            .products_repository
            .list_products(workspace_id, &search)
            .await?;

        Ok(products)
    }

    pub async fn get_product(
        &self,
        workspace_id: WorkspaceId,
        product_id: i32,
    ) -> Result<Product, ApplicationError> {
        let product = self
            .products_repository
            .get_product_by_id(workspace_id, product_id)
            .await?;

        match product {
            Some(p) => Ok(p),
            None => Err(ApplicationError::NotFound),
        }
    }

    pub async fn create_product(
        &self,
        workspace_id: WorkspaceId,
        payload: CreateProductDto,
    ) -> Result<Product, ApplicationError> {
        let create_product_data = CreateProduct {
            workspace_id: workspace_id,
            name: payload.name,
            base_unit: payload.base_unit,
            brand: payload.brand,
            min_stock: payload.min_stock,
            observation: payload.observation,
        };

        let created_product = self
            .products_repository
            .create_product(create_product_data)
            .await?;

        self.event_bus
            .publish(Event::ProductCreated(created_product.clone()));

        Ok(created_product)
    }

    pub async fn update_product(
        &self,
        workspace_id: WorkspaceId,
        product_id: i32,
        payload: UpdateProductDto,
    ) -> Result<Product, ApplicationError> {
        let update_product_data = UpdateProduct {
            name: payload.name,
            base_unit: payload.base_unit,
            brand: payload.brand,
            min_stock: payload.min_stock,
            observation: payload.observation,
        };

        let updated_product = self
            .products_repository
            .update_product(workspace_id, product_id, update_product_data)
            .await?;

        let updated_product = updated_product.ok_or(ApplicationError::NotFound)?;

        self.event_bus
            .publish(Event::ProductUpdated(updated_product.clone()));

        Ok(updated_product)
    }

    pub async fn delete_product(
        &self,
        workspace_id: WorkspaceId,
        product_id: i32,
    ) -> Result<(), ApplicationError> {
        let deleted_product = self
            .products_repository
            .delete_product(workspace_id, product_id)
            .await?;

        let deleted_product = deleted_product.ok_or(ApplicationError::NotFound)?;

        self.event_bus
            .publish(Event::ProductDeleted(deleted_product.clone()));

        Ok(())
    }
}
