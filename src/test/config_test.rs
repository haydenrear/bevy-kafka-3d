use bevy::log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use bevy::utils::HashMap;
use crate::config::ConfigurationProperties;
use crate::graph::GridAxis;

#[test]
fn test_serialize_config() {
    let config = ConfigurationProperties::read_config();
    assert_ne!(config.metrics.metric_type.len(), 0);
    assert_ne!(config.network.layer_type.len(), 0);
}

#[test]
fn test_serialize_config_matcher() {
    let read = ConfigurationProperties::read_config();
    assert_eq!(read.metrics.get_grid_axis("loss-hello"), GridAxis::Y);
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
