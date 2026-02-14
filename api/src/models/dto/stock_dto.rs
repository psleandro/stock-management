use serde::Serialize;

use crate::models::product::{Product, Stock};

#[derive(Serialize)]
pub struct ProductWithStockDto {
    pub product: Product,
    pub stock: Stock,
}
