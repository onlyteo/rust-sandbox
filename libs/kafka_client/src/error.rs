#[derive(Debug, thiserror::Error)]
pub enum KafkaClientError {
    #[error("kafka client error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),

    #[error("failed to serialize/deserialize message payload: {0}")]
    Payload(#[from] kafka_serde::KafkaSerdeError),
}
