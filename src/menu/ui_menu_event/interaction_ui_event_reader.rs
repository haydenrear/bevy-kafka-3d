use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Component, Style, Visibility, With};
use crate::event::event_actions::{InsertComponentInteractionEventReader, InteractionEventReader};
use crate::event::event_state::{ComponentChangeEventData, Context, NextComponentInsert, StyleStateChangeEventData};
use crate::menu::config_menu_event::config_event::{ConfigEventStateFactory, ConfigurationOptionEventArgs, NextConfigurationOptionState};
use crate::menu::{DataType, MetricsConfigurationOption, UiComponent};
use crate::menu::config_menu_event::interaction_config_event_writer::NetworkMenuResultBuilder;
use crate::menu::ui_menu_event::next_action::NextUiState;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::types::StyleStateChange;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionComponentStateFactory, StateChangeActionType, UiEventArgs};

pub struct UiEventReader;

impl InteractionEventReader<
    StyleStateChangeEventData, UiEventArgs, Style, Style,
    StateChangeActionComponentStateFactory,
    NextUiState, UiContext,
    (With<UiComponent>)
> for UiEventReader
{}

pub struct ComponentChangeEventReader<NextEventComponentT, AdviserComponentT, Ctx>
where
    NextEventComponentT: Component,
    Ctx: Context,
    AdviserComponentT: Component
{
    insert_component: PhantomData<NextEventComponentT>,
    ctx: PhantomData<Ctx>,
    adviser: PhantomData<AdviserComponentT>
}

impl InsertComponentInteractionEventReader<
    ComponentChangeEventData, UiEventArgs, Visibility, Visibility,
    StateChangeActionComponentStateFactory,
    NextComponentInsert<Visibility, Visibility, UiContext>, UiContext,
    (With<Visibility>)
> for ComponentChangeEventReader<Visibility, Visibility, UiContext>
{}
