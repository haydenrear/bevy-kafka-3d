use bevy::prelude::*;
use bevy::app::{App, Plugin};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use crate::cursor_adapter::{event_merge_propagate, propagate_drag_events, propagate_scroll_events};
use crate::event::event_actions::{EventsSystem, InsertComponentInteractionEventReader, InteractionEventReader};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{Relationship};
use crate::event::event_state::{ClickContext, ComponentChangeEventData, Context, HoverStateChange, InsertComponentChangeFactory, NextComponentInsert, NextStateChange, StateChangeFactory, StateUpdate, StyleStateChangeEventData, Update, UpdateStateInPlace};
use crate::event::state_transition::get_state_transitions::GetStateTransitions;
use crate::event::state_transition::state_transitions_system::insert_state_transitions;
use crate::interactions::InteractionEvent;
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::interaction_ui_event_reader::{ComponentChangeEventReader, UiEventReader};
use crate::menu::ui_menu_event::interaction_ui_event_writer::{ClickSelectOptions, DragEvents, ScrollEvents, StateChangeActionTypeStateRetriever, VisibilitySystems};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::next_action::{Matches, NextUiState, UiComponentState};
use crate::menu::ui_menu_event::types::{ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, DraggableUiComponentIxnFilter, ScrollableIxnFilterQuery, ScrollableStateChangeRetriever, StyleStateChange, UiComponentEventDescriptor, UiComponentStyleIxnFilter, VisibilityComponentChangeEventReader};
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::{GlobalState, StateChangeMachine, UiClickStateChange};
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;
use crate::ui_components::ui_menu_component::{create_menu, populate_options_builder, UiIdentifiableComponent};

pub struct UiEventPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CreateMenu {
    #[default]
    AddResources,
    PopulateOptionsBuilder,
    InsertStateTransitions,
}

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<CreateMenu>()
            .add_startup_system(create_menu)
            .add_system(populate_options_builder
                .in_schedule(OnEnter(CreateMenu::PopulateOptionsBuilder))
            )
            // .add_system(insert_state_transitions::<PropagateDisplay, DrawCollapsableMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>
            //     .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            // )
            // .add_system(insert_state_transitions::<PropagateSelect, DropdownMenuOptionResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>
            //     .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            // )
            // .add_system(insert_state_transitions::<PropagateDisplay, DrawDropdownMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>
            //     .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            // )
            // .add_system(insert_state_transitions::<PropagateDisplay, DrawCollapsableMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>
            //     .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            // )
            // .add_system(insert_state_transitions::<PropagateDisplay, BuildBaseMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>
            //     .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            // )
            // .add_system(insert_state_transitions::<PropagateVisible, GraphMenuResultBuilder, GraphMenuResultBuilder, ComponentChangeEventData, MetricsConfigurationOption<Menu>, UiComponentState, Visibility, UiComponentState>
            //     .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            // )
            .add_system(insert_state_transitions::<PropagateVisible, NetworkMenuResultBuilder, NetworkMenuResultBuilder, ComponentChangeEventData, MetricsConfigurationOption<Menu>, UiComponentState, Visibility, UiComponentState>
                .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            )
            .insert_resource(BuildMenuResult::default())
            .insert_resource(ClickEvents::default())
            .insert_resource(DraggableStateChangeRetriever::default())
            .insert_resource(ScrollableStateChangeRetriever::default())
            .insert_resource(UiContext::default())
            .insert_resource(ClickSelectOptions::default())
            .insert_resource(ClickSelectionEventRetriever::default())
            .insert_resource(VisibilitySystems::<MetricsConfigurationOption<Menu>>::default())
            .add_system(VisibilitySystems::<MetricsConfigurationOption<Menu>>::click_write_events)
            .add_system(ClickEvents::click_write_events)
            .add_system(DragEvents::click_write_events)
            .add_system(ScrollEvents::click_write_events)
            .add_system(ClickSelectOptions::click_write_events)
            .add_system(UiEventReader::read_events)
            .add_system(VisibilityComponentChangeEventReader::read_events)
            .add_system(ui_state_change::hover_event)
            .add_system(event_merge_propagate::<DraggableUiComponentIxnFilter>)
            .add_system(event_merge_propagate::<ScrollableIxnFilterQuery>)
            .add_system(event_merge_propagate::<UiComponentStyleIxnFilter>)
            .add_system(propagate_scroll_events)
            .add_system(propagate_drag_events)
            .add_event::<InteractionEvent<DraggableUiComponentIxnFilter>>()
            .add_event::<InteractionEvent<ScrollableIxnFilterQuery>>()
            .add_event::<InteractionEvent<UiComponentStyleIxnFilter>>()
            .add_event::<UiEventArgs>()
            .add_event::<UiComponentEventDescriptor>()
            .add_event::<EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>>()
        ;
    }
}

