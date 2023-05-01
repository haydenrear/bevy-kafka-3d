#![feature(default_free_fn)]

use bevy::prelude::*;
use bevy::ui::UiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_prototype_lyon::plugin::ShapePlugin;
use crate::camera::{camera_control, setup_camera, ZoomableDraggableCamera};
use crate::initialize_test_plugin::add_node_entities;
use crate::metrics::{MetricsMetadataSubscription, MetricState, MetricsSubscription, update_metrics, publish_metrics};
use crate::draw_network::{create_network, draw_network_initial, draw_node_connections, update_network};
use menu::menu_event::UiEventPlugin;
use crate::menu::menu_resource::MenuResource;

mod config;
mod metrics;
mod network;
mod network_state;
mod initialize_test_plugin;
mod camera;
mod draw_network;
mod visualization;
mod menu;
mod test;

fn main() {
    App::new()
        .insert_resource(MetricState::default())
        .insert_resource(MetricsSubscription::default())
        .insert_resource(MetricsMetadataSubscription::default())
        .insert_resource(ZoomableDraggableCamera::default())
        .insert_resource(MenuResource::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(UiEventPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(add_node_entities)
        .add_system(update_network)
        .add_system(camera_control)
        .add_system(draw_node_connections)
        .add_system(create_network)
        .add_system(draw_network_initial)
        .add_system(update_metrics)
        .add_system(publish_metrics)
        .run();
}