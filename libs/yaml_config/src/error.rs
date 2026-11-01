#[derive(Debug, thiserror::Error)]
pub enum YamlConfigError {
    #[error("Failed to read YAML file: {0}")]
    Read(#[from] std::io::Error),

    #[error("Failed to parse YAML: {0}")]
    Parse(#[from] serde_yaml::Error),
}
