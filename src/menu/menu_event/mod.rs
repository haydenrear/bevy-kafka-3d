use std::fmt::{Debug, Formatter, Pointer, Write};
use std::hash::Hash;
use std::marker::PhantomData;
use bevy::prelude::{BackgroundColor, Button, Changed, Children, Color, Commands, Component, Condition, Display, Entity, EventReader, EventWriter, Interaction, ParamSet, Parent, Query, Style, With, Without, World};
use bevy::app::{App, Plugin};
use bevy::ecs::component::ComponentId;
use bevy::ecs::query::QueryEntityError;
use bevy::log::info;
use bevy::ui::{Size, ui_focus_system, Val};
use bevy::utils::HashMap;
use bevy_mod_picking::Hover;
use crate::menu::{CollapsableMenu, ConfigurationOption, Dropdown, DropdownOption};
use change_style::ChangeStyleTypes;
use crate::menu::menu_event::HoverStateChange::ColoredHover;
use crate::visualization;
use crate::visualization::{create_dropdown, UiIdentifiableComponent};

pub(crate) mod interaction_ui_event_writer;
pub(crate) mod interaction_ui_event_reader;
pub(crate) mod interaction_config_event_writer;
pub(crate) mod interaction_config_event_reader;
pub(crate) mod change_style;
pub(crate) mod change_options;
pub(crate) mod network_component_event_reader;

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(interaction_ui_event_writer::write_ui_events)
            .add_system(interaction_ui_event_reader::read_menu_ui_event)
            .add_system(hover_event)
            .add_startup_system(create_dropdown)
            .add_event::<UiEvent>();
    }
}

#[derive(Debug)]
pub enum UiEvent {
    Event(ClickStateChangeState)
}

#[derive(Component, Default)]
pub enum NodeConfigurationOption {
    #[default]
    Option
}


#[derive(Debug, Clone)]
pub enum ConfigurationOptionEvent<T: Component + Send + Sync + 'static> {
    Event(ConfigurationOption<T>)
}

#[derive(Clone, Debug)]
pub enum HoverStateChange {
    ColoredHover {
        color: Color
    },
    None,
}

#[derive(Clone, Debug)]
pub enum StateChange<T: Component + Send + Sync + 'static> {
    ChangeComponentColor(Color, ChangePropagation),
    ChangeComponentStyle(ChangeStyleTypes, ChangePropagation),
    ChangeConfigurationOption(ConfigurationOptionEvent<T>),
    None,
}

/// Determines where to get the starting state from, which determines the next state. For instance,
/// if a child is swapping from visible to invisible, and the parent is swapping, then in order so
/// that they won't swap out of sync, you use starting state of one to determine next state of both.
#[derive(Clone, Debug)]
pub enum StartingState {
    Child,
    Parent,
    SelfState,
    Other(f32)
}

#[derive(Clone, Debug)]
pub enum ChangePropagation {
    // Include self as parent and any children of parent
    ParentToChild(StartingState),
    // Include self as child and parent of self
    ChildToParent(StartingState),
    // Include self only
    SelfChange(StartingState),
    // Include children only
    Children(StartingState),
    // Include parent only
    Parent(StartingState),
    // Propagate event to specific Id's
    CustomPropagation {
        to: Vec<f32>,
        // starting state
        from: StartingState
    }
}

impl ChangePropagation {
    fn get_starting_state(&self) -> StartingState {
        match self {
            ChangePropagation::ParentToChild(starting) => {
                starting.clone()
            }
            ChangePropagation::ChildToParent(starting) => {
                starting.clone()
            }
            ChangePropagation::SelfChange(starting) => {
                starting.clone()
            }
            ChangePropagation::Children(starting) => {
                starting.clone()
            }
            ChangePropagation::Parent(starting) => {
                starting.clone()
            }
            ChangePropagation::CustomPropagation { to , from} => {
                from.clone()
            }
        }
    }
}

impl ChangePropagation {
    pub(crate) fn includes_parent(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(_) => {
                true
            }
            ChangePropagation::ChildToParent(_) => {
                true
            }
            ChangePropagation::SelfChange(_) => {
                false
            }
            ChangePropagation::Children(_) => {
                false
            }
            ChangePropagation::Parent(_) => {
                true
            }
            ChangePropagation::CustomPropagation { .. } => {
                false
            }
        }
    }

    pub(crate) fn includes_self(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(_) => {
                true
            }
            ChangePropagation::ChildToParent(_) => {
                true
            }
            ChangePropagation::SelfChange(_) => {
                true
            }
            ChangePropagation::Children(_) => {
                false
            }
            ChangePropagation::Parent(_) => {
                false
            }
            ChangePropagation::CustomPropagation { .. } => {
                false
            }
        }
    }

    pub(crate) fn includes_children(&self) -> bool {
        match self {
            ChangePropagation::ParentToChild(_) => {
                true
            }
            ChangePropagation::ChildToParent(_) => {
                false
            }
            ChangePropagation::SelfChange(_) => {
                false
            }
            ChangePropagation::Children(_) => {
                true
            }
            ChangePropagation::Parent(_) => {
                false
            }
            ChangePropagation::CustomPropagation { .. } => {
                false
            }
        }
    }
}


