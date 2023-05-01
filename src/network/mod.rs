use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashMap;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::GeometryBuilder;
use bevy_prototype_lyon::shapes;
use crate::draw_network::{LAYER_SPACING, NODE_RADIUS, NODE_SPACING};

#[derive(Default, Component, Clone)]
pub struct Node {
    pub(crate) entity: Option<Entity>,
    pub(crate) connections: Vec<Entity>,
    layer_type: LayerType,
    layer_num: usize,
    pub(crate) node_pos: u8
}

#[derive(Default, Component, Clone)]
pub struct Layer {
    pub(crate) nodes: Vec<Node>,
    pub(crate) name: &'static str,
    pub(crate) layer_type: LayerType,
    pub(crate) layer_depth: u8,
    pub(crate) sub_layers: Vec<Layer>,
    pub(crate) network_id: NetworkId
}

#[derive(Default, Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
pub struct NetworkId {
    pub(crate) network_id: &'static str
}

impl NetworkId {
    pub(crate) fn new(network_id: &'static str) -> Self {
        Self {
            network_id
        }
    }
}

#[derive(Default, Clone)]
pub enum LayerType {
    TFormer,
    #[default]
    FullyConnected,
    Normalization
}

/// Based on the different type of network, different display.
impl LayerType {
    pub(crate) fn create_mesh(
        &self, layer: &Layer,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        // match self {
        //     LayerType::TFormer => {}
        //     LayerType::FullyConnected => {
        //
        //     }
        //     LayerType::Normalization => {}
        // }
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(NODE_RADIUS + 20.0, NODE_RADIUS * 2.0 * layer.nodes.len() as f32 + 100.0)))).into(),
            transform: Transform::from_xyz(layer.layer_depth as f32 * LAYER_SPACING, 0.0, 0.0),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            ..default()
        }
    }

    pub(crate) fn draw_node_mesh(
        &self,
        y: f32,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(30.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(0.0, y, 1.0),
            ..default()
        }
    }
}

#[derive(Default, Component, Clone)]
pub struct Network {
    pub(crate) layers: Vec<Entity>,
    pub(crate) network_id: NetworkId
}

impl Network {
    pub(crate) fn new(layers: Vec<Entity>, network_id: NetworkId) -> Self {
        Self {
            layers, network_id
        }
    }
}

#[derive(Default)]
pub struct MetaNetwork {
    network_connections: HashMap<Entity, Entity>
}


