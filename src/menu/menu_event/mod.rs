use std::fmt::{Debug, Formatter, Pointer, Write};
use std::hash::Hash;
use std::marker::PhantomData;
use bevy::prelude::{BackgroundColor, Button, Changed, Children, Color, Commands, Component, Condition, Display, Entity, EventReader, EventWriter, Interaction, ParamSet, Parent, Query, Res, ResMut, Resource, Style, With, Without, World};
use bevy::app::{App, Plugin};
use bevy::ecs::component::ComponentId;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::log::info;
use bevy::ui::{Size, ui_focus_system, Val};
use bevy::utils::HashMap;
use bevy_mod_picking::Hover;
use crate::menu::{CollapsableMenu, ConfigurationOption, Dropdown, DropdownOption};
use change_style::ChangeStyleTypes;
use crate::menu::menu_event::HoverStateChange::ColoredHover;
use crate::visualization;
use crate::visualization::{create_dropdown, UiIdentifiableComponent};
use bevy::ecs::event::Event;
use crate::menu::menu_event::interaction_ui_event_reader::{EventReaderImpl, EventReaderT};
use crate::menu::menu_event::interaction_ui_event_writer::StateChangeActionTypeStateRetriever;

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
            .insert_resource(StateChangeActionTypeStateRetriever::default())
            .add_startup_system(create_dropdown)
            .add_system(write_events::<
                StateChangeActionTypeStateRetriever,
                StateChangeActionTypeStateRetriever,
                UiEventArgs,
                StateChangeActionType,
                Style,
                (Entity, &UiComponent, &Style, &UiIdentifiableComponent),
                (Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style),
                (Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style),
                (With<UiComponent>, With<Style>),
                (With<UiComponent>, With<Parent>, With<Style>),
                (With<UiComponent>, With<Children>, With<Style>),
                ()
            >)
            .add_system(<EventReaderImpl as EventReaderT<
                StateChangeActionType, UiEventArgs, Style,
                StateChangeActionComponentStateFactory,
                NextUiState,
                (With<UiComponent>)
            >>::read_event)
            .add_event::<UiEventArgs>()
            .add_event::<EventDescriptor<StateChangeActionType, UiEventArgs, Style>>();
    }
}

impl EventArgs for UiEventArgs {}

impl EventData for StateChangeActionType {}

pub trait EventArgs: Send + Sync {}

pub struct EventDescriptor<T: EventData, A: EventArgs, C: Component + Send + Sync + 'static> {
    /// The component for which the state will be updated
    component: PhantomData<C>,
    /// Contains all data needed to update the state
    description: T,
    event_args: A
}

pub trait StateChangeFactory<T, A, C, U: UpdateStateInPlace<C> = ()>: Sized + Resource
    where
        T: EventData,
        A: EventArgs,
        C: Component
{
    fn current_state(current: &EventDescriptor<T, A, C>) -> Vec<NextStateChange<U, C>>;
}

/// If the UpdateStateInPlace contains a struct that converts from certain components to other
/// components
pub trait UpdateStateInPlace<T> {
    fn update_state(&self, value: &mut T);
}

#[derive(Resource, Default, Clone)]
pub struct StateChangeActionComponentStateFactory;

