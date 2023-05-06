use bevy::prelude::*;
use crate::graph::{Graph, GraphData, setup_graph};

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphData::default())
            .add_startup_system(setup_graph::setup_graph);
    }
}
