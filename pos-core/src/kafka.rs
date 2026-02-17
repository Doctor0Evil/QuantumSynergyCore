use anyhow::Result;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::time::Duration;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn send<T: Serialize>(&self, key: &str, payload: &T) -> Result<()> {
        let json = serde_json::to_vec(payload)?;
        let record = FutureRecord::to(&self.topic)
            .key(key)
            .payload(&json);
        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .map_err(|(e, _)| e)?;
        Ok(())
    }
}
