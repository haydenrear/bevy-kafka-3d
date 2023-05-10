use bevy::prelude::*;
use crate::event::event_descriptor::EventDescriptor;
use crate::menu::config_menu_event::config_event::ConfigurationOptionEventArgs;
use crate::menu::config_menu_event::interaction_config_event_writer::{ConfigOptionActionStateRetriever, ConfigOptionContext};
use crate::menu::{DataType, Menu, MetricsConfigurationOption};


pub struct ConfigMenuEventPlugin;

impl Plugin for ConfigMenuEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ConfigurationOptionEventArgs<Node>>()
            .insert_resource(ConfigOptionActionStateRetriever::default())
            .add_system(crate::event::event_actions::click_write_events::<
                ConfigOptionActionStateRetriever, ConfigurationOptionEventArgs<Menu>,
                DataType, MetricsConfigurationOption<Menu>, ConfigOptionContext,
                // self query
                (Entity, &MetricsConfigurationOption<Menu>),
                // self filter
                (With<MetricsConfigurationOption<Menu>>),
                // parent query
                (Entity, &Parent, &MetricsConfigurationOption<Menu>),
                // parent filter
                (With<Parent>, With<MetricsConfigurationOption<Menu>>),
                // child query
                (Entity, &Children, &MetricsConfigurationOption<Menu>),
                // child filter
                (With<Children>, With<MetricsConfigurationOption<Menu>>),
                // interaction filter
                (With<MetricsConfigurationOption<Menu>>, With<Button>, Changed<Interaction>)
            >)
            .add_event::<EventDescriptor<DataType, ConfigurationOptionEventArgs<Menu>, MetricsConfigurationOption<Menu>>>();
    }
}
