#![feature(core_intrinsics)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(associated_type_defaults)]


use bevy::ecs::schedule::SystemSetConfig;
use bevy::prelude::*;
use bevy::ui::UiPlugin;
use bevy::utils::petgraph::Graph;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{DefaultPickingPlugins};
use bevy_prototype_lyon::plugin::ShapePlugin;
use camera::lerping_camera::camera_control;
use graph::setup_graph;
use crate::camera::{NnFeCameraPlugin, setup_camera, ZoomableDraggableCamera};
use menu::ui_menu_event::ui_menu_event_plugin::UiEventPlugin;
use menu::ui_menu_event::ui_state_change::GlobalState;
use network::draw_network::{create_network, draw_network_initial, draw_node_connections, update_network};
use crate::camera::lerping_camera::camera_rotation_system;
use crate::camera::raycast_select::BevyPickingState;
use crate::config::ConfigurationProperties;
use crate::cursor_adapter::{calculate_picks, event_merge_propagate, PickableComponent};
use crate::data_subscriber::data_subscriber_plugin::DataSubscriberPlugin;
use crate::graph::draw_graph_points::draw_graph_points;
use crate::graph::graph_plugin::GraphPlugin;
use crate::graph::{GraphDimComponent, GraphingMetricsResource};
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
pub(crate) mod pickable_events;
pub(crate) mod interactions;
pub(crate) mod render_html;
pub(crate) mod test;

/// 1. At the beginning, insert all TransitionGroups, which are components used to determine when
///    to propagate. The TransitionGroups are used to query information to determine what changes
///    to make, create an EventDescriptor, and then propagate that EventDescriptor to an EventReader
///    to update the context (change component or change context, for instance).
/// 2. Retrieve the components that need to change and/or components that contain the state to
///    determine what to change to the context using the StateChangeActionTypeStateRetriever. This
///    retrieves the associated StateMachine which creates an EventDescriptor.
/// 3. Write the associated events (EventsSystem)
/// 4. Read the associated EventDescriptors in the various EventReaders and perform an action. The
///    associated EventReaders are InsertComponentInteractionEventReader, InteractionEventReader.
#[tokio::main]
async fn main() {
    App::new()
        .insert_resource(MenuResource::default())
        .insert_resource(NetworkMenuResultBuilder::default())
        .insert_resource(GraphMenuResultBuilder::default())
        .insert_resource(ConfigurationProperties::default())
        .insert_resource(BevyPickingState::default())
        .insert_resource(GlobalState::default())
        .insert_resource(GraphingMetricsResource::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(UiEventPlugin)
        .add_plugins(GraphPlugin)
        // .add_plugin(DataSubscriberPlugin)
        .add_plugins(ConfigMenuEventPlugin)
        .add_plugins(NnFeCameraPlugin)
        .add_startup_systems(test::test_plugin::add_node_entities)
        .add_system(calculate_picks)
        .add_system(update_network)
        .add_system(draw_node_connections)
        .add_system(create_network)
        .add_system(draw_network_initial)
        .add_event::<InteractionEvent<(With<PickableComponent>, With<GraphDimComponent>)>>()
        .run();
}


