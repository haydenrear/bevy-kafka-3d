use bevy::prelude::Event;
use bevy::utils::HashMap;
use serde::Deserialize;
use crate::data_subscriber::metric_event::NetworkEvent;

#[derive(Deserialize, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LayerTypes {
    #[default]
    FullyConnected,
    Normalization,
    AttentionEncoder,
    AttentionDecoder,
}

#[derive(Deserialize, Default, Clone, Event)]
pub struct NetworkMetadataEvent {
    name: String,
    depends_on: Vec<String>,
    topic: String,
    layer_type: String,
    dependencies: Vec<String>
}

impl NetworkEvent for NetworkMetadataEvent {
    fn topic_matcher() -> &'static str {
        "network_changes"
    }
}

