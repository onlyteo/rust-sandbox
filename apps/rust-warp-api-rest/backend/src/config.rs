use observability::config::ObservabilityConfig;
use server::config::ServerConfig;
use yaml_config::{config::load_from_yaml, error::YamlConfigError};

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

pub fn load_observability_config() -> Result<ObservabilityConfig, YamlConfigError> {
    let contents = read_config_file!("observability_config.yaml");
    load_from_yaml(contents)
}
