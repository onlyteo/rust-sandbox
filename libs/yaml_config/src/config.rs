use serde::de::DeserializeOwned;
use std::path::Path;

use crate::error::YamlConfigError;

pub fn load_from_yaml<T: DeserializeOwned>(path: &Path) -> Result<T, YamlConfigError> {
    let contents = std::fs::read_to_string(path)?;
    let value = serde_yaml::from_str(&contents)?;
    Ok(value)
}
