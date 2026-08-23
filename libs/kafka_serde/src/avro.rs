use apache_avro::Schema;
use apache_avro::reader::datum::GenericDatumReader;
use apache_avro::writer::datum::GenericDatumWriter;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::{KafkaSerde, KafkaSerdeError};

/// Serializes/deserializes payloads as raw Avro, encoded against a fixed schema.
#[derive(Clone)]
pub struct AvroSerde {
    schema: Schema,
}

impl AvroSerde {
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

impl KafkaSerde for AvroSerde {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        let writer = GenericDatumWriter::builder(&self.schema).build()?;
        let bytes = writer.write_ser_to_vec(value)?;
        Ok(bytes)
    }

    fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        let reader = GenericDatumReader::builder(&self.schema).build()?;
        let mut cursor = bytes;
        let value = reader.read_deser(&mut cursor)?;
        Ok(value)
    }
}
