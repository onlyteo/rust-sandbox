use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Greeting {
    id: i32,
    message: String,
}

#[cfg(feature = "json")]
#[test]
fn json_serde_round_trips_a_value() {
    use kafka_serde::{JsonSerde, KafkaSerde};

    let codec = JsonSerde;
    let greeting = Greeting {
        id: 1,
        message: "hello".to_string(),
    };

    let bytes = codec.serialize(&greeting).unwrap();
    let decoded: Greeting = codec.deserialize(&bytes).unwrap();

    assert_eq!(decoded, greeting);
}

#[cfg(feature = "avro")]
#[test]
fn avro_serde_round_trips_a_value() {
    use kafka_serde::{AvroSerde, KafkaSerde};

    let raw_schema = r#"
    {
        "type": "record",
        "name": "Greeting",
        "fields": [
            {"name": "id", "type": "int"},
            {"name": "message", "type": "string"}
        ]
    }
    "#;
    let codec = AvroSerde::from_schema_str(raw_schema).unwrap();
    let greeting = Greeting {
        id: 1,
        message: "hello".to_string(),
    };

    let bytes = codec.serialize(&greeting).unwrap();
    let decoded: Greeting = codec.deserialize(&bytes).unwrap();

    assert_eq!(decoded, greeting);
}
