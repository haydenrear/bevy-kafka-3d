#![feature(core_intrinsics)]
#![feature(default_free_fn)]
#![feature(let_chains)]


extern crate core;

use bevy::prelude::*;
use bevy::ui::UiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_prototype_lyon::plugin::ShapePlugin;
use graph::setup_graph;
use crate::camera::{camera_control, setup_camera, ZoomableDraggableCamera};
use crate::initialize_test_plugin::add_node_entities;
use crate::draw_network::{create_network, draw_network_initial, draw_node_connections, update_network};
use menu::ui_menu_event::ui_menu_event_plugin::UiEventPlugin;
use lines::line_list::LineMaterial;
use metrics::network_metrics::{publish_metrics, update_metrics};
use crate::config::ConfigurationProperties;
use crate::data_subscriber::data_subscriber_plugin::DataSubscriberPlugin;
use crate::event::event_propagation::{component_propagation_system, PropagateComponentEvent};
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::graph_plugin::GraphPlugin;
use crate::graph::setup_graph::setup_graph;
use crate::menu::config_menu_event::config_menu_event_plugin::ConfigMenuEventPlugin;
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::menu::ui_menu_event::interaction_ui_event_writer::StateChangeActionTypeStateRetriever;
use crate::menu::menu_resource::MenuResource;

pub(crate) mod config;
pub(crate) mod lines;
pub(crate) mod metrics;
pub(crate) mod network;
pub(crate) mod network_state;
pub(crate) mod initialize_test_plugin;
pub(crate) mod camera;
pub(crate) mod draw_network;
pub(crate) mod ui_components;
pub(crate) mod menu;
pub(crate) mod event;
pub(crate) mod graph;
pub(crate) mod data_subscriber;
pub(crate) mod ndarray;
mod test;

#[tokio::main]
async fn main() {
    App::new()
        .insert_resource(ZoomableDraggableCamera{
            min_distance: -5000.0,
            max_distance: 1000.0,
            current_distance: -1000.0,
            zoom_sensitivity: 10.0,
            initialized: false,
           ..default()
        })
        .insert_resource(MenuResource::default())
        .insert_resource(ConfigOptionContext::default())
        .insert_resource(ConfigurationProperties::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(UiEventPlugin)
        .add_plugin(GraphPlugin)
        .add_plugin(DataSubscriberPlugin)
        .add_plugin(ConfigMenuEventPlugin)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_startup_system(setup_camera)
        .add_startup_system(add_node_entities)
        .add_system(update_network)
        .add_system(camera_control)
        .add_system(draw_node_connections)
        .add_system(create_network)
        .add_system(draw_network_initial)
        .add_system(update_metrics)
        .add_system(publish_metrics)
        .add_system(component_propagation_system)
        .add_event::<PropagateComponentEvent>()
        .run();
}


