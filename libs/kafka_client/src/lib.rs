pub mod config;
pub mod consumer;
pub mod error;
pub mod producer;

pub use config::{
    Acks, AutoOffsetReset, CommonConfig, CompressionType, ConsumerConfig, ProducerConfig,
    SaslMechanism, SecurityProtocol, build_consumer_client_config, build_producer_client_config,
};
pub use consumer::KafkaConsumer;
pub use error::KafkaClientError;
pub use kafka_serde::{JsonSerde, KafkaSerde};
pub use producer::KafkaProducer;
