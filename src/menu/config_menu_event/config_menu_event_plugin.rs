use bevy::prelude::*;
use crate::cursor_adapter::event_merge_propagate;
use crate::event::event_actions::{EventsSystem, InteractionEventReader};
use crate::event::event_descriptor::EventDescriptor;
use crate::interactions::InteractionEvent;
use crate::menu::config_menu_event::config_event::{ConfigurationOptionEventArgs};
use crate::menu::config_menu_event::interaction_config_event_writer::{ConfigOptionActionStateRetriever};
use crate::menu::{DataType, Menu, MetricsConfigurationOption};
use crate::menu::config_menu_event::config_event_reader::ConfigEventReader;

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
            .add_system(event_merge_propagate::<MetricsSelfIxnQueryFilter<Menu>>)
            .add_event::<InteractionEvent<MetricsSelfIxnQueryFilter<Menu>>>()
            .add_event::<EventDescriptor<DataType, ConfigurationOptionEventArgs<Menu>, MetricsConfigurationOption<Menu>>>();
    }
}


