use observability::config::ObservabilityConfig;
use serde::Deserialize;
use server::config::ServerConfig;
use yaml_config::{config::load_from_yaml, error::YamlConfigError};

#[derive(Deserialize)]
pub struct AppConfig {
    pub user: AppUserConfig,
    pub backend: AppBackendConfig,
}

#[derive(Deserialize)]
pub struct AppUserConfig {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AppBackendConfig {
    pub url: String,
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

pub fn load_app_config() -> Result<AppConfig, YamlConfigError> {
    let contents = read_config_file!("app_config.yaml");
    load_from_yaml(contents)
}

pub fn load_observability_config() -> Result<ObservabilityConfig, YamlConfigError> {
    let contents = read_config_file!("observability_config.yaml");
    load_from_yaml(contents)
}
