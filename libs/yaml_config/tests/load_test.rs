use serde::Deserialize;
use yaml_config::{config::load_from_yaml, error::YamlConfigError};

#[derive(Debug, PartialEq, Deserialize)]
struct SampleConfig {
    name: String,
    port: u16,
    tags: Vec<String>,
}

#[test]
fn loads_struct_from_yaml_file() {
    let contents = include_str!("./resources/sample.yaml");
    let config: SampleConfig = load_from_yaml(contents).expect("Could not load config file");

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
fn returns_parse_error_for_malformed_yaml() {
    let contents = include_str!("./resources/malformed_sample.yaml");
    let result: Result<SampleConfig, YamlConfigError> = load_from_yaml(contents);

    assert!(matches!(result, Err(YamlConfigError::Parse(_))));
}
