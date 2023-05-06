use std::fs::read_to_string;
use std::path::Path;
use crate::data_subscriber::kafka_data_subscriber::ConfigurationProperties;

#[test]
fn test_serialize_config() {
    let path = Path::new("resources/config.toml");
    assert!(path.exists());
    let config = read_to_string(path)
        .map(|toml| toml::from_str::<ConfigurationProperties>(toml.as_str())
            .or_else(|e| {
                println!("{:?} is error", e);
                Err(e)
            })
            .ok()
        )
        .or_else(|e| {
            println!("{:?} is error", e);
            Err(e)
        })
        .ok()
        .flatten();
    assert!(config.is_some());
}