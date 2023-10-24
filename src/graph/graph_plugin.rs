use bevy::prelude::*;
use bevy_polyline::PolylinePlugin;
use bevy_polyline::prelude::PolylineMaterial;
use crate::graph::{GraphParent, setup_graph, GraphConfigurationResource, GraphingMetricsResource};
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::radial::RadialGraphPoints;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PolylinePlugin)
            .insert_resource(GraphConfigurationResource::<Node>::default())
            .add_startup_system(setup_graph::setup_graph)
            .add_system(draw_graph_points::<Node, RadialGraphPoints>)
        ;
    }
}
