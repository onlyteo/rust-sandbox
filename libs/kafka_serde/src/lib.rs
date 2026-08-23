pub mod error;

#[cfg(feature = "avro")]
pub mod avro;
#[cfg(feature = "json")]
pub mod json;

pub use error::KafkaSerdeError;

#[cfg(feature = "avro")]
pub use avro::AvroSerde;
#[cfg(feature = "json")]
pub use json::JsonSerde;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// A pluggable codec for turning Kafka message payloads into typed values and back.
///
/// Implement this trait to support a wire format other than the ones shipped here
/// (JSON behind the `json` feature, Avro behind the `avro` feature), then pass the
/// implementation to `kafka_client::KafkaProducer::new` / `KafkaConsumer::new`.
pub trait KafkaSerde: Send + Sync {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError>;

    fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError>;
}
