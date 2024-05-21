use std::time::Duration;

use chrono::Utc;
use rdkafka::producer::FutureRecord;
use serde_json::json;

mod kafka;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let js = json!({
        "hello" : "world"
    });
    let payload = serde_json::to_string(&js).unwrap();
    let timestamp = format!("{}", Utc::now().timestamp());
    let topic = std::env::var("KAFKA_FILLS_TOPIC").expect("KAFKA_FILLS_TOPIC must be set");
    let record = FutureRecord::to(&topic).key(&timestamp).payload(&payload);
    let producer = kafka::create_producer_from_env();
    producer.send(record, Duration::from_secs(2)).await.unwrap();
}
