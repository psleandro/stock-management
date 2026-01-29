use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct ListProductsParams {
    pub search: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: String,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: i32,
    pub observation: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: Option<String>,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: Option<i32>,
    pub observation: Option<String>,
}
