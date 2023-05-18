use bevy::prelude::*;
use crate::cursor_adapter::CursorResource;
use crate::event::event_actions::{ClickWriteEvents, InteractionEventReader};
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_state::NextStateChange;
use crate::graph::GraphParent;
use crate::menu::config_menu_event::config_event::{ConfigEventStateFactory, ConfigurationOptionChange, ConfigurationOptionEventArgs, NextConfigurationOptionState};
use crate::menu::config_menu_event::interaction_config_event_writer::{ConfigOptionActionStateRetriever, ConfigOptionContext};
use crate::menu::{DataType, Menu, MetricsConfigurationOption};
use crate::menu::config_menu_event::config_event_reader::ConfigEventReader;
use crate::menu::ui_menu_event::ui_state_change::GlobalState;
use crate::network::Network;

pub type MetricsSelfQueryFilter<T> = (With<MetricsConfigurationOption<T>>);
pub type MetricsSelfIxnQueryFilter<T> = (With<MetricsConfigurationOption<T>>, With<Button>, Changed<Interaction>);

pub struct ConfigMenuEventPlugin;

impl Plugin for ConfigMenuEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConfigurationOptionEventArgs<Node>>()
            .insert_resource(ConfigOptionActionStateRetriever::<Menu>::default())
            .add_system(ConfigOptionActionStateRetriever::<Menu>::click_write_events)
            .add_system(ConfigEventReader::<Menu>::read_events)
            .add_event::<EventDescriptor<DataType, ConfigurationOptionEventArgs<Menu>, MetricsConfigurationOption<Menu>>>();
    }
}


