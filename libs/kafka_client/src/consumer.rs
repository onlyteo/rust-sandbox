use kafka_serde::{JsonSerde, KafkaSerde};
use rdkafka::consumer::{Consumer, DefaultConsumerContext, MessageStream, StreamConsumer};
use rdkafka::message::{BorrowedMessage, Message};
use serde::de::DeserializeOwned;

use crate::config::{CommonConfig, ConsumerConfig, build_consumer_client_config};
use crate::error::KafkaClientError;

pub struct KafkaConsumer<S: KafkaSerde = JsonSerde> {
    inner: StreamConsumer,
    serde: S,
}

impl<S: KafkaSerde> KafkaConsumer<S> {
    pub fn new(
        common: &CommonConfig,
        consumer: &ConsumerConfig,
        topics: &[&str],
        serde: S,
    ) -> Result<Self, KafkaClientError> {
        let inner: StreamConsumer = build_consumer_client_config(common, consumer).create()?;
        inner.subscribe(topics)?;
        Ok(Self { inner, serde })
    }

    pub fn stream(&self) -> MessageStream<'_, DefaultConsumerContext> {
        self.inner.stream()
    }

    pub fn deserialize_payload<T: DeserializeOwned>(
        &self,
        message: &BorrowedMessage<'_>,
    ) -> Result<T, KafkaClientError> {
        let payload = message.payload().unwrap_or_default();
        let value = self.serde.deserialize(payload)?;
        Ok(value)
    }
}

impl KafkaConsumer<JsonSerde> {
    pub fn new_json(
        common: &CommonConfig,
        consumer: &ConsumerConfig,
        topics: &[&str],
    ) -> Result<Self, KafkaClientError> {
        Self::new(common, consumer, topics, JsonSerde)
    }
}
