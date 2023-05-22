use bevy::prelude::*;
use crate::graph::{GraphParent, setup_graph, GraphConfigurationResource, GraphingMetricsResource};
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::radial::RadialGraphPoints;
use crate::lines::line_list::LineMaterial;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GraphConfigurationResource::<Node>::default())
            .insert_resource(GraphingMetricsResource::default())
            .add_startup_system(setup_graph::setup_graph)
            .add_system(draw_graph_points::<Node, RadialGraphPoints, LineMaterial>)
        ;
    }
}
