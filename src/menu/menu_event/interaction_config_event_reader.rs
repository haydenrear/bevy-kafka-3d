use bevy::prelude::*;
use crate::menu::menu_event::{ConfigurationOptionEvent, UiComponent, UiEventArgs};
use crate::network::Node;

fn read_configuration_event(
    mut commands: Commands,
    mut read_events: EventReader<ConfigurationOptionEvent<Node>>,
    mut query: ParamSet<(
        Query<(Entity, &UiComponent, &mut Style), (With<UiComponent>)>,
        Query<(Entity, &UiComponent, &mut BackgroundColor), (With<UiComponent>)>
    )>,
) {

}
