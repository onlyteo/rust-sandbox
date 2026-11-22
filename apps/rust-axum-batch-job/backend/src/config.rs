use std::time::Duration;

use observability::config::ObservabilityConfig;
use serde::Deserialize;
use server::config::ServerConfig;
use yaml_config::{config::load_from_yaml, error::YamlConfigError};

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

/// Client side configuration for the external user API.
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalApiConfig {
    pub base_url: String,
    pub request_timeout_secs: u64,
}

/// Configuration for the in-process mock of the external user API.
#[derive(Debug, Clone, Deserialize)]
pub struct MockExternalApiConfig {
    pub host: String,
    pub port: u16,
    /// How often the mock randomly mutates its users.
    pub mutation_interval_secs: u64,
    /// Percentage (0-100) of known users mutated per mutation round.
    pub mutation_percent: u8,
    /// Percentage (0-100) of requests answered with HTTP 500.
    pub failure_percent: u8,
}

impl MockExternalApiConfig {
    pub fn server_config(&self) -> ServerConfig {
        ServerConfig {
            host: self.host.clone(),
            port: self.port,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SyncJobConfig {
    /// How often an instance tries to claim a new batch.
    pub tick_interval_secs: u64,
    /// Max rows claimed per tick.
    pub batch_size: i64,
    /// How long a claim is valid before other instances may reclaim the row.
    pub lease_secs: u64,
    /// Delay until a successfully synced user is due again.
    pub sync_interval_secs: u64,
    /// Max concurrent requests to the external API per instance.
    pub max_concurrency: usize,
    /// Upper bound of the exponential backoff after failures.
    pub max_backoff_secs: u64,
}

impl SyncJobConfig {
    pub fn tick_interval(&self) -> Duration {
        Duration::from_secs(self.tick_interval_secs)
    }

    pub fn lease(&self) -> Duration {
        Duration::from_secs(self.lease_secs)
    }

    pub fn sync_interval(&self) -> Duration {
        Duration::from_secs(self.sync_interval_secs)
    }

    pub fn max_backoff(&self) -> Duration {
        Duration::from_secs(self.max_backoff_secs)
    }
}

#[macro_export]
macro_rules! read_config_file {
    ($cfg_name:expr) => {
        if cfg!(feature = "docker") {
            include_str!(concat!("../config/docker/", $cfg_name))
        } else {
            include_str!(concat!("../config/local/", $cfg_name))
        }
    };
}

pub fn load_server_config() -> Result<ServerConfig, YamlConfigError> {
    load_from_yaml(read_config_file!("server_config.yaml"))
}

pub fn load_database_config() -> Result<DatabaseConfig, YamlConfigError> {
    load_from_yaml(read_config_file!("database_config.yaml"))
}

pub fn load_observability_config() -> Result<ObservabilityConfig, YamlConfigError> {
    load_from_yaml(read_config_file!("observability_config.yaml"))
}

pub fn load_external_api_config() -> Result<ExternalApiConfig, YamlConfigError> {
    load_from_yaml(read_config_file!("external_api_config.yaml"))
}

pub fn load_mock_external_api_config() -> Result<MockExternalApiConfig, YamlConfigError> {
    load_from_yaml(read_config_file!("mock_external_api_config.yaml"))
}

pub fn load_sync_job_config() -> Result<SyncJobConfig, YamlConfigError> {
    load_from_yaml(read_config_file!("sync_job_config.yaml"))
}
