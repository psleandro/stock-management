use std::env;

use rdkafka::ClientConfig;
use rdkafka::producer::BaseProducer;

use dotenvy::dotenv;

use crate::infrastructure::messaging::kafka::kafka_context::LoggingContext;

pub fn create_producer() -> BaseProducer<LoggingContext> {
    dotenv().ok();

    let brokers = env::var("KAFKA_BROKERS").expect("KAFKA_BROKERS must be set");

    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "60000")
        .set("enable.idempotence", "true")
        .set("retries", "2147483647")
        .create_with_context(LoggingContext)
        .expect("Kafka producer creation failed")
}
