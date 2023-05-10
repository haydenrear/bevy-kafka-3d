use bevy::prelude::*;
use crate::graph::Graph;
use crate::menu::config_menu_event::config_event::ConfigurationOptionEventArgs;
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiComponent, UiEventArgs};
use crate::network::Node;

fn read_configuration_event(
    mut commands: Commands,
    mut read_events: EventReader<ConfigurationOptionEventArgs<Graph<Menu>>>,
    mut query: ParamSet<(
        Query<(Entity, &UiComponent, &mut Style), (With<UiComponent>)>,
        Query<(Entity, &UiComponent, &mut BackgroundColor), (With<UiComponent>)>
    )>,
) {
    for event in read_events.iter() {
        if let ConfigurationOptionEventArgs::Event(config_option_change, entity) = event {
        }
    }
}

pub fn read_item(
    mut commands: Commands,
    query: Query<(Entity, &MetricsConfigurationOption<Menu>, &Interaction),
        (Changed<Interaction>, With<Button>)>,
) {
    for event in query.iter() {
        if let Interaction::Clicked = event.2 {
            info!("Found a metrics config option.!")
        }
    }
}

