use serde::Serialize;
use uuid::Uuid;

use crate::models::product::Product;

#[derive(Debug)]
pub enum Event {
    ProductCreated(Product),
    ProductUpdated(Product),
    ProductDeleted(Product),
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
        }
    }
}

pub trait EventBus: Send + Sync {
    fn publish(&self, event: Event);
}
