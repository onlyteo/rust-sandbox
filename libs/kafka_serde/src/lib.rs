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

pub trait KafkaSerde: Send + Sync {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError>;

    fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError>;
}
