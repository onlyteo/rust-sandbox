use apache_avro::Schema;
use apache_avro::reader::datum::GenericDatumReader;
use apache_avro::writer::datum::GenericDatumWriter;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{KafkaDeserializer, KafkaSerde, KafkaSerdeError, KafkaSerializer};

#[derive(Debug, Clone)]
pub struct AvroKafkaSerializer {
    schema: Schema,
}

impl AvroKafkaSerializer {
    pub fn new(schema: Schema) -> Self {
        Self { schema }
    }

    pub fn from_schema_str(raw: &str) -> Result<Self, KafkaSerdeError> {
        let schema = Schema::parse_str(raw)?;
        Ok(Self::new(schema))
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl KafkaSerializer for AvroKafkaSerializer {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        let writer = GenericDatumWriter::builder(&self.schema).build()?;
        let bytes = writer.write_ser_to_vec(value)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct AvroKafkaDeserializer {
    schema: Schema,
}

impl AvroKafkaDeserializer {
    pub fn new(schema: Schema) -> Self {
        Self { schema }
    }

    pub fn from_schema_str(raw: &str) -> Result<Self, KafkaSerdeError> {
        let schema = Schema::parse_str(raw)?;
        Ok(Self::new(schema))
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl KafkaDeserializer for AvroKafkaDeserializer {
    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        let reader = GenericDatumReader::builder(&self.schema).build()?;
        let mut cursor = bytes;
        let value = reader.read_deser(&mut cursor)?;
        Ok(value)
    }
}

#[derive(Clone)]
pub struct AvroKafkaSerde {
    schema: Schema,
    serializer: AvroKafkaSerializer,
    deserializer: AvroKafkaDeserializer,
}

impl AvroKafkaSerde {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema: schema.clone(),
            serializer: AvroKafkaSerializer::new(schema.clone()),
            deserializer: AvroKafkaDeserializer::new(schema.clone()),
        }
    }

    pub fn from_schema_str(raw: &str) -> Result<Self, KafkaSerdeError> {
        let schema = Schema::parse_str(raw)?;
        Ok(Self::new(schema))
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl KafkaSerde for AvroKafkaSerde {
    async fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        self.serializer.serialize(value).await
    }

    async fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        self.deserializer.deserialize::<T>(bytes).await
    }
}
