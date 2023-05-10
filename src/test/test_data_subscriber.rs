use std::fs::read_to_string;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use bevy::utils::HashMap;
use bevy::log::{error, info};
use crate::config::ConfigurationProperties;

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
    assert_ne!(config.as_ref().unwrap().metrics.metric_type.len(), 0);
    assert_ne!(config.as_ref().unwrap().network.layer_type.len(), 0);
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Test {
    pub(crate) shape: Vec<usize>,
    pub(crate) data: Mutex<Option<Vec<f32>>>,
    pub(crate) metric_name: String,
    pub(crate) included: Vec<u32>,
    pub(crate) columns: Option<HashMap<String, usize>>
}

impl Test {
    pub(crate) fn get_data(&self) -> Vec<f32> {
        let mut return_val = None;
        let _ = self.data.lock()
            .as_mut()
            .map(|inner| {
                std::mem::replace(&mut return_val, inner.take())
            })
            .or_else(|err| {
                error!("Error replacing data: {:?}.", err);
                Err(err)
            });
        return_val.or(Some(vec![]))
            .unwrap()
    }
}

pub(crate) fn get_metric_message_test(in_value: &str) -> Option<Test> {
    serde_json::from_str::<Test>(in_value)
        .or_else(|err| {
            println!("Error deserializing array: {:?}", err);
            info!("Error deserializing array: {:?}", err);
            Err(err)
        })
        .ok()
}

