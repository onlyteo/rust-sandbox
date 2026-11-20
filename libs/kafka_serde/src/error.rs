#[cfg(feature = "schema_registry")]
use schema_registry_converter::error::SRCError;

#[derive(Debug, thiserror::Error)]
pub enum KafkaSerdeError {
    #[cfg(feature = "json")]
    #[error("json serde error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(any(feature = "avro", feature = "schema_registry"))]
    #[error("avro serde error: {0}")]
    Avro(#[from] apache_avro::Error),

    #[cfg(feature = "schema_registry")]
    #[error("schema registry serde error: {0}")]
    SchemaRegistry(#[from] SRCError),
}
