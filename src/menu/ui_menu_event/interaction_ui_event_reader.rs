use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Component, Style, Visibility, With};
use crate::event::event_actions::{InsertComponentInteractionEventReader, InteractionEventReader};
use crate::event::event_state::{ComponentChangeEventData, Context, NextComponentInsert, StyleStateChangeEventData};
use crate::menu::UiComponent;
use crate::menu::ui_menu_event::next_action::NextUiState;
use crate::menu::ui_menu_event::state_change_factory::StateChangeActionComponentStateFactory;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;

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
    AdviserComponentT: Component,
    Ctx: Context,
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
