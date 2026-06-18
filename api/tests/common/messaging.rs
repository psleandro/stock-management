use std::sync::{Arc, Mutex};

use stock_management_api::{contracts::event_bus::EventBus, models::event::EventEnvelope};

pub struct MockEventBus {
    pub published_events: Arc<Mutex<Vec<EventEnvelope>>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn published_events(&self) -> Vec<EventEnvelope> {
        self.published_events.lock().unwrap().clone()
    }
}

impl EventBus for MockEventBus {
    fn publish(&self, event_message: EventEnvelope) {
        let (topic, topic_key) = event_message.get_topic_data();

        self.published_events.lock().unwrap().push(event_message);

        println!("[EVENTBUS] Topic: {:?} | Key: {:?}", topic, topic_key)
    }
}
