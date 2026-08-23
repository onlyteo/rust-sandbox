use kafka_client::{CommonConfig, ConsumerConfig, KafkaConsumer, KafkaProducer, ProducerConfig};
use kafka_serde::{AvroSerde, KafkaSerde};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: i32,
    message: String,
}

fn common_config() -> CommonConfig {
    serde_yaml::from_str(r#"bootstrap.servers: "localhost:9092""#).unwrap()
}

fn event_schema() -> AvroSerde {
    let raw_schema = r#"
    {
        "type": "record",
        "name": "Event",
        "fields": [
            {"name": "id", "type": "int"},
            {"name": "message", "type": "string"}
        ]
    }
    "#;
    AvroSerde::from_schema_str(raw_schema).unwrap()
}

#[test]
fn producer_defaults_to_json_codec() {
    let common = common_config();
    let producer_config = ProducerConfig::default();

    let _producer = KafkaProducer::new_json(&common, &producer_config).unwrap();
}

#[tokio::test]
async fn producer_and_consumer_are_pluggable_with_an_avro_codec() {
    let common = common_config();
    let producer_config = ProducerConfig::default();
    let consumer_config: ConsumerConfig = serde_yaml::from_str(r#"group.id: "my-group""#).unwrap();

    let _producer: KafkaProducer<AvroSerde> =
        KafkaProducer::new(&common, &producer_config, event_schema()).unwrap();
    let _consumer: KafkaConsumer<AvroSerde> =
        KafkaConsumer::new(&common, &consumer_config, &["events"], event_schema()).unwrap();

    let event = Event {
        id: 1,
        message: "hello".to_string(),
    };
    let codec = event_schema();
    let bytes = codec.serialize(&event).unwrap();
    let decoded: Event = codec.deserialize(&bytes).unwrap();
    assert_eq!(decoded.id, event.id);
    assert_eq!(decoded.message, event.message);
}
