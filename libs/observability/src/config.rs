use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObservabilityConfig {
    pub logging: LoggingConfig,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub directives: Option<Vec<String>>,
}
