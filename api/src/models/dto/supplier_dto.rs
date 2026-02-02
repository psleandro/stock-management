use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct ListSuppliersParams {
    pub search: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreateSupplierDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateSupplierDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: Option<String>,
}
