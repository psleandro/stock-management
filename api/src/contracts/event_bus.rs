use crate::models::product::Product;

#[derive(Debug)]
pub enum Event {
    ProductCreated(Product),
    ProductUpdated(Product),
    ProductDeleted(Product),
}

impl Event {
    pub fn to_message(&self) -> Result<(&'static str, String, Vec<u8>), serde_json::Error> {
        match self {
            Event::ProductCreated(product) => Ok((
                "product_created",
                format!("product#{}", product.id.to_string()),
                serde_json::to_vec(product)?,
            )),
            Event::ProductUpdated(product) => Ok((
                "product_updated",
                format!("product#{}", product.id.to_string()),
                serde_json::to_vec(product)?,
            )),
            Event::ProductDeleted(product) => Ok((
                "product_deleted",
                format!("product#{}", product.id.to_string()),
                serde_json::to_vec(product)?,
            )),
        }
    }
}

pub trait EventBus: Send + Sync {
    fn publish(&self, event: Event);
}
