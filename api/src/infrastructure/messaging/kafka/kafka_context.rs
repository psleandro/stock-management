use rdkafka::Message;
use rdkafka::client::ClientContext;
use rdkafka::producer::{DeliveryResult, ProducerContext};

pub struct LoggingContext;

impl ClientContext for LoggingContext {}

impl ProducerContext for LoggingContext {
    type DeliveryOpaque = ();

    fn delivery(&self, result: &DeliveryResult, _opaque: Self::DeliveryOpaque) {
        match result {
            Ok(meta) => {
                let message = meta.detach();

                println!(
                    "[KAFKA][DELIVERED] topic={} partition={} offset={}",
                    message.topic(),
                    message.partition(),
                    message.offset()
                );
            }
            Err((err, msg)) => {
                let message = msg.detach();
                println!(
                    "[KAFKA][FAILED DELIVERY] topic={} error={:?}",
                    message.topic(),
                    err
                );
            }
        }
    }
}
