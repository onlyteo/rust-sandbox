pub mod error;

#[cfg(feature = "avro")]
pub mod avro;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "schema_registry")]
pub mod schema_registry;

pub use error::KafkaSerdeError;

#[cfg(feature = "avro")]
pub use avro::AvroKafkaSerde;
#[cfg(feature = "json")]
pub use json::JsonKafkaSerde;
#[cfg(feature = "schema_registry")]
pub use schema_registry::SchemaRegistryKafkaSerde;

use serde::Serialize;
use serde::de::DeserializeOwned;

#[allow(async_fn_in_trait)]
pub trait KafkaSerializer {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError>;
}

#[allow(async_fn_in_trait)]
pub trait KafkaDeserializer {
    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError>;
}

#[allow(async_fn_in_trait)]
pub trait KafkaSerde: Send + Sync {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError>;
    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError>;
}
