use std::marker::PhantomData;
use bevy::prelude::{Component, Style, With};
use crate::event::event_actions::InteractionEventReader;
use crate::event::event_state::StyleStateChangeEventData;
use crate::menu::config_menu_event::config_event::{ConfigEventStateFactory, ConfigurationOptionEventArgs, NextConfigurationOptionState};
use crate::menu::{DataType, MetricsConfigurationOption, UiComponent};
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::menu::ui_menu_event::next_action::NextUiState;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::types::StyleStateChange;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionComponentStateFactory, StateChangeActionType, UiEventArgs};

pub struct UiEventReader {}

impl InteractionEventReader<
    StyleStateChangeEventData, UiEventArgs, Style, Style,
    StateChangeActionComponentStateFactory,
    NextUiState, UiContext, (With<UiComponent>)
> for UiEventReader
{}
