use observability::config::ObservabilityConfig;
use serde::Deserialize;
use yaml_config::{config::load_from_yaml, error::YamlConfigError};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
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
    let contents = read_config_file!("server_config.yaml");
    load_from_yaml(contents)
}

pub fn load_database_config() -> Result<DatabaseConfig, YamlConfigError> {
    let contents = read_config_file!("database_config.yaml");
    load_from_yaml(contents)
}

pub fn load_observability_config() -> Result<ObservabilityConfig, YamlConfigError> {
    let contents = read_config_file!("observability_config.yaml");
    load_from_yaml(contents)
}
