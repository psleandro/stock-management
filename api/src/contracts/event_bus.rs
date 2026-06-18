use crate::models::event::EventEnvelope;

pub trait EventBus: Send + Sync {
    fn publish(&self, event: EventEnvelope);
}
