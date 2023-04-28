use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashMap;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use crate::component_interaction::Highlightable;
use crate::metrics::{LayerMetrics, Metric, MetricState};

#[derive(Default, Component, Clone)]
pub struct Node {
    metrics: Metric,
    pub(crate) entity: Option<Entity>,
    layer_type: LayerType,
    layer_num: usize,
    connections: Vec<Entity>
}

#[derive(Default, Component)]
pub struct Layer {
    pub(crate) nodes: Vec<Node>,
    pub(crate) name: &'static str,
    pub(crate) layer_type: LayerType,
    pub(crate) layer_depth: u8,
    pub(crate) sub_layers: Vec<Layer>
}

#[derive(Default, Clone)]
pub enum LayerType {
    #[default]
    TFormer,
    FullyConnected,
    Normalization
}

#[derive(Default, Component)]
pub struct Network {
    layers: Vec<Layer>,
    network_id: &'static str
}

#[derive(Default)]
pub struct MetaNetwork {
    network_connections: HashMap<Entity, Entity>
}


