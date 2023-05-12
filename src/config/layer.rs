use serde::Deserialize;
use std::collections::HashMap;
use crate::data_subscriber::network_metadata_event::LayerTypes;

#[derive(Deserialize)]
pub struct LayerTypeConfiguration {
    pub(crate) layer_type: HashMap<LayerTypes, Vec<String>>
}
