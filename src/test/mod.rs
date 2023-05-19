use std::marker::PhantomData;
use bevy::ecs::system::CommandQueue;
use bevy::prelude::{Commands, Component, Entity, ResMut, World};
use bevy::ui::{Display, Style};
use bevy_inspector_egui::egui::CursorIcon::ResizeNorth;
use crate::event::event_state::{Update, UpdateStateInPlace};
use crate::menu::{DataType, MetricsConfigurationOption};
use crate::menu::menu_resource::{CONCAVITY, METRICS};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::next_action::NextUiState;
use crate::metrics::network_metrics::HistoricalData;
use crate::network::Node;

#[cfg(test)]
mod test_arr;
#[cfg(test)]
mod test_convergence;
#[cfg(test)]
mod bevy_tests;
#[cfg(test)]
mod config_test;
#[cfg(test)]
mod test_interpolate;


pub(crate) mod test_plugin;

#[derive(Component, Default)]
pub struct TestComponent {
}