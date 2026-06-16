use std::sync::{Arc, Mutex};

use stock_management_api::contracts::event_bus::{Event, EventBus};

pub struct MockEventBus {
    pub published_events: Arc<Mutex<Vec<Event>>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn published_events(&self) -> Vec<Event> {
        self.published_events.lock().unwrap().clone()
    }
}

impl EventBus for MockEventBus {
    fn publish(&self, event: Event) {
        let event_message = event.to_message();
        self.published_events.lock().unwrap().push(event);

        match event_message {
            Ok(message) => {
                println!("[EVENTBUS] Topic: {:?} | Key: {:?}", message.0, message.1)
            }
            Err(err) => {
                println!("[EVENTBUS] Error: {}", err.to_string())
            }
        }
    }
}
