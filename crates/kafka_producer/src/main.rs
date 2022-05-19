use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let delivery_status = producer
        .send(
            FutureRecord::to("my_topic")
                .payload("Hello, World".to_string().as_str())
                .key("KEY0".to_string().as_str())
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            Duration::from_secs(2),
        )
        .await;

    // This will be executed when the result is received.
    println!("Delivery status for message received");
    println!("{:?}", delivery_status);
}
