use bevy::prelude::{Component, Entity, Resource};
use crate::graph::GraphDim;
use crate::menu::MenuData;

#[derive(Resource)]
pub struct GraphBuilder {
    graph_name: String,
    metric: Entity,
    dimensions: Vec<GraphDim>
}

#[derive(Resource)]
pub struct GraphBuilders {
    graph_builders: Vec<GraphBuilder>
}

#[derive(Resource)]
pub struct GraphMenuResource {
    pub(crate) data: MenuData
}

#[derive(Component)]
pub struct GraphingPotential {
    realized: bool
}