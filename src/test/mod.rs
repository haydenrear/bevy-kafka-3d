use std::marker::PhantomData;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::{Commands, Entity, World};
use bevy::ui::{Display, Style};
use crate::event::event_state::{Update, UpdateStateInPlace};
use crate::menu::{ConfigurationOption, DataType};
use crate::menu::menu_resource::{CONCAVITY, METRICS};
use crate::menu::ui_menu_event::ui_menu_event_plugin::NextUiState;
use crate::metrics::HistoricalData;
use crate::network::Node;

#[test]
fn test_historical_data() {
    let mut historical_data = HistoricalData::new(3);

    historical_data.push(1.0);
    historical_data.push(2.0);
    historical_data.push(3.0);

    assert_eq!(historical_data.get(0), Some(3.0));
    assert_eq!(historical_data.get(1), Some(2.0));
    assert_eq!(historical_data.get(2), Some(1.0));
    assert_eq!(historical_data.get(3), None);

    historical_data.push(4.0);

    assert_eq!(historical_data.get(0), Some(4.0));
    assert_eq!(historical_data.get(1), Some(3.0));
    assert_eq!(historical_data.get(2), Some(2.0));
    assert_eq!(historical_data.get(3), None);

    historical_data.push(5.0);

    assert_eq!(historical_data.get(0), Some(5.0));
    assert_eq!(historical_data.get(1), Some(4.0));
    assert_eq!(historical_data.get(2), Some(3.0));
    assert_eq!(historical_data.get(3), None);
}

#[test]
fn test_update_state() {
    let mut next_state = NextUiState::ReplaceDisplay(Update {
        update_to: Some(Display::None),
    });
    let mut display = Display::Flex;
    let mut style = Style::default();
    style.display = display;
    next_state.update_state(&mut Commands::new(&mut CommandQueue::default(), &World::default()), &mut style);

    assert_eq!(style.display, Display::None);
}

#[test]
fn test_update_state_config() {
    let config_option = ConfigurationOption::Concavity(PhantomData::<Node>::default(), DataType::Number(Some(20.0)), CONCAVITY);
    let x = &mut ConfigurationOption::Metrics(PhantomData::<Node>::default(), DataType::Number(Some(0.0)), METRICS);
    config_option.update_state(&mut Commands::new(&mut CommandQueue::default(), &World::default()),x);
    if let ConfigurationOption::Concavity(_, DataType::Number(Some(n)), id) = x {
        assert_eq!(*n, 20.0);
    } else {
        assert!(false);
    }
}