#[derive(Resource, Default, Clone, Debug)]
pub struct StateChangeActionComponentStateFactory;

impl StateChangeFactory<StyleStateChangeEventData, UiEventArgs, Style, Style, UiContext, NextUiState>
for StateChangeActionComponentStateFactory {
    fn current_state(current: &EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>, context: &mut ResMut<UiContext>) -> Vec<NextStateChange<NextUiState, Style, UiContext>> {
        if let UiEventArgs::Event(UiClickStateChange::ChangeSize { entity, update_display}) = &current.event_args {
            vec![NextStateChange {
                entity: *entity,
                next_state: NextUiState::ReplaceSize(update_display.clone()),
                phantom: Default::default(),
                phantom_ctx: Default::default(),
            }]
        } else if let UiEventArgs::Event(UiClickStateChange::ChangeDisplay { entity, update_display}) = &current.event_args {
            vec![NextStateChange {
                entity: *entity,
                next_state: NextUiState::ReplaceDisplay(update_display.clone()),
                phantom: Default::default(),
                phantom_ctx: Default::default(),
            }]
        } else if let UiEventArgs::Event(UiClickStateChange::Slider { update_scroll, entity }) = &current.event_args {
            vec![NextStateChange {
                entity: *entity,
                next_state: NextUiState::UpdatePosition(update_scroll.clone()),
                phantom: Default::default(),
                phantom_ctx: Default::default(),
            }]
        } else {
            vec![]
        }
    }
}


impl InsertComponentChangeFactory<ComponentChangeEventData, UiEventArgs, Visibility, Visibility, UiContext>
for StateChangeActionComponentStateFactory
{
    fn current_state(
        current: &EventDescriptor<ComponentChangeEventData, UiEventArgs, Visibility>,
        ctx: &mut ResMut<UiContext>
    ) -> Vec<NextComponentInsert<Visibility, Visibility, UiContext>> {
        if let UiEventArgs::Event(ui) = &current.event_args {
            return match ui {
                UiClickStateChange::ChangeVisible { entity, update_component } => {
                    vec![
                        NextComponentInsert {
                            entity: *entity,
                            next_state: *update_component,
                            phantom: Default::default(),
                            phantom_ctx: Default::default(),
                        }
                    ]
                }
                _ => {
                    vec![]
                }
            };
        }
        vec![]
    }
}


/// Any arbitrary state transition for any arbitrary component and context.
#[derive(Debug)]
pub struct EntityComponentStateTransition<
    StateMachineT,
    StateComponentT,
    FilterMatchesT,
    UpdateComponentT,
    UpdateMatchesT,
    Ctx,
    EventArgsT,
    TransitionGroupComponentT
>
    where
        Ctx: Context,
        EventArgsT: EventArgs,
        UpdateMatchesT: Matches<UpdateComponentT>,
        FilterMatchesT: Matches<StateComponentT>,
        StateMachineT: EventData,
        TransitionGroupComponentT: TransitionGroup
{
    pub(crate) entity_to_change: EntitiesStateTypes<UpdateComponentT, StateMachineT, Ctx, EventArgsT>,
    // filter for whether or not to send events at all.
    pub(crate) filter_state: FilterMatchesT,
    // filter for that entity to be changed, whether it is the parent, child, etc..
    pub(crate) current_state_filter: UpdateMatchesT,
    pub(crate) filter_component: PhantomData<TransitionGroupComponentT>,
    pub(crate) state_component: PhantomData<StateComponentT>,
}


