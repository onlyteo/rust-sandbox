# YAML Config

A tiny helper for reading a YAML file straight into any `Deserialize` struct.

```rust
use serde::Deserialize;
use yaml_config::load_from_yaml;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
}

let config: AppConfig = load_from_yaml(Path::new("application.yaml"))?;
```

Errors (`YamlConfigError`) distinguish an unreadable file (`Read`) from invalid YAML content or a shape mismatch with the target struct (`Parse`).

This crate has no opinion on what the YAML looks like or what struct it maps to — it composes with any app-defined config type, including ones that embed other libraries' config structs (e.g. `kafka_client::CommonConfig`).
