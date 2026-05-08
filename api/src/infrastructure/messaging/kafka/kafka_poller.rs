use rdkafka::producer::BaseProducer;
use std::{sync::Arc, time::Duration};

use crate::infrastructure::messaging::kafka::kafka_context::LoggingContext;

pub fn start_kafka_poller(producer: Arc<BaseProducer<LoggingContext>>) {
    tokio::spawn(async move {
        loop {
            producer.poll(Duration::from_millis(0));
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}