/// In some cases, some events should and should not be propagated to all children. So this determines
/// who to include when building out the tree of events.
pub trait TransitionGroup: Component {
}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateDisplay;
impl TransitionGroup for PropagateDisplay {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateVisible;
impl TransitionGroup for PropagateVisible {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateScrollable;
impl TransitionGroup for PropagateScrollable {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateDraggable;
impl TransitionGroup for PropagateDraggable {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateSelect;
impl TransitionGroup for PropagateSelect {}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateRaycast;
impl TransitionGroup for PropagateRaycast {}

#[derive(Debug)]
pub struct EntitiesStateTypes<T, StateMachine, Ctx, Args>
where
    StateMachine: EventData,
    Ctx: Context,
    Args: EventArgs
{
    /// In the update function, the EntityComponentStateTransitionComponent will iterate through
    /// each of the states and get the related components to calculate the value to be passed to
    /// the StateUpdate function.
    pub(crate) states: Vec<(Entity, Relationship, StateChangeActionType<StateMachine, T, Ctx, Args>)>
}

/// Paricular types of state transitions, that are associated with a particular state machine, component, and transition group.
#[derive(Component, Debug)]
pub struct UiEntityComponentStateTransitions<
    StateMachineT,
    StateComponentT,
    FilterMatchesT,
    UpdateComponentT,
    UpdateMatchesT,
    Ctx,
    EventArgsT,
    TransitionGroupComponentT
>
where
    Ctx: Context,
    EventArgsT: EventArgs,
    FilterMatchesT: Matches<StateComponentT>,
    UpdateMatchesT: Matches<UpdateComponentT>,
    StateMachineT: EventData,
    TransitionGroupComponentT: TransitionGroup
{
    pub(crate) transitions: Vec<EntityComponentStateTransition<
        StateMachineT,
        StateComponentT,
        FilterMatchesT,
        UpdateComponentT,
        UpdateMatchesT,
        Ctx,
        EventArgsT,
        TransitionGroupComponentT
    >>,
    pub(crate) state_component: PhantomData<StateComponentT>,
}

#[derive(Debug)]
pub enum UiEventArgs {
    Event(UiClickStateChange)
}

impl EventArgs for UiEventArgs {}

#[derive(Debug)]
pub struct ChangeComponentColorUpdate {
    new_color: Color
}

impl UpdateStateInPlace<BackgroundColor, UiContext> for ChangeComponentColorUpdate {
    fn update_state(&self, commands: &mut Commands, value: &mut BackgroundColor, ctx: &mut ResMut<UiContext>) {
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

impl <T, Ctx, C, Args> EventData for StateChangeActionType<T, C, Ctx, Args>
    where
        Ctx: Context,
        Args: EventArgs,
        T: StateChangeMachine<C, Ctx, Args> + Send + Sync,
        C: Send + Sync
{}

#[derive(Clone, Debug)]
pub enum StateChangeActionType<StateMachineT, ComponentT, Ctx, EventArgsT>
where
    Ctx: Context,
    EventArgsT: EventArgs,
    StateMachineT: EventData
{
    Hover { value: StateMachineT, p: PhantomData<ComponentT>, p1: PhantomData<Ctx>, p2: PhantomData<EventArgsT> },
    Clicked { value: StateMachineT, p: PhantomData<ComponentT>, p1: PhantomData<Ctx>, p2: PhantomData<EventArgsT> },
    Dragged { value: StateMachineT, p: PhantomData<ComponentT>, p1: PhantomData<Ctx>, p2: PhantomData<EventArgsT> },
    Scrolled { value: StateMachineT, p: PhantomData<ComponentT>, p1: PhantomData<Ctx>, p2: PhantomData<EventArgsT> },
}

macro_rules! get_value {
    ($($name:ident),*)  => {
        impl <T, C, Ctx, Args> StateChangeActionType<T, C, Ctx, Args>
        where
            Ctx: Context,
            Args: EventArgs,
            T: StateChangeMachine<C, Ctx, Args> + Send + Sync
        {

            pub(crate) fn get_state_machine<'a>(&'a self) -> &'a T {
                match &self {
                    $(
                        StateChangeActionType::$name{ value, .. } => {
                            &value
                        }
                    )*
                }
            }
        }
    }
}

get_value!(
    Hover, Clicked, Dragged, Scrolled
);
