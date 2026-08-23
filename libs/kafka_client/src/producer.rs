use std::time::Duration;

use kafka_serde::{JsonSerde, KafkaSerde};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::Serialize;

use crate::config::{CommonConfig, ProducerConfig, build_producer_client_config};
use crate::error::KafkaClientError;

const DEFAULT_SEND_TIMEOUT: Duration = Duration::from_secs(5);

pub struct KafkaProducer<S: KafkaSerde = JsonSerde> {
    inner: FutureProducer,
    serde: S,
}

impl<S: KafkaSerde> KafkaProducer<S> {
    pub fn new(
        common: &CommonConfig,
        producer: &ProducerConfig,
        serde: S,
    ) -> Result<Self, KafkaClientError> {
        let inner = build_producer_client_config(common, producer).create()?;
        Ok(Self { inner, serde })
    }

    pub async fn send(
        &self,
        topic: &str,
        key: Option<&[u8]>,
        payload: &[u8],
    ) -> Result<(), KafkaClientError> {
        let record = FutureRecord::to(topic).payload(payload);

        let result = match key {
            Some(key) => {
                self.inner
                    .send(record.key(key), Timeout::After(DEFAULT_SEND_TIMEOUT))
                    .await
            }
            None => {
                self.inner
                    .send(record, Timeout::After(DEFAULT_SEND_TIMEOUT))
                    .await
            }
        };

        result.map_err(|(error, _message)| error)?;
        Ok(())
    }

    pub async fn send_payload<T: Serialize>(
        &self,
        topic: &str,
        key: Option<&str>,
        payload: &T,
    ) -> Result<(), KafkaClientError> {
        let bytes = self.serde.serialize(payload)?;
        self.send(topic, key.map(str::as_bytes), &bytes).await
    }
}

impl KafkaProducer<JsonSerde> {
    pub fn new_json(
        common: &CommonConfig,
        producer: &ProducerConfig,
    ) -> Result<Self, KafkaClientError> {
        Self::new(common, producer, JsonSerde)
    }
}
