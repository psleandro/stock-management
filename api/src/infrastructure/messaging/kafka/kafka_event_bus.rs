use std::sync::Arc;

use rdkafka::producer::{BaseProducer, BaseRecord};

use crate::contracts::event_bus::EventBus;
use crate::infrastructure::messaging::kafka::kafka_context::LoggingContext;
use crate::models::event::{DomainEvent, EventEnvelope};

pub struct KafkaEventBus {
    producer: Arc<BaseProducer<LoggingContext>>,
}

impl KafkaEventBus {
    pub fn new(producer: Arc<BaseProducer<LoggingContext>>) -> Self {
        Self { producer }
    }
}

impl EventEnvelope {
    pub fn get_topic_data(&self) -> (&'static str, String) {
        let (topic, topic_key) = match &self.event {
            DomainEvent::ProductCreated(product) => ("products.events", product.id.into()),
            DomainEvent::ProductUpdated(product) => ("products.events", product.id.into()),
            DomainEvent::ProductDeleted(product) => ("products.events", product.id.into()),
            DomainEvent::StockIn(event) => ("stock_movements.events", event.product_id.into()),
            DomainEvent::StockOut(event) => ("stock_movements.events", event.product_id.into()),
        };

        (topic, topic_key)
    }
}

impl EventBus for KafkaEventBus {
    fn publish(&self, event_envelope: EventEnvelope) {
        let Ok(payload) = serde_json::to_vec(&event_envelope) else {
            println!("[KAFKA] Invalid event message serialization");
            return;
        };
        let (topic, key) = event_envelope.get_topic_data();

        match self
            .producer
            .send(BaseRecord::to(topic).key(&key).payload(&payload))
        {
            Ok(_) => {}
            Err((err, msg)) => {
                println!("[KAFKA][SEND ERROR] topic={} error={:?}", msg.topic, err);
            }
        }
    }
}
