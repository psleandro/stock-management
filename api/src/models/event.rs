use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::{product::Product, stock_movement::StockMovement};

#[derive(Serialize, Debug, Clone)]
pub struct StockMovementEvent {
    pub product_id: Uuid,
    pub movement_id: i32,
    pub quantity: i32,
}

impl From<StockMovement> for StockMovementEvent {
    fn from(stock_movement: StockMovement) -> Self {
        Self {
            movement_id: stock_movement.id,
            product_id: stock_movement.product_id,
            quantity: stock_movement.quantity.abs(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "event_type", content = "data")]
pub enum DomainEvent {
    ProductCreated(Product),
    ProductUpdated(Product),
    ProductDeleted(Product),
    StockIn(StockMovementEvent),
    StockOut(StockMovementEvent),
}

impl DomainEvent {
    pub fn into_envelope(self) -> EventEnvelope {
        EventEnvelope::new(self)
    }
}

#[derive(Serialize, Clone)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub created_at: DateTime<Utc>,

    #[serde(flatten)]
    pub event: DomainEvent,
}

impl EventEnvelope {
    pub fn new(event: DomainEvent) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            created_at: Utc::now(),
            event,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_domain_event_with_expected_structure() {
        let event = DomainEvent::StockIn(StockMovementEvent {
            movement_id: 1,
            product_id: Uuid::now_v7(),
            quantity: 10,
        });

        let value = serde_json::to_value(event).unwrap();

        assert_eq!(value["event_type"], json!("StockIn"));
        assert!(value["data"].is_object());
    }

    #[test]
    fn serializes_event_envelope_with_expected_structure() {
        let event = DomainEvent::StockOut(StockMovementEvent {
            movement_id: 2,
            product_id: Uuid::now_v7(),
            quantity: 5,
        })
        .into_envelope();

        let value = serde_json::to_value(event).unwrap();

        assert!(value["event_id"].is_string());
        assert!(value["created_at"].is_string());
        assert!(value["event"].is_null());
    }
}
