use std::sync::Arc;

use schema_registry_converter::{
    async_impl::{
        avro::{AvroDecoder, AvroEncoder},
        schema_registry::SrSettings,
    },
    schema_registry_common::SubjectNameStrategy,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{KafkaDeserializer, KafkaSerde, KafkaSerdeError, KafkaSerializer};

#[derive(Clone, Debug)]
pub struct SchemaRegistryKafkaSerializer {
    encoder: Arc<AvroEncoder>,
    strategy: SubjectNameStrategy,
}

impl SchemaRegistryKafkaSerializer {
    pub fn new(settings: SrSettings, strategy: SubjectNameStrategy) -> Self {
        let encoder = AvroEncoder::new(settings);
        Self {
            encoder: Arc::new(encoder),
            strategy,
        }
    }
}

impl KafkaSerializer for SchemaRegistryKafkaSerializer {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        let payload = self.encoder.encode_struct(value, &self.strategy).await?;
        Ok(payload)
    }
}

#[derive(Clone, Debug)]
pub struct SchemaRegistryKafkaDeserializer {
    decoder: Arc<AvroDecoder>,
}

impl SchemaRegistryKafkaDeserializer {
    pub fn new(settings: SrSettings) -> Self {
        let decoder = AvroDecoder::new(settings);
        Self {
            decoder: Arc::new(decoder),
        }
    }
}

impl KafkaDeserializer for SchemaRegistryKafkaDeserializer {
    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        let decoded = self.decoder.decode(Some(bytes)).await?;
        let value = apache_avro::from_value::<T>(&decoded.value)?;
        Ok(value)
    }
}

#[derive(Clone, Debug)]
pub struct SchemaRegistryKafkaSerde {
    serializer: SchemaRegistryKafkaSerializer,
    deserializer: SchemaRegistryKafkaDeserializer,
}

impl SchemaRegistryKafkaSerde {
    pub fn new(settings: SrSettings, strategy: SubjectNameStrategy) -> Self {
        Self {
            serializer: SchemaRegistryKafkaSerializer::new(settings.clone(), strategy),
            deserializer: SchemaRegistryKafkaDeserializer::new(settings.clone()),
        }
    }
}

impl KafkaSerde for SchemaRegistryKafkaSerde {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        self.serializer.serialize(value).await
    }

    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        self.deserializer.deserialize(bytes).await
    }
}
