# Kafka Client

A thin, typed wrapper around [`rdkafka`](https://docs.rs/rdkafka) that makes it easy to configure and construct Kafka producers and consumers from YAML.

The library owns the "how do I turn config into a working client" problem. The YAML files themselves live in whichever application embeds this crate's config structs — `kafka_client` has no opinion on your application's overall config schema. Reading a YAML file into that config struct is handled by the sibling crate [`yaml_config`](../yaml_config), not by this one.

Payload serialization is handled by the sibling crate [`kafka_serde`](../kafka_serde), which `KafkaProducer`/`KafkaConsumer` are generic over — pick JSON, Avro, or bring your own codec.

## Prerequisites

This crate links dynamically against a system-installed `librdkafka`. Install it before building:

- Arch Linux: `sudo pacman -S librdkafka pkgconf`
- Debian/Ubuntu: `sudo apt install librdkafka-dev pkg-config`
- macOS (Homebrew): `brew install librdkafka pkg-config`

## Usage

Embed `CommonConfig`, and whichever of `ProducerConfig` / `ConsumerConfig` you need, into your application's own config struct:

```rust
use kafka_client::{CommonConfig, ProducerConfig, ConsumerConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    kafka: KafkaSection,
}

#[derive(Debug, Deserialize)]
struct KafkaSection {
    common: CommonConfig,
    producer: ProducerConfig,
    consumer: ConsumerConfig,
}
```

With a matching `application.yaml` in the application:

```yaml
kafka:
  common:
    bootstrap.servers: localhost:9092
    client.id: my-app
    security.protocol: plaintext
  producer:
    acks: all
    compression.type: snappy
    message.timeout.ms: 5000
  consumer:
    group.id: my-group
    auto.offset.reset: earliest
    enable.auto.commit: false
```

Loading it, using [`yaml_config`](../yaml_config):

```rust
let config: AppConfig = yaml_config::load_from_yaml(Path::new("application.yaml"))?;
```

### Producing

`KafkaProducer` defaults to JSON (`kafka_serde::JsonSerde`) via `new_json`:

```rust
use kafka_client::KafkaProducer;

let producer = KafkaProducer::new_json(&config.kafka.common, &config.kafka.producer)?;
producer.send_payload("my-topic", Some("some-key"), &my_event).await?;
```

### Consuming

The library builds and subscribes the client; your application owns the message loop, so it composes naturally with a shutdown signal (e.g. `libs/async_handler::task::shutdown_signal`):

```rust
use futures::StreamExt;
use kafka_client::KafkaConsumer;

let consumer = KafkaConsumer::new_json(&config.kafka.common, &config.kafka.consumer, &["my-topic"])?;
let mut stream = consumer.stream();

while let Some(message) = stream.next().await {
    let message = message?;
    let event: MyEvent = consumer.deserialize_payload(&message)?;
    // handle event
}
```

### Plugging in a different serde codec

`KafkaProducer<S>` and `KafkaConsumer<S>` are generic over any `S: kafka_serde::KafkaSerde`. `new_json` is just a convenience for `S = JsonSerde`; to use Avro (or your own codec), construct it and pass it to `new`:

```rust
use kafka_client::{KafkaConsumer, KafkaProducer};
use kafka_serde::AvroSerde;

let codec = AvroSerde::from_schema_str(MY_EVENT_SCHEMA)?;

let producer = KafkaProducer::new(&config.kafka.common, &config.kafka.producer, codec.clone())?;
let consumer = KafkaConsumer::new(&config.kafka.common, &config.kafka.consumer, &["my-topic"], codec)?;
```

See [`kafka_serde`](../kafka_serde) for the available codecs and how to implement your own.

## Configuration reference

Only the properties commonly tuned in practice are typed. Anything else can still be set directly on the `rdkafka::ClientConfig` returned by `build_producer_client_config`/`build_consumer_client_config` if needed.

**Common** (`CommonConfig`): `bootstrap.servers` (required), `client.id`, `security.protocol` (`plaintext` default, `ssl`, `sasl_plaintext`, `sasl_ssl`), `sasl.mechanism` (`plain`, `scram-sha-256`, `scram-sha-512`), `sasl.username`, `sasl.password`, `ssl.ca.location`.

**Producer** (`ProducerConfig`): `acks` (`all` default, `one`, `none`), `compression.type` (`none` default, `gzip`, `snappy`, `lz4`, `zstd`), `message.timeout.ms`, `retries`, `linger.ms`, `batch.size`, `enable.idempotence`.

**Consumer** (`ConsumerConfig`): `group.id` (required), `auto.offset.reset` (`earliest` default, `latest`, `none`), `enable.auto.commit` (`true` default), `session.timeout.ms`, `max.poll.interval.ms`, `fetch.min.bytes`.
