use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{KafkaSerde, KafkaSerdeError};

/// Serializes/deserializes payloads as JSON.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonSerde;

impl KafkaSerde for JsonSerde {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        Ok(serde_json::to_vec(value)?)
    }

    fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
