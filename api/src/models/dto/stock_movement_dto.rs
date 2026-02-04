use chrono::NaiveDateTime;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StockMovementEntryDto {
    pub movement_date: NaiveDateTime,

    pub product_id: i32,

    pub supplier_id: i32,

    #[validate(range(min = 1, message = "needs to be greater than zero"))]
    pub quantity: i32,

    #[validate(range(min = 0, message = "needs to be greater than or equal to zero"))]
    pub unit_cost_in_cents: i32,

    pub invoice_number: String,

    pub notes: Option<String>,
}
