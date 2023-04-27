#![feature(default_free_fn)]

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_prototype_lyon::plugin::ShapePlugin;
use crate::camera::{camera_control, setup_camera, ZoomableDraggableCamera};
use crate::component_interaction::highlight_nodes;
use crate::initialize_test_plugin::add_node_entities;
use crate::metrics::{MetricsMetadataSubscription, MetricState, MetricsSubscription};
use draw_network::draw_network;
use crate::draw_network::draw_node_connections;

mod config;
mod metrics;
mod network;
mod network_state;
mod initialize_test_plugin;
mod camera;
mod component_interaction;
mod draw_network;
mod test;

fn main() {
    App::new()
        .insert_resource(MetricState::default())
        .insert_resource(MetricsSubscription::default())
        .insert_resource(MetricsMetadataSubscription::default())
        .insert_resource(ZoomableDraggableCamera::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(add_node_entities)
        .add_system(draw_network)
        .add_system(camera_control)
        .add_system(highlight_nodes)
        .add_system(draw_node_connections)
        .run();
}