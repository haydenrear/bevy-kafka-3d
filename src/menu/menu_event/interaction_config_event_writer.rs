use bevy::prelude::*;
use crate::menu::ConfigurationOptionComponent;
use crate::menu::menu_event::ConfigurationOptionEvent;
use crate::network::Node;

/// When an interaction happens with a ConfigurationOptionComponent, then the following happens
/// 1.
pub fn write_config_events<T: Component>(
    mut commands: Commands,
    mut event_write: EventWriter<ConfigurationOptionEvent<T>>,
    entity_query: Query<(Entity, &Style), (With<ConfigurationOptionComponent<Node>>)>,
    with_children_query: Query<(Entity, &ConfigurationOptionComponent<T>, &Children), (With<ConfigurationOptionComponent<T>>, With<Children>)>,
    with_parent_query: Query<(Entity, &ConfigurationOptionComponent<T>, &Parent), (With<ConfigurationOptionComponent<T>>, With<Parent>)>,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>)>,
) {

}