impl StateChangeFactory<StateChangeActionType, UiEventArgs, Style, NextUiState>
for StateChangeActionComponentStateFactory {
    fn current_state(current: &EventDescriptor<StateChangeActionType, UiEventArgs, Style>) -> Vec<NextStateChange<NextUiState, Style>> {
        if let UiEventArgs::Event(ClickStateChangeState::ChangeSize { update_display}) = &current.event_args {
            return update_display.iter()
                .map(|(entity, size)| {
                    NextStateChange {
                        entity: entity.clone(),
                        next_state: NextUiState::ReplaceSize(size.clone()),
                        phantom: Default::default(),
                    }
                })
                .collect();
        } else if let UiEventArgs::Event(ClickStateChangeState::ChangeDisplay { update_display}) = &current.event_args {
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

pub struct NextStateChange<T: UpdateStateInPlace<U>, U: Component + Send + Sync + 'static> {
    entity: Entity,
    next_state: T,
    phantom: PhantomData<U>
}

impl <T: UpdateStateInPlace<U>, U: Component + Send + Sync + 'static> NextStateChange<T, U> {
    pub(crate) fn update_state(&self, value: &mut U) {
        self.next_state.update_state(value);
    }
}

pub enum NextUiState {
    ReplaceSize(Update<Size>),
    ReplaceDisplay(Update<Display>),
}

impl UpdateStateInPlace<Style> for NextUiState {
    fn update_state(&self, value: &mut Style) {
        if let NextUiState::ReplaceSize(update) = &self {
            update.update_state(&mut value.size);
        } else if let NextUiState::ReplaceDisplay(display) = &self {
            display.update_state(&mut value.display);
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
pub enum StateChange {
    ChangeComponentColor(Color, ChangePropagation),
    ChangeComponentStyle(ChangeStyleTypes, ChangePropagation),
    None,
}

pub struct ChangeComponentColorUpdate {
    new_color: Color
}

impl UpdateStateInPlace<BackgroundColor> for ChangeComponentColorUpdate {
    fn update_state(&self, value: &mut BackgroundColor) {
        value.0 = self.new_color;
    }
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


impl StateChange {
    /// Here we have all of the nodes that
    pub fn get_ui_event(&self, args: HashMap<Entity, StyleNode>) -> Option<UiEventArgs> {
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
        update_display: HashMap<Entity, Update<BackgroundColor>>,
    },
    ChangeDisplay {
        update_display: HashMap<Entity, Update<Display>>,
    },
    ChangeSize {
        update_display: HashMap<Entity, Update<Size>>,
    },
    None,
}

#[derive(Clone, Debug)]
pub struct Update<T>
    where T: Clone + Debug + Send + Sync + Default
{
    pub(crate) update_to: Option<T>,
}

impl <T> UpdateStateInPlace<T> for Update<T>
    where T: Clone + Debug + Send + Sync + Default
{
    fn update_state(&self, value: &mut T) {
        *value = self.update_to.as_ref().unwrap().clone();
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

#[derive(Clone, Debug)]
pub struct StateChangeActionType {
    pub(crate) hover: HoverStateChange,
    pub(crate) clicked: StateChange,
}

/// Each type of event has it's own systems for both writing the events and reading the events.
pub trait EventData: Send + Sync {}


/// Fetch the information about the event, such as the child and parent values, to be included
/// in the event.
pub trait RetrieveState<
    A,
    D,
    C,
    SelfQuery,
    ParentQuery,
    ChildQuery,
    SelfFilterQuery: ReadOnlyWorldQuery = (),
    ParentFilterQuery: ReadOnlyWorldQuery = (),
    ChildFilterQuery: ReadOnlyWorldQuery = (),
>: Resource
    where
        A: EventArgs,
        D: EventData,
        C: Component + Send + Sync + 'static,
        SelfQuery: WorldQuery,
        ParentQuery: WorldQuery,
        ChildQuery: WorldQuery
{
    fn retrieve_state(
        entity: Entity,
        self_query: &Query<SelfQuery, SelfFilterQuery>,
        with_parent_query: &Query<ParentQuery, ParentFilterQuery>,
        with_child_query: &Query<ChildQuery, ChildFilterQuery>
    ) ->  Option<EventDescriptor<D, A, C>>;
}

pub fn write_events<
    State,
    ClickState,
    EArgs: EventArgs + 'static,
    EvD: EventData + 'static,
    C: Component + Send + Sync + 'static,
    SelfQuery: WorldQuery,
    ParentQuery: WorldQuery,
    ChildQuery: WorldQuery,
    SelfFilterQuery: ReadOnlyWorldQuery,
    ParentFilterQuery: ReadOnlyWorldQuery,
    ChildFilterQuery: ReadOnlyWorldQuery,
    InteractionFilterQuery: ReadOnlyWorldQuery,
>(
    mut commands: Commands,
    retrieve: ResMut<ClickState>,
    mut event_write: EventWriter<EventDescriptor<EvD, EArgs, C>>,
    self_query: Query<SelfQuery, SelfFilterQuery>,
    with_parent_query: Query<ParentQuery, ParentFilterQuery>,
    with_child_query: Query<ChildQuery, ChildFilterQuery>,
    interaction_query: Query<(Entity, &Interaction), InteractionFilterQuery>,
)
    where
        ClickState: RetrieveState<
            EArgs, EvD, C, SelfQuery, ParentQuery, ChildQuery, SelfFilterQuery,
            ParentFilterQuery, ChildFilterQuery
        >,
        State: RetrieveState<
            EArgs,
            EvD,
            C,
            SelfQuery,
            ParentQuery,
            ChildQuery,
            SelfFilterQuery,
            ParentFilterQuery,
            ChildFilterQuery
        >
{

    let _ = interaction_query
        .iter()
        .for_each(|(entity, interaction)| {
            if let Interaction::Clicked = interaction {
                ClickState::retrieve_state(
                        entity, &self_query,
                        &with_parent_query, &with_child_query,
                    )
                    .map(|event| event_write.send(event));
            }
        });
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

