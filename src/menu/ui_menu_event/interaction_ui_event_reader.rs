use std::marker::PhantomData;
use bevy::prelude::{Component, Style, With};
use crate::event::event_actions::InteractionEventReader;
use crate::menu::config_menu_event::config_event::{ConfigEventStateFactory, ConfigurationOptionEventArgs, NextConfigurationOptionState};
use crate::menu::{DataType, MetricsConfigurationOption, UiComponent};
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{NextUiState, StateChangeActionComponentStateFactory, StateChangeActionType, StyleContext, UiEventArgs};

pub struct UiEventReader {}

impl InteractionEventReader<
    StateChangeActionType, UiEventArgs, Style, Style,
    StateChangeActionComponentStateFactory,
    NextUiState, StyleContext, (With<UiComponent>)
> for UiEventReader
{}
