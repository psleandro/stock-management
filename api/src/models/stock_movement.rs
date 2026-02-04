use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct StockMovement {
    pub id: i32,
    pub movement_date: NaiveDateTime,
    pub product_id: i32,
    pub supplier_id: Option<i32>,
    pub place_id: Option<i32>,
    pub quantity: i32,
    pub unit_cost_in_cents: Option<i32>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<NaiveDateTime>,
}

pub struct StockMovementEntry {
    pub movement_date: NaiveDateTime,
    pub product_id: i32,
    pub supplier_id: i32,
    pub quantity: i32,
    pub unit_cost_in_cents: i32,
    pub invoice_number: String,
    pub notes: Option<String>,
}
