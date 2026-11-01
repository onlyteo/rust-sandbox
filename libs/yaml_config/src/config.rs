use serde::de::DeserializeOwned;

use crate::error::YamlConfigError;

pub fn load_from_yaml<T: DeserializeOwned>(contents: &str) -> anyhow::Result<T, YamlConfigError> {
    let config = serde_yaml::from_str(contents).map_err(YamlConfigError::Parse)?;
    Ok(config)
}