impl StateChange<UiIdentifiableComponent> {
    /// Here we have all of the nodes that
    pub fn get_ui_event(&self, args: HashMap<Entity, StyleNode>) -> Option<UiEvent> {
        if let StateChange::ChangeComponentStyle(change_style, propagation) = self {
            // here we translate to the UiComponentFilters, from the change_style, and then
            // pass the UiComponentFilter to a method that executes
            return change_style.get_current_state(&args, propagation)
                .map(|style| {
                    let filtered = change_style.filter_entities(args);
                    if filtered.is_empty() {
                        info!("Filtered entities were none.");
                    } else {
                        info!("Doing state change with {:?}", filtered);
                    }
                    change_style.do_change(&style, filtered)
                })
                .flatten()
                .or_else(|| {
                    info!("Could not get current state.");
                    None
                });
        }
        None
    }

    pub fn propagation(&self) -> Option<&ChangePropagation> {
        match self {
            StateChange::ChangeComponentColor(_, change_type) => {
                Some(change_type)
            }
            StateChange::ChangeComponentStyle(_, change_type) => {
                Some(change_type)
            }
            StateChange::None => {
                None
            }
            StateChange::ChangeConfigurationOption(_) => {
                None
            }
        }
    }
}

/// May consider adding a flag to signify that the state of that node should be the one to determine
/// the state of the others. For instance, if switching from visible to invisible, which node determines?
/// So you can use a flag here.
#[derive(Clone)]
pub enum StyleNode {
    Child(Style, f32),
    SelfNode(Style, f32),
    Parent(Style, f32),
    Other(Style, f32)
}

impl Debug for StyleNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node enum: ");
        match self {
            StyleNode::Child(_, _) => {
                f.write_str(" Child ");
            }
            StyleNode::SelfNode(_, _) => {
                f.write_str(" Self ");
            }
            StyleNode::Parent(_, _) => {
                f.write_str(" Parent ");
            }
            StyleNode::Other(_, _) => {
                f.write_str(" Other ");
            }
        }
        f.write_str(self.id().to_string().as_str())
    }
}

impl StyleNode {
    fn id(&self) -> f32 {
        match self {
            StyleNode::Child(_, id) => {
                *id
            }
            StyleNode::SelfNode(_, id) => {
                *id
            }
            StyleNode::Parent(_, id) => {
                *id
            }
            StyleNode::Other(_, id) => {
                *id
            }
        }
    }

    pub(crate) fn get_style(&self) -> Style {
        match self {
            StyleNode::Child(style, _) => {
                style.clone()
            }
            StyleNode::SelfNode(style, id) => {
                style.clone()
            }
            StyleNode::Parent(style, id) => {
                style.clone()
            }
            StyleNode::Other(style, _) => {
                style.clone()
            }
        }
    }
}

/// Contains the state data needed in order to generate the UIEvents from the state change required.
#[derive(Clone, Debug)]
pub enum ClickStateChangeState {
    ChangeColor {
        current_display: HashMap<Entity, Color>,
        update_display: HashMap<Entity, Color>,
    },
    ChangeDisplay {
        current_display: HashMap<Entity, Display>,
        update_display: HashMap<Entity, Display>,
    },
    ChangeSize {
        current_display: HashMap<Entity, Size>,
        update_display: HashMap<Entity, Size>,
    },

    None,
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

#[derive(Clone, Debug)]
pub struct StateChangeActionType<T: Component + Send + Sync + 'static> {
    pub(crate) hover: HoverStateChange,
    pub(crate) clicked: StateChange<T>,
}

pub trait EventData: Send + Sync {}

pub struct EventDescriptor<T: EventData, C: Component> {
    component: C,
    description: T
}

pub trait UiComponentStateFactory<U: UpdateStateInPlace<Self>, T: EventData>: Sized + Component {
    fn current_state(current: EventDescriptor<T, Self>) -> NextState<Self, U>;
}

pub trait UpdateStateInPlace<T: Component> {
    fn update_state(value: &mut T);
}

pub enum NextState<T: Component, U: UpdateStateInPlace<T>> {
    Replace(T, PhantomData<U>),
    Update(U)
}




/// If each UIComponent contained a state and implemented a next_action trait, which passed
/// in some state of enum, and then generated the next state, then a high level of modularity
/// could be achieved. Actions would be passed in, which would contain the state of the other
/// components and entities required to perform the state change, and then the new state for that
/// component would be created by that component, and the state of that component would be updated,
/// or the old component would be replaced by the new component.
///
/// k
#[derive(Component, Debug, Clone)]
pub enum UiComponent {
    Dropdown(Dropdown, Vec<StateChangeActionType<UiIdentifiableComponent>>),
    DropdownOption(DropdownOption, Vec<StateChangeActionType<UiIdentifiableComponent>>),
    CollapsableMenuComponent(CollapsableMenu, Vec<StateChangeActionType<UiIdentifiableComponent>>),
    Node(Vec<StateChangeActionType<UiIdentifiableComponent>>),
}

impl UiComponent {
    pub fn get_state_change_types(&self) -> &Vec<StateChangeActionType<UiIdentifiableComponent>> {
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

fn hover_event(
    mut query: Query<(&mut Style, &mut BackgroundColor, &Interaction), (With<UiComponent>, With<Button>, Changed<Interaction>)>,
) {
    for (_, mut color, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
            }
            Interaction::Hovered => {
                color.0 = Color::YELLOW;
            }
            Interaction::None => {
                color.0 = Color::GREEN;
            }
        }
    }
}

