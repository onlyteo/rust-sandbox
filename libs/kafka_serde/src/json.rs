use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{KafkaDeserializer, KafkaSerde, KafkaSerdeError, KafkaSerializer};

#[derive(Debug, Clone)]
pub struct JsonKafkaSerializer;

impl JsonKafkaSerializer {
    pub fn new() -> Self {
        Self {}
    }
}

impl KafkaSerializer for JsonKafkaSerializer {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        Ok(serde_json::to_vec(value)?)
    }
}

#[derive(Debug, Clone)]
pub struct JsonKafkaDeserializer;

impl JsonKafkaDeserializer {
    pub fn new() -> Self {
        Self {}
    }
}

impl KafkaDeserializer for JsonKafkaDeserializer {
    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[derive(Debug, Clone)]
pub struct JsonKafkaSerde;

impl JsonKafkaSerde {
    pub fn new() -> Self {
        Self {}
    }
}

impl KafkaSerde for JsonKafkaSerde {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        Ok(serde_json::to_vec(value)?)
    }

    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
