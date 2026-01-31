use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::errors::ApplicationError;
use crate::infrastructure::db::products_repository::ProductsRepository;
use crate::models::dto::product_dto::{CreateProductDto, ListProductsParams, UpdateProductDto};
use crate::models::product::{CreateProduct, Product, UpdateProduct};

pub struct ProductsService {
    pub products_repository: ProductsRepository,
}

impl ProductsService {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        let products_repository = ProductsRepository::new(pool.clone());
        Self {
            products_repository,
        }
    }

    pub async fn list_products(
        &self,
        workspace_id: i32,
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
        workspace_id: i32,
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
        workspace_id: i32,
        payload: CreateProductDto,
    ) -> Result<Product, ApplicationError> {
        let create_product_data = CreateProduct {
            workspace_id: workspace_id,
            name: payload.name,
            unit: payload.unit,
            brand: payload.brand,
            min_stock: payload.min_stock,
            observation: payload.observation,
        };

        let created_product = self
            .products_repository
            .create_product(create_product_data)
            .await?;

        Ok(created_product)
    }

    pub async fn update_product(
        &self,
        workspace_id: i32,
        product_id: i32,
        payload: UpdateProductDto,
    ) -> Result<Product, ApplicationError> {
        let update_product_data = UpdateProduct {
            name: payload.name,
            unit: payload.unit,
            brand: payload.brand,
            min_stock: payload.min_stock,
            observation: payload.observation,
        };

        let updated_product = self
            .products_repository
            .update_product(workspace_id, product_id, update_product_data)
            .await?;

        match updated_product {
            Some(p) => Ok(p),
            None => Err(ApplicationError::NotFound),
        }
    }

    pub async fn delete_product(
        &self,
        workspace_id: i32,
        product_id: i32,
    ) -> Result<(), ApplicationError> {
        let deleted_product = self
            .products_repository
            .delete_product(workspace_id, product_id)
            .await?;

        match deleted_product {
            Some(_) => Ok(()),
            None => Err(ApplicationError::NotFound),
        }
    }
}
