use std::fmt::Debug;
use bevy::prelude::*;
use crate::event::event_actions::RetrieveState;
use crate::event::event_descriptor::EventDescriptor;
use crate::menu::{MetricsConfigurationOption, ConfigurationOptionComponent, ConfigurationOptionEnum, DataType};
use crate::menu::config_menu_event::config_event::ConfigurationOptionEventArgs;
use crate::network::Node;

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionActionStateRetriever;

impl <T: Component + Send + Sync + Default + Clone + Debug + 'static> RetrieveState<
    ConfigurationOptionEventArgs<T>,
    DataType,
    MetricsConfigurationOption<T>,
    (Entity, &MetricsConfigurationOption<T>),
    (Entity, &Parent, &MetricsConfigurationOption<T>),
    (Entity, &Children, &MetricsConfigurationOption<T>),
    (With<MetricsConfigurationOption<T>>),
    (With<Parent>, With<MetricsConfigurationOption<T>>),
    (With<Children>, With<MetricsConfigurationOption<T>>),
>
for ConfigOptionActionStateRetriever
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        self_query: &Query<(Entity, &MetricsConfigurationOption<T>), (With<MetricsConfigurationOption<T>>)>,
        with_parent_query: &Query<
            (Entity, &Parent, &MetricsConfigurationOption<T>),
            (With<Parent>, With<MetricsConfigurationOption<T>>)
        >,
        with_child_query: &Query<
            (Entity, &Children, &MetricsConfigurationOption<T>),
            (With<Children>, With<MetricsConfigurationOption<T>>)
        >
    ) -> Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>> {
        // Get all of the node entities
        // self_query.get(entity)
        //     .map(|entity_option| {
        //
        //     })
        vec![]
    }
}