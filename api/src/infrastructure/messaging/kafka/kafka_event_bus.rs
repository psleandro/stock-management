use std::sync::Arc;

use rdkafka::producer::{BaseProducer, BaseRecord};

use crate::contracts::event_bus::{Event, EventBus};
use crate::infrastructure::messaging::kafka::kafka_context::LoggingContext;

pub struct KafkaEventBus {
    producer: Arc<BaseProducer<LoggingContext>>,
}

impl KafkaEventBus {
    pub fn new(producer: Arc<BaseProducer<LoggingContext>>) -> Self {
        Self { producer }
    }
}

impl EventBus for KafkaEventBus {
    fn publish(&self, event: Event) {
        let Ok((topic, key, payload)) = event.to_message() else {
            println!("[EVENTBUS] invalid event serialization");
            return;
        };

        let key_str = key.to_string();

        match self
            .producer
            .send(BaseRecord::to(topic).key(&key_str).payload(&payload))
        {
            Ok(_) => {}
            Err((err, msg)) => {
                println!("[KAFKA][SEND ERROR] topic={} error={:?}", msg.topic, err);
            }
        }
    }
}
