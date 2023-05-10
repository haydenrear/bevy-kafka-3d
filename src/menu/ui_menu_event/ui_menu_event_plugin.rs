use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy::hierarchy::{Children, Parent};
use std::fmt::{Debug, Formatter};
use bevy::time::TimerMode;
use crate::event::event_actions::{InteractionEventReader, StateChangeEventReader};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::ChangePropagation;
use crate::event::event_state::{Context, HoverStateChange, NextStateChange, StateChange, StateChangeFactory, Update, UpdateStateInPlace};
use crate::menu::{CollapsableMenu, Dropdown, DropdownOption, ui_menu_event};
use crate::menu::ui_menu_event::interaction_ui_event_writer::StateChangeActionTypeStateRetriever;
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::UiClickStateChange;
use crate::ui_components::ui_menu_component::{create_dropdown, UiIdentifiableComponent};

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(create_dropdown)
            .insert_resource(StateChangeActionTypeStateRetriever::default())
            .insert_resource(StyleContext::default())
            .add_system(crate::event::event_actions::click_write_events::<
                StateChangeActionTypeStateRetriever, UiEventArgs, StateChangeActionType, Style, StyleContext,
                // self query
                (Entity, &UiComponent, &UiComponentStateTransitions, &Style, &UiIdentifiableComponent),
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
                NextUiState, StyleContext, (With<UiComponent>)
            >>::read_events)
            .add_system(ui_state_change::hover_event)
            .add_event::<UiEventArgs>()
            .add_event::<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>();
    }
}

#[derive(Resource, Default, Clone)]
pub struct StateChangeActionComponentStateFactory;

impl StateChangeFactory<StateChangeActionType, UiEventArgs, Style, Style, StyleContext, NextUiState>
for StateChangeActionComponentStateFactory {
    fn current_state(current: &EventDescriptor<StateChangeActionType, UiEventArgs, Style>, context: &mut ResMut<StyleContext>) -> Vec<NextStateChange<NextUiState, Style, StyleContext>> {
        if let UiEventArgs::Event(UiClickStateChange::ChangeSize { update_display}) = &current.event_args {
            return update_display.iter()
                .map(|(entity, size)| {
                    NextStateChange {
                        entity: entity.clone(),
                        next_state: NextUiState::ReplaceSize(size.clone()),
                        phantom: Default::default(),
                        phantom_ctx: Default::default(),
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
                        phantom_ctx: Default::default(),
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

#[derive(Resource, Default, Clone)]
pub struct StyleContext {
}

impl Context for StyleContext {
}

impl UpdateStateInPlace<Style, StyleContext> for NextUiState {
    fn update_state(&self, commands: &mut Commands,  value: &mut Style, style_context: &mut ResMut<StyleContext>) {
        if let NextUiState::ReplaceSize(update) = &self {
            update.update_state(commands, &mut value.size, style_context);
        } else if let NextUiState::ReplaceDisplay(display) = &self {
            display.update_state(commands, &mut value.display, style_context);
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
    Dropdown(Dropdown),
    MenuOption(DropdownOption),
    CollapsableMenuComponent(CollapsableMenu),
    Node,
}

pub trait UiComponentStateFilter<T> {
    fn matches(&self, other: &T) -> bool;
}

#[derive(Debug)]
pub enum UiComponentState {
    StateDisplay(DisplayState),
    StateSize(SizeState)
}

#[derive(Debug)]
pub struct UiComponentStateTransition {
    pub(crate) filter_state: UiComponentState,
    pub(crate) state_change: Vec<StateChangeActionType>,
    pub(crate) propagation: ChangePropagation,
    /// Sometimes the event is driven from another component, and then propagated to other components.
    /// In this case, there needs to be a filter for whether or not to change the other component, the
    /// component for which the event is propagated to.
    pub(crate) current_state_filter: UiComponentState
}

#[derive(Component, Debug)]
pub struct UiComponentStateTransitions {
    pub(crate) transitions: Vec<UiComponentStateTransition>
}

#[derive(Debug, Eq, PartialEq)]
pub enum DisplayState {
    DisplayFlex, DisplayNone,
    DisplayAny,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SizeState {
    Expanded{
        height: u32,
        width: u32,
    }, Minimized {
        height: u32,
        width: u32,
    }
}

impl DisplayState {
    fn get_display(&self) -> Display {
        match self {
            DisplayState::DisplayFlex => {
                Display::Flex
            }
            DisplayState::DisplayNone => {
                Display::None
            }
            DisplayState::DisplayAny => {
                Display::Flex
            }
        }
    }
}

impl UiComponentStateFilter<Display> for DisplayState {
    fn matches(&self, other: &Display) -> bool {
        if let DisplayState::DisplayAny = self {
            return true;
        }
        if self.get_display() == *other  {
            return true;
        } else {
            return false;
        }
    }
}

impl SizeState {

    fn get_width_height(&self) -> (u32, u32) {
        match self  {
            SizeState::Expanded { height, width } => {
                (*height, *width)
            }
            SizeState::Minimized { height, width } => {
                (*height, *width)
            }
        }
    }

}

impl UiComponentStateFilter<Size> for SizeState {
    fn matches(&self, starting_state: &Size) -> bool {
        let (height_state, width_state) = self.get_width_height();
        info!("{} is height and {} is width, and {:?} is starting_state.", height_state, width_state, starting_state);
        if let Val::Percent(height) = starting_state.height {
            if let Val::Percent(width) = starting_state.width {
                info!("{} is match height and {} is match width.", height, width);
                if height as u32 == height_state && width as u32 == width_state {
                    info!("matched");
                    return true;
                }
                return false;
            }
        }
        if let Val::Px(height) = starting_state.height {
            if let Val::Px(width) = starting_state.width {
                info!("{} is match height and {} is match width.", height, width);
                info!("{} is match height and {} is match width.", height, width);
                if height as u32 == height_state && width as u32 == width_state {
                    info!("matched");
                    return true;
                }
                return false;
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum UiEventArgs {
    Event(UiClickStateChange)
}

impl EventArgs for UiEventArgs {}

pub struct ChangeComponentColorUpdate {
    new_color: Color
}

impl UpdateStateInPlace<BackgroundColor, StyleContext> for ChangeComponentColorUpdate {
    fn update_state(&self, commands: &mut Commands, value: &mut BackgroundColor, ctx: &mut ResMut<StyleContext>) {
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

impl EventData for StateChangeActionType {}

#[derive(Clone, Debug)]
pub enum StateChangeActionType {
    Hover(StateChange),
    Clicked(StateChange)
}
