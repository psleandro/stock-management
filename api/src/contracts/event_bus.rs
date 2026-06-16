use serde::Serialize;
use uuid::Uuid;

use crate::models::product::Product;

#[derive(Debug, Clone)]
pub enum Event {
    ProductCreated(Product),
    ProductUpdated(Product),
    ProductDeleted(Product),
    StockChanged(StockEventType),
}

#[derive(Serialize, Debug, Clone)]
pub struct StockMovementEvent {
    pub product_id: Uuid,
    pub movement_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "event_type", content = "data")]
pub enum StockEventType {
    StockIn(StockMovementEvent),
    StockOut(StockMovementEvent),
}

impl StockEventType {
    pub fn product_id(&self) -> Uuid {
        match self {
            StockEventType::StockIn(e) | StockEventType::StockOut(e) => e.product_id,
        }
    }
}

#[derive(Serialize)]
pub enum ProductEventType {
    ProductCreated,
    ProductUpdated,
    ProductDeleted,
}
#[derive(Serialize)]
pub struct ProductEvent<'a> {
    pub event_type: ProductEventType,
    pub product_id: Uuid,
    pub data: &'a Product,
}

impl Event {
    pub fn to_message(&self) -> Result<(&'static str, String, Vec<u8>), serde_json::Error> {
        match self {
            Event::ProductCreated(product) => Ok((
                "products.events",
                product.id.to_string(),
                serde_json::to_vec(&ProductEvent {
                    event_type: ProductEventType::ProductCreated,
                    product_id: product.id,
                    data: product,
                })?,
            )),
            Event::ProductUpdated(product) => Ok((
                "products.events",
                product.id.to_string(),
                serde_json::to_vec(&ProductEvent {
                    event_type: ProductEventType::ProductUpdated,
                    product_id: product.id,
                    data: product,
                })?,
            )),
            Event::ProductDeleted(product) => Ok((
                "products.events",
                product.id.to_string(),
                serde_json::to_vec(&ProductEvent {
                    event_type: ProductEventType::ProductDeleted,
                    product_id: product.id,
                    data: product,
                })?,
            )),
            Event::StockChanged(event) => Ok((
                "stock_movements.events",
                event.product_id().to_string(),
                serde_json::to_vec(event)?,
            )),
        }
    }
}

pub trait EventBus: Send + Sync {
    fn publish(&self, event: Event);
}
