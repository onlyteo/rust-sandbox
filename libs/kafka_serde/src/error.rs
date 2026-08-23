#[derive(Debug, thiserror::Error)]
pub enum KafkaSerdeError {
    #[cfg(feature = "json")]
    #[error("json serde error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "avro")]
    #[error("avro serde error: {0}")]
    Avro(#[from] apache_avro::Error),
}
