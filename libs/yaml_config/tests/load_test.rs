use std::path::{Path, PathBuf};

use serde::Deserialize;
use yaml_config::{config::load_from_yaml, error::YamlConfigError};

#[derive(Debug, PartialEq, Deserialize)]
struct SampleConfig {
    name: String,
    port: u16,
    tags: Vec<String>,
}

fn temp_file_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "yaml_config_test_{}_{}.yaml",
        std::process::id(),
        name
    ))
}

#[test]
fn loads_struct_from_yaml_file() {
    let config: SampleConfig = load_from_yaml(Path::new("tests/resources/sample.yaml")).unwrap();

    assert_eq!(
        config,
        SampleConfig {
            name: "sample-app".to_string(),
            port: 8080,
            tags: vec!["alpha".to_string(), "beta".to_string()],
        }
    );
}

#[test]
fn returns_io_error_for_missing_file() {
    let path = Path::new("tests/resources/does-not-exist.yaml");

    let result: Result<SampleConfig, YamlConfigError> = load_from_yaml(path);

    assert!(matches!(result, Err(YamlConfigError::Read(_))));
}

#[test]
fn returns_parse_error_for_malformed_yaml() {
    let path = temp_file_path("malformed");
    std::fs::write(&path, "name: [unterminated").unwrap();

    let result: Result<SampleConfig, YamlConfigError> = load_from_yaml(&path);

    std::fs::remove_file(&path).unwrap();
    assert!(matches!(result, Err(YamlConfigError::Parse(_))));
}
