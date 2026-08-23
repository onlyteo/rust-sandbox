# Kafka Serde

A small, pluggable serialization layer for Kafka message payloads.

This crate defines the `KafkaSerde` trait — serialize a value to bytes, deserialize bytes back to a value — plus a couple of ready-made implementations. [`kafka_client`](../kafka_client)'s `KafkaProducer`/`KafkaConsumer` are generic over `KafkaSerde`, so swapping the wire format is a matter of swapping which codec you construct them with; the client itself has no opinion on the format.

## Codecs

* **JSON** (`JsonSerde`) — enabled by the `json` feature, on by default.
* **Avro** (`AvroSerde`) — enabled by the `avro` feature. Encodes/decodes raw Avro against a fixed `apache_avro::Schema` (no Confluent wire-format framing or schema registry lookup — bring your own if you need that).

Enable only what you use:

```toml
[dependencies]
kafka_serde = { path = "../kafka_serde", default-features = false, features = ["avro"] }
```

Since Cargo unifies features across a build, `kafka_client` can depend on `kafka_serde` with just `json` while your application also depends on `kafka_serde` with `avro` enabled — both crates share the same compiled `kafka_serde`, and `AvroSerde` becomes available.

## Usage

```rust
use kafka_serde::{JsonSerde, KafkaSerde};

let codec = JsonSerde;
let bytes = codec.serialize(&my_event)?;
let event: MyEvent = codec.deserialize(&bytes)?;
```

```rust
use kafka_serde::{AvroSerde, KafkaSerde};

let codec = AvroSerde::from_schema_str(MY_EVENT_SCHEMA)?;
let bytes = codec.serialize(&my_event)?;
let event: MyEvent = codec.deserialize(&bytes)?;
```

## Adding a new codec

Implement `KafkaSerde` for your own type:

```rust
use kafka_serde::{KafkaSerde, KafkaSerdeError};
use serde::Serialize;
use serde::de::DeserializeOwned;

struct MyCodec;

impl KafkaSerde for MyCodec {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<Vec<u8>, KafkaSerdeError> {
        todo!()
    }

    fn deserialize<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, KafkaSerdeError> {
        todo!()
    }
}
```

Then pass an instance of it to `kafka_client::KafkaProducer::new` / `KafkaConsumer::new` — no changes needed in `kafka_client` itself.
