use std::collections::HashMap;
use std::env;
use std::env::VarError;
use std::fs::read_to_string;
use std::path::Path;
use bevy::log::error;
use bevy::prelude::Resource;
use serde::Deserialize;
use kafka::KafkaConfiguration;
use layer::LayerTypeConfiguration;
use metrics::MetricsConfiguration;
use crate::data_subscriber::network_metadata_event::LayerTypes;
use crate::graph::GraphDimType;
use crate::menu::Menu;
use crate::metrics::network_metrics::{MetricType, MetricTypeMatcher};

pub(crate) mod kafka;
pub(crate) mod metrics;
pub(crate) mod layer;


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
                println!("Error reading configuration properties: {:?}.", e);
                Ok::<Option<ConfigurationProperties>, toml::de::Error>(Some(ConfigurationProperties::default()))
            })
            .ok()
            .flatten()
            .unwrap();
        config
    }
}
