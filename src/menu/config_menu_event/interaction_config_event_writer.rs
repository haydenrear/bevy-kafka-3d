use std::fmt::Debug;
use bevy::prelude::*;
use crate::event::event_actions::RetrieveState;
use crate::event::event_descriptor::EventDescriptor;
use crate::menu::{ConfigurationOption, ConfigurationOptionComponent, ConfigurationOptionEnum, DataType};
use crate::menu::config_menu_event::config_event::ConfigurationOptionEvent;
use crate::network::Node;

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionActionStateRetriever;

impl <T: Component + Send + Sync + Default + Clone + Debug + 'static> RetrieveState<
    ConfigurationOptionEvent<T>, DataType, ConfigurationOption<T>,
    (Entity, &ConfigurationOption<T>),
    (Entity, &Parent, &ConfigurationOption<T>),
    (Entity, &Children, &ConfigurationOption<T>),
    (With<ConfigurationOption<T>>),
    (With<Parent>, With<ConfigurationOption<T>>),
    (With<Children>, With<ConfigurationOption<T>>),
>
for ConfigOptionActionStateRetriever
{
    fn retrieve_state(
        commands: &mut Commands,
        entity: Entity,
        self_query: &Query<(Entity, &ConfigurationOption<T>), (With<ConfigurationOption<T>>)>,
        with_parent_query: &Query<
            (Entity, &Parent, &ConfigurationOption<T>),
            (With<Parent>, With<ConfigurationOption<T>>)
        >,
        with_child_query: &Query<
            (Entity, &Children, &ConfigurationOption<T>),
            (With<Children>, With<ConfigurationOption<T>>)
        >
    ) -> Option<EventDescriptor<DataType, ConfigurationOptionEvent<T>, ConfigurationOption<T>>> {
        // self_query.get(entity)
        //     .map(|entity_option| {
        //
        //     })
        None
    }
}