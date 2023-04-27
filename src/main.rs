#![feature(default_free_fn)]

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_prototype_lyon::plugin::ShapePlugin;
use crate::camera::{camera_control, setup_camera, ZoomableDraggableCamera};
use crate::component_interaction::highlight_nodes;
use crate::initialize_test_plugin::add_node_entities;
use crate::metrics::{MetricsMetadataSubscription, MetricState, MetricsSubscription};
use crate::network::draw_layers;

mod config;
mod metrics;
mod network;
mod network_state;
mod initialize_test_plugin;
mod camera;
mod component_interaction;
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
        .add_system(draw_layers)
        .add_system(camera_control)
        .add_system(highlight_nodes)
        .run();
}