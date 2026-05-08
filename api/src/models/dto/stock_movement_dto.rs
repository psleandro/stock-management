use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct StockMovementEntryDto {
    pub movement_date: NaiveDateTime,

    pub product_id: Uuid,

    pub supplier_id: i32,

    #[validate(range(min = 1, message = "needs to be greater than zero"))]
    pub quantity: i32,

    #[validate(range(min = 0, message = "needs to be greater than or equal to zero"))]
    pub unit_cost_in_cents: i32,

    pub invoice_number: String,

    pub notes: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct StockMovementExitDto {
    pub movement_date: NaiveDateTime,

    pub product_id: Uuid,

    pub place_id: i32,

    #[validate(range(min = 1, message = "needs to be greater than zero"))]
    pub quantity: i32,

    pub notes: Option<String>,
}
