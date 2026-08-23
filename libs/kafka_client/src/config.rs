use rdkafka::ClientConfig;
use serde::Deserialize;

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityProtocol {
    #[default]
    Plaintext,
    Ssl,
    SaslPlaintext,
    SaslSsl,
}

impl SecurityProtocol {
    pub fn as_kafka_value(&self) -> &'static str {
        match self {
            Self::Plaintext => "plaintext",
            Self::Ssl => "ssl",
            Self::SaslPlaintext => "sasl_plaintext",
            Self::SaslSsl => "sasl_ssl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SaslMechanism {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "scram-sha-256")]
    ScramSha256,
    #[serde(rename = "scram-sha-512")]
    ScramSha512,
}

impl SaslMechanism {
    pub fn as_kafka_value(&self) -> &'static str {
        match self {
            Self::Plain => "PLAIN",
            Self::ScramSha256 => "SCRAM-SHA-256",
            Self::ScramSha512 => "SCRAM-SHA-512",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Acks {
    #[default]
    All,
    One,
    None,
}

impl Acks {
    pub fn as_kafka_value(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::One => "1",
            Self::None => "0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompressionType {
    #[default]
    None,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

impl CompressionType {
    pub fn as_kafka_value(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Gzip => "gzip",
            Self::Snappy => "snappy",
            Self::Lz4 => "lz4",
            Self::Zstd => "zstd",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoOffsetReset {
    #[default]
    Earliest,
    Latest,
    None,
}

impl AutoOffsetReset {
    pub fn as_kafka_value(&self) -> &'static str {
        match self {
            Self::Earliest => "earliest",
            Self::Latest => "latest",
            Self::None => "none",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommonConfig {
    #[serde(rename = "bootstrap.servers")]
    pub bootstrap_servers: String,
    #[serde(rename = "client.id")]
    pub client_id: Option<String>,
    #[serde(rename = "security.protocol", default)]
    pub security_protocol: SecurityProtocol,
    #[serde(rename = "sasl.mechanism")]
    pub sasl_mechanism: Option<SaslMechanism>,
    #[serde(rename = "sasl.username")]
    pub sasl_username: Option<String>,
    #[serde(rename = "sasl.password")]
    pub sasl_password: Option<String>,
    #[serde(rename = "ssl.ca.location")]
    pub ssl_ca_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProducerConfig {
    #[serde(default)]
    pub acks: Acks,
    #[serde(rename = "compression.type", default)]
    pub compression_type: CompressionType,
    #[serde(rename = "message.timeout.ms")]
    pub message_timeout_ms: Option<u32>,
    pub retries: Option<u32>,
    #[serde(rename = "linger.ms")]
    pub linger_ms: Option<u32>,
    #[serde(rename = "batch.size")]
    pub batch_size: Option<u32>,
    #[serde(rename = "enable.idempotence")]
    pub enable_idempotence: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConsumerConfig {
    #[serde(rename = "group.id")]
    pub group_id: String,
    #[serde(rename = "auto.offset.reset", default)]
    pub auto_offset_reset: AutoOffsetReset,
    #[serde(rename = "enable.auto.commit", default = "default_true")]
    pub enable_auto_commit: bool,
    #[serde(rename = "session.timeout.ms")]
    pub session_timeout_ms: Option<u32>,
    #[serde(rename = "max.poll.interval.ms")]
    pub max_poll_interval_ms: Option<u32>,
    #[serde(rename = "fetch.min.bytes")]
    pub fetch_min_bytes: Option<u32>,
}

fn apply_common_config(client_config: &mut ClientConfig, common: &CommonConfig) {
    client_config.set("bootstrap.servers", &common.bootstrap_servers);
    client_config.set(
        "security.protocol",
        common.security_protocol.as_kafka_value(),
    );

    if let Some(client_id) = &common.client_id {
        client_config.set("client.id", client_id);
    }
    if let Some(sasl_mechanism) = &common.sasl_mechanism {
        client_config.set("sasl.mechanism", sasl_mechanism.as_kafka_value());
    }
    if let Some(sasl_username) = &common.sasl_username {
        client_config.set("sasl.username", sasl_username);
    }
    if let Some(sasl_password) = &common.sasl_password {
        client_config.set("sasl.password", sasl_password);
    }
    if let Some(ssl_ca_location) = &common.ssl_ca_location {
        client_config.set("ssl.ca.location", ssl_ca_location);
    }
}

pub fn build_producer_client_config(common: &CommonConfig, producer: &ProducerConfig) -> ClientConfig {
    let mut client_config = ClientConfig::new();
    apply_common_config(&mut client_config, common);

    client_config.set("acks", producer.acks.as_kafka_value());
    client_config.set(
        "compression.type",
        producer.compression_type.as_kafka_value(),
    );

    if let Some(message_timeout_ms) = producer.message_timeout_ms {
        client_config.set("message.timeout.ms", message_timeout_ms.to_string());
    }
    if let Some(retries) = producer.retries {
        client_config.set("retries", retries.to_string());
    }
    if let Some(linger_ms) = producer.linger_ms {
        client_config.set("linger.ms", linger_ms.to_string());
    }
    if let Some(batch_size) = producer.batch_size {
        client_config.set("batch.size", batch_size.to_string());
    }
    if let Some(enable_idempotence) = producer.enable_idempotence {
        client_config.set("enable.idempotence", enable_idempotence.to_string());
    }

    client_config
}

pub fn build_consumer_client_config(common: &CommonConfig, consumer: &ConsumerConfig) -> ClientConfig {
    let mut client_config = ClientConfig::new();
    apply_common_config(&mut client_config, common);

    client_config.set("group.id", &consumer.group_id);
    client_config.set(
        "auto.offset.reset",
        consumer.auto_offset_reset.as_kafka_value(),
    );
    client_config.set(
        "enable.auto.commit",
        consumer.enable_auto_commit.to_string(),
    );

    if let Some(session_timeout_ms) = consumer.session_timeout_ms {
        client_config.set("session.timeout.ms", session_timeout_ms.to_string());
    }
    if let Some(max_poll_interval_ms) = consumer.max_poll_interval_ms {
        client_config.set("max.poll.interval.ms", max_poll_interval_ms.to_string());
    }
    if let Some(fetch_min_bytes) = consumer.fetch_min_bytes {
        client_config.set("fetch.min.bytes", fetch_min_bytes.to_string());
    }

    client_config
}
