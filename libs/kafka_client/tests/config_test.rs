use kafka_client::{
    Acks, AutoOffsetReset, CommonConfig, CompressionType, ConsumerConfig, ProducerConfig,
    SaslMechanism, SecurityProtocol, build_consumer_client_config, build_producer_client_config,
};

#[test]
fn deserializes_common_config_with_defaults() {
    let yaml = r#"
bootstrap.servers: "localhost:9092"
"#;
    let common: CommonConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(common.bootstrap_servers, "localhost:9092");
    assert_eq!(common.client_id, None);
    assert_eq!(common.security_protocol, SecurityProtocol::Plaintext);
    assert_eq!(common.sasl_mechanism, None);
}

#[test]
fn deserializes_common_config_with_sasl_ssl() {
    let yaml = r#"
bootstrap.servers: "broker:9093"
client.id: "my-app"
security.protocol: sasl_ssl
sasl.mechanism: scram-sha-512
sasl.username: "user"
sasl.password: "pass"
ssl.ca.location: "/etc/ssl/ca.pem"
"#;
    let common: CommonConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(common.client_id, Some("my-app".to_string()));
    assert_eq!(common.security_protocol, SecurityProtocol::SaslSsl);
    assert_eq!(common.sasl_mechanism, Some(SaslMechanism::ScramSha512));
    assert_eq!(common.sasl_username, Some("user".to_string()));
    assert_eq!(common.sasl_password, Some("pass".to_string()));
    assert_eq!(common.ssl_ca_location, Some("/etc/ssl/ca.pem".to_string()));
}

#[test]
fn deserializes_producer_config_with_defaults() {
    let producer: ProducerConfig = serde_yaml::from_str("{}").unwrap();

    assert_eq!(producer.acks, Acks::All);
    assert_eq!(producer.compression_type, CompressionType::None);
    assert_eq!(producer.message_timeout_ms, None);
}

#[test]
fn deserializes_producer_config_with_overrides() {
    let yaml = r#"
acks: one
compression.type: snappy
message.timeout.ms: 5000
retries: 3
linger.ms: 10
batch.size: 16384
enable.idempotence: true
"#;
    let producer: ProducerConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(producer.acks, Acks::One);
    assert_eq!(producer.compression_type, CompressionType::Snappy);
    assert_eq!(producer.message_timeout_ms, Some(5000));
    assert_eq!(producer.retries, Some(3));
    assert_eq!(producer.linger_ms, Some(10));
    assert_eq!(producer.batch_size, Some(16384));
    assert_eq!(producer.enable_idempotence, Some(true));
}

#[test]
fn deserializes_consumer_config_with_defaults() {
    let yaml = r#"
group.id: "my-group"
"#;
    let consumer: ConsumerConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(consumer.group_id, "my-group");
    assert_eq!(consumer.auto_offset_reset, AutoOffsetReset::Earliest);
    assert!(consumer.enable_auto_commit);
    assert_eq!(consumer.session_timeout_ms, None);
}

#[test]
fn deserializes_consumer_config_with_overrides() {
    let yaml = r#"
group.id: "my-group"
auto.offset.reset: latest
enable.auto.commit: false
session.timeout.ms: 45000
max.poll.interval.ms: 300000
fetch.min.bytes: 1024
"#;
    let consumer: ConsumerConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(consumer.auto_offset_reset, AutoOffsetReset::Latest);
    assert!(!consumer.enable_auto_commit);
    assert_eq!(consumer.session_timeout_ms, Some(45000));
    assert_eq!(consumer.max_poll_interval_ms, Some(300000));
    assert_eq!(consumer.fetch_min_bytes, Some(1024));
}

#[test]
fn builds_producer_client_config_with_expected_properties() {
    let common: CommonConfig = serde_yaml::from_str(
        r#"
bootstrap.servers: "localhost:9092"
client.id: "producer-app"
"#,
    )
    .unwrap();
    let producer: ProducerConfig = serde_yaml::from_str(
        r#"
acks: all
compression.type: snappy
message.timeout.ms: 5000
"#,
    )
    .unwrap();

    let client_config = build_producer_client_config(&common, &producer);

    assert_eq!(
        client_config.get("bootstrap.servers"),
        Some("localhost:9092")
    );
    assert_eq!(client_config.get("client.id"), Some("producer-app"));
    assert_eq!(client_config.get("acks"), Some("all"));
    assert_eq!(client_config.get("compression.type"), Some("snappy"));
    assert_eq!(client_config.get("message.timeout.ms"), Some("5000"));
}

#[test]
fn builds_consumer_client_config_with_expected_properties() {
    let common: CommonConfig = serde_yaml::from_str(
        r#"
bootstrap.servers: "localhost:9092"
"#,
    )
    .unwrap();
    let consumer: ConsumerConfig = serde_yaml::from_str(
        r#"
group.id: "my-group"
auto.offset.reset: earliest
enable.auto.commit: false
"#,
    )
    .unwrap();

    let client_config = build_consumer_client_config(&common, &consumer);

    assert_eq!(
        client_config.get("bootstrap.servers"),
        Some("localhost:9092")
    );
    assert_eq!(client_config.get("group.id"), Some("my-group"));
    assert_eq!(client_config.get("auto.offset.reset"), Some("earliest"));
    assert_eq!(client_config.get("enable.auto.commit"), Some("false"));
}
