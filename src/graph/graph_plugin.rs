use bevy::prelude::*;
use crate::graph::{Graph, GraphData, setup_graph};
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::radial::RadialGraphPoints;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphData::default())
            .add_startup_system(setup_graph::setup_graph)
            .add_system(draw_graph_points::<Node, RadialGraphPoints>)
        ;
    }
}
