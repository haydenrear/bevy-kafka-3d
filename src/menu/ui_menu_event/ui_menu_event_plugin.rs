use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy::hierarchy::{Children, Parent};
use std::fmt::{Debug, Formatter};
use bevy::time::TimerMode;
use crate::event::event_actions::{InteractionEventReader, StateChangeEventReader};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::{HoverStateChange, NextStateChange, StateChange, StateChangeFactory, Update, UpdateStateInPlace};
use crate::menu::{CollapsableMenu, Dropdown, DropdownOption, ui_menu_event};
use crate::menu::ui_menu_event::interaction_ui_event_writer::StateChangeActionTypeStateRetriever;
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::UiClickStateChange;
use crate::visualization::{create_dropdown, UiIdentifiableComponent};

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_dropdown)
            .insert_resource(StateChangeActionTypeStateRetriever::default())
            .add_system(crate::event::event_actions::write_events::<
                StateChangeActionTypeStateRetriever, UiEventArgs, StateChangeActionType, Style,
                // self query
                (Entity, &UiComponent, &Style, &UiIdentifiableComponent),
                // self filter
                (With<UiComponent>, With<Style>),
                // parent query
                (Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style),
                // parent filter
                (With<UiComponent>, With<Parent>, With<Style>),
                // child query
                (Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style),
                // child filter
                (With<UiComponent>, With<Children>, With<Style>),
                // interaction filter
                (With<UiComponent>, With<Button>, Changed<Interaction>)
            >)
            .add_system(<StateChangeEventReader as InteractionEventReader<
                StateChangeActionType, UiEventArgs, Style, Style,
                StateChangeActionComponentStateFactory,
                NextUiState, (With<UiComponent>)
            >>::read_events)
            .add_system(ui_state_change::hover_event)
            .add_event::<UiEventArgs>()
            .add_event::<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>();
    }
}

#[derive(Resource, Default, Clone)]
pub struct StateChangeActionComponentStateFactory;

impl StateChangeFactory<StateChangeActionType, UiEventArgs, Style, Style, NextUiState>
for StateChangeActionComponentStateFactory {
    fn current_state(current: &EventDescriptor<StateChangeActionType, UiEventArgs, Style>) -> Vec<NextStateChange<NextUiState, Style>> {
        if let UiEventArgs::Event(UiClickStateChange::ChangeSize { update_display}) = &current.event_args {
            return update_display.iter()
                .map(|(entity, size)| {
                    NextStateChange {
                        entity: entity.clone(),
                        next_state: NextUiState::ReplaceSize(size.clone()),
                        phantom: Default::default(),
                    }
                })
                .collect();
        } else if let UiEventArgs::Event(UiClickStateChange::ChangeDisplay { update_display}) = &current.event_args {
            return update_display.iter()
                .map(|(entity, size)| {
                    NextStateChange {
                        entity: entity.clone(),
                        next_state: NextUiState::ReplaceDisplay(size.clone()),
                        phantom: Default::default(),
                    }
                })
                .collect();
        }
        vec![]
    }
}

pub enum NextUiState {
    ReplaceSize(Update<Size>),
    ReplaceDisplay(Update<Display>),
}

impl UpdateStateInPlace<Style> for NextUiState {
    fn update_state(&self, commands: &mut Commands,  value: &mut Style) {
        if let NextUiState::ReplaceSize(update) = &self {
            update.update_state(commands, &mut value.size);
        } else if let NextUiState::ReplaceDisplay(display) = &self {
            display.update_state(commands, &mut value.display);
        }
    }
}

/// The event writer will take the component and it will create the event descriptor, and then
/// pass it on to the event reader, which will read the event. There will be a generic event reader
/// function, and the generic event reader function will be generic over the UiComponentStateFactory
/// and the EventData type. The UiEvents will then be read in that generic function and the state
/// will be updated by the UiComponentStateFactory.
#[derive(Component, Debug, Clone)]
pub enum UiComponent {
    Dropdown(Dropdown, Vec<StateChangeActionType>),
    DropdownOption(DropdownOption, Vec<StateChangeActionType>),
    CollapsableMenuComponent(CollapsableMenu, Vec<StateChangeActionType>),
    Node(Vec<StateChangeActionType>),
}

#[derive(Debug)]
pub enum UiEventArgs {
    Event(UiClickStateChange)
}

impl EventArgs for UiEventArgs {}

pub struct ChangeComponentColorUpdate {
    new_color: Color
}

impl UpdateStateInPlace<BackgroundColor> for ChangeComponentColorUpdate {
    fn update_state(&self, commands: &mut Commands, value: &mut BackgroundColor) {
        value.0 = self.new_color;
    }
}

#[derive(Clone, Debug)]
pub enum ColorChange {
    ChangeColor(Color),
    SwapColor {
        color_1: Color,
        color_2: Color,
    },
}

impl ColorChange {
    fn change_color(&self, mut display: &mut BackgroundColor) {
        match &self {
            ColorChange::ChangeColor(color) => {
                display.0 = color.clone();
            }
            ColorChange::SwapColor { color_1, color_2 } => {
                if &display.0 == color_1 {
                    display.0 = color_2.clone();
                } else {
                    display.0 = color_1.clone();
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct UiComponentFilters {
    pub(crate) exclude: Option<Vec<f32>>,
}


impl UiComponent {
    pub fn get_state_change_types(&self) -> &Vec<StateChangeActionType> {
        match self {
            UiComponent::Dropdown(_, events) => {
                events
            }
            UiComponent::DropdownOption(_, events) => {
                events
            }
            UiComponent::CollapsableMenuComponent(_, events) => {
                events
            }
            UiComponent::Node(events) => {
                events
            }
        }
    }
}

impl EventData for StateChangeActionType {}

#[derive(Clone, Debug)]
pub struct StateChangeActionType {
    pub(crate) hover: HoverStateChange,
    pub(crate) clicked: StateChange,
}
