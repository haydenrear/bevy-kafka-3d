use std::marker::PhantomData;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::{Commands, Component, Entity, ResMut, World};
use bevy::ui::{Display, Style};
use crate::event::event_state::{Update, UpdateStateInPlace};
use crate::menu::{MetricsConfigurationOption, DataType};
use crate::menu::menu_resource::{CONCAVITY, METRICS};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{NextUiState, StyleContext};
use crate::metrics::network_metrics::HistoricalData;
use crate::network::Node;

mod test_arr;
mod test_data_subscriber;
mod test_convergence;

#[test]
fn test_update_state() {
    let mut next_state = NextUiState::ReplaceDisplay(Update {
        update_to: Some(Display::None),
    });
    let mut display = Display::Flex;
    let mut style = Style::default();
    style.display = display;
    next_state.update_state(&mut Commands::new(&mut CommandQueue::default(), &World::default()), &mut style, &mut None);

    assert_eq!(style.display, Display::None);
}

#[test]
fn test_update_state_config() {
    let config_option = MetricsConfigurationOption::Concavity(PhantomData::<Node>::default(), DataType::Number(Some(20.0)), CONCAVITY);
    let x = &mut MetricsConfigurationOption::Metrics(PhantomData::<Node>::default(), DataType::Number(Some(0.0)), METRICS);
    config_option.update_state(&mut Commands::new(&mut CommandQueue::default(), &World::default()),x, &mut None::<ResMut<StyleContext>>);
    if let MetricsConfigurationOption::Concavity(_, DataType::Number(Some(n)), id) = x {
        assert_eq!(*n, 20.0);
    } else {
        assert!(false);
    }
}


#[derive(Component, Default)]
pub struct TestComponent {

}