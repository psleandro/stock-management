use serde::Deserialize;
use validator::Validate;

use crate::models::product::BaseUnit;

#[derive(Deserialize)]
pub struct ListProductsParams {
    pub search: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: String,
    pub base_unit: BaseUnit,
    pub brand: Option<String>,
    pub min_stock: i64,
    pub observation: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: Option<String>,
    pub base_unit: Option<BaseUnit>,
    pub brand: Option<String>,
    pub min_stock: Option<i64>,
    pub observation: Option<String>,
}
