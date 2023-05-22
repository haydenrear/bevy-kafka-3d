#![feature(core_intrinsics)]
#![feature(default_free_fn)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(associated_type_defaults)]


use bevy::ecs::schedule::SystemSetConfig;
use bevy::prelude::*;
use bevy::ui::UiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins, SelectionEvent};
use bevy_prototype_lyon::plugin::ShapePlugin;
use camera::lerping_camera::camera_control;
use graph::setup_graph;
use crate::camera::{setup_camera, ZoomableDraggableCamera};
use menu::ui_menu_event::ui_menu_event_plugin::UiEventPlugin;
use lines::line_list::LineMaterial;
use menu::ui_menu_event::ui_state_change::GlobalState;
use network::draw_network::{create_network, draw_network_initial, draw_node_connections, update_network};
use crate::camera::lerping_camera::camera_rotation_system;
use crate::camera::raycast_select::BevyPickingState;
use crate::config::ConfigurationProperties;
use crate::cursor_adapter::{calculate_picks, event_merge_propagate};
use crate::event::event_propagation::{component_propagation_system, SideEffectWriter};
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::graph_plugin::GraphPlugin;
use crate::graph::setup_graph::setup_graph;
use crate::interactions::InteractionEvent;
use crate::menu::config_menu_event::config_menu_event_plugin::ConfigMenuEventPlugin;
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::menu_resource::MenuResource;

pub(crate) mod config;
pub(crate) mod util;
pub(crate) mod lines;
pub(crate) mod metrics;
pub(crate) mod network;
pub(crate) mod camera;
pub(crate) mod ui_components;
pub(crate) mod menu;
pub(crate) mod event;
pub(crate) mod graph;
pub(crate) mod data_subscriber;
pub(crate) mod ndarray;
pub(crate) mod cursor_adapter;
pub(crate) mod interactions;
pub(crate) mod render_html;
pub(crate) mod test;

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
        .insert_resource(NetworkMenuResultBuilder::default())
        .insert_resource(GraphMenuResultBuilder::default())
        .insert_resource(ConfigurationProperties::default())
        .insert_resource(BevyPickingState::default())
        .insert_resource(GlobalState::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(UiEventPlugin)
        .add_plugin(GraphPlugin)
        // .add_plugin(DataSubscriberPlugin)
        .add_plugin(ConfigMenuEventPlugin)
        .add_plugin(MaterialPlugin::<LineMaterial>::default())
        .add_startup_system(setup_camera)
        .add_startup_system(test::test_plugin::add_node_entities)
        .add_system(calculate_picks)
        .add_system(camera_rotation_system)
        .add_system(update_network)
        .add_system(camera_control)
        .add_system(draw_node_connections)
        .add_system(create_network)
        .add_system(draw_network_initial)
        .add_system(component_propagation_system)
        .add_event::<SideEffectWriter>()
        .run();
}


