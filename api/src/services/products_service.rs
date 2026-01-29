use std::error::Error;

use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

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
    ) -> Result<Vec<Product>, Box<dyn Error>> {
        let search = params.search.unwrap_or_default();

        let products = self
            .products_repository
            .list_products(workspace_id, &search)
            .await?;

        Ok(products)
    }

    pub async fn get_product(
        &self,
        _user_id: i32,
        product_id: i32,
    ) -> Result<Product, Box<dyn Error>> {
        let product = self
            .products_repository
            .get_product_by_id(product_id)
            .await?;

        Ok(product)
    }

    pub async fn create_product(
        &self,
        workspace_id: i32,
        payload: CreateProductDto,
    ) -> Result<Product, Box<dyn Error>> {
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
            .await;

        created_product
    }

    pub async fn update_product(
        &self,
        _user_id: i32,
        product_id: i32,
        payload: UpdateProductDto,
    ) -> Result<Product, Box<dyn Error>> {
        let update_product_data = UpdateProduct {
            name: payload.name,
            unit: payload.unit,
            brand: payload.brand,
            min_stock: payload.min_stock,
            observation: payload.observation,
        };

        let updated_product = self
            .products_repository
            .update_product(product_id, update_product_data)
            .await?;

        Ok(updated_product)
    }

    pub async fn delete_product(
        &self,
        _user_id: i32,
        product_id: i32,
    ) -> Result<(), Box<dyn Error>> {
        self.products_repository.delete_product(product_id).await?;

        Ok(())
    }
}
