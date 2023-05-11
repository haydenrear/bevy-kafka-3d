use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::fs::read_to_string;
use std::path::Path;
use bevy::log::error;
use bevy::prelude::Resource;
use serde::Deserialize;
use crate::data_subscriber::network_metadata_event::LayerTypes;
use crate::graph::GraphDimType;
use crate::menu::Menu;
use crate::metrics::network_metrics::{MetricType, MetricTypeMatcher};


#[derive(Deserialize, Resource)]
pub struct ConfigurationProperties {
    pub(crate) kafka: KafkaConfiguration,
    pub(crate) metrics: MetricsConfiguration,
    pub(crate) network: LayerTypeConfiguration
}

impl Default for ConfigurationProperties {
    fn default() -> Self {
        Self::read_config()
    }
}

impl ConfigurationProperties {

    pub(crate) fn read_config() -> ConfigurationProperties {
        let config_file = env::var("CONFIG_PROPS")
            .or(Ok::<String, VarError>("resources/config.toml".to_string()))
            .unwrap();
        let config = read_to_string(Path::new(&config_file))
            .map(|toml| toml::from_str::<ConfigurationProperties>(toml.as_str()).ok())
            .or_else(|e| {
                error!("Error reading configuration properties: {:?}.", e);
                Ok::<Option<ConfigurationProperties>, toml::de::Error>(Some(ConfigurationProperties::default()))
            })
            .ok()
            .flatten()
            .unwrap();
        config
    }
}

#[derive(Deserialize)]
pub struct KafkaConfiguration {
    pub(crate) hosts: Vec<String>,
    pub(crate) consumer_group_id: String,
    pub(crate) client_id: String
}

#[derive(Deserialize, Default)]
pub struct MetricsConfiguration {
    pub(crate) metric_type: HashMap<MetricTypeMatcher, String>,
    pub(crate) dim_type: HashMap<GraphDimType, Vec<String>>
}

#[derive(Deserialize)]
pub struct LayerTypeConfiguration {
    pub(crate) layer_type: HashMap<LayerTypes, Vec<String>>
}

impl Default for KafkaConfiguration {
    fn default() -> Self {
        Self {
            hosts: vec!["localhost:9092".to_string()],
            consumer_group_id: "consumer".to_string(),
            client_id: "nn-fe".to_string()
        }
    }
}
