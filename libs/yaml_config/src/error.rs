#[derive(Debug, thiserror::Error)]
pub enum YamlConfigError {
    #[error("failed to read yaml file: {0}")]
    Read(#[from] std::io::Error),

    #[error("failed to parse yaml: {0}")]
    Parse(#[from] serde_yaml::Error),
}
