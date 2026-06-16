use serde::Serialize;
use uuid::Uuid;

use crate::models::product::Product;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "event_type", content = "data")]
pub enum Event {
    ProductCreated(Product),
    ProductUpdated(Product),
    ProductDeleted(Product),
    StockIn(StockMovementEvent),
    StockOut(StockMovementEvent),
}

#[derive(Serialize, Debug, Clone)]
pub struct StockMovementEvent {
    pub product_id: Uuid,
    pub movement_id: i32,
    pub quantity: i32,
}

impl Event {
    pub fn to_message(&self) -> Result<(&'static str, String, Vec<u8>), serde_json::Error> {
        let (topic, key) = match self {
            Event::ProductCreated(product) => ("products.events", product.id.to_string()),
            Event::ProductUpdated(product) => ("products.events", product.id.to_string()),
            Event::ProductDeleted(product) => ("products.events", product.id.to_string()),
            Event::StockIn(event) => ("stock_movements.events", event.product_id.to_string()),
            Event::StockOut(event) => ("stock_movements.events", event.product_id.to_string()),
        };

        let payload = serde_json::to_vec(self)?;

        Ok((topic, key, payload))
    }
}

pub trait EventBus: Send + Sync {
    fn publish(&self, event: Event);
}
