use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy::hierarchy::{Children, Parent};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use bevy::ecs::schedule::SystemSetConfig;
use bevy::ecs::system::SystemParam;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::time::TimerMode;
use bevy::utils::{HashMap, HashSet};
use crate::cursor_adapter::event_merge_propagate;
use crate::event::event_actions::{ClickWriteEvents, InteractionEventReader};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::{ClickContext, Context, HoverStateChange, NextStateChange, StateChangeFactory, StateUpdate, StyleStateChangeEventData, Update, UpdateStateInPlace};
use crate::event::event_state::StyleStateChangeEventData::ChangeComponentStyle;
use crate::event::state_transition::state_transitions_system::insert_state_transitions;
use crate::interactions::InteractionEvent;
use crate::menu::{CollapsableMenuComponent, ConfigurationOptionEnum, DraggableComponent, Dropdown, DropdownOption, MenuItemMetadata, Radial, RadialButton, RadialButtonSelection, ScrollableComponent, Slider, SliderKnob, ui_menu_event, UiComponent};
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::interaction_ui_event_reader::UiEventReader;
use crate::menu::ui_menu_event::interaction_ui_event_writer::{DragEvents, ScrollEvents, StateChangeActionTypeStateRetriever};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::next_action::DisplayState::DisplayNone;
use crate::menu::ui_menu_event::next_action::{Matches, NextUiState, UiComponentState};
use crate::menu::ui_menu_event::types::{ClickEvents, DraggableStateChangeRetriever, DraggableUiComponentIxnFilter, ScrollableStateChangeRetriever, ScrollableUiComponentIxnFilter, StyleStateChange, UiComponentEventDescriptor, UiComponentStyleIxnFilter};
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::{GlobalState, StateChangeMachine, UiClickStateChange};
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::ui_menu_component::{create_menu, UiIdentifiableComponent};

pub struct UiEventPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CreateMenu {
    #[default]
    AddResources,
    InsertStateTransitions,
}

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<CreateMenu>()
            .add_startup_system(create_menu)
            .add_system(insert_state_transitions::<PropagateDisplay>
                .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            )
            .add_system(insert_state_transitions::<SelectOptions>
                .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            )
            .insert_resource(BuildMenuResult::default())
            .insert_resource(ClickEvents::default())
            .insert_resource(DraggableStateChangeRetriever::default())
            .insert_resource(ScrollableStateChangeRetriever::default())
            .insert_resource(UiContext::default())
            .add_system(ClickEvents::click_write_events)
            .add_system(DragEvents::click_write_events)
            .add_system(ScrollEvents::click_write_events)
            .add_system(UiEventReader::read_events)
            .add_system(ui_state_change::hover_event)
            .add_system(event_merge_propagate::<DraggableUiComponentIxnFilter>)
            .add_system(event_merge_propagate::<ScrollableUiComponentIxnFilter>)
            .add_system(event_merge_propagate::<UiComponentStyleIxnFilter>)
            .add_event::<InteractionEvent<DraggableUiComponentIxnFilter>>()
            .add_event::<InteractionEvent<ScrollableUiComponentIxnFilter>>()
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

#[derive(Debug)]
pub struct UiComponentStateTransition {
    pub(crate) filter_state: UiComponentState,
    pub(crate) state_change: Vec<StyleStateChange>,
    pub(crate) propagation: ChangePropagation,
    /// Sometimes the event is driven from another component, and then propagated to other components.
    /// In this case, there needs to be a filter for whether or not to change the other component, the
    /// component for which the event is propagated to.
    pub(crate) current_state_filter: UiComponentState
}

#[derive(Debug)]
pub struct EntityComponentStateTransition<StateMachineT, ComponentT, MatchesT, Ctx, EventArgsT, TransitionGroupComponentT>
    where
        Ctx: Context,
        EventArgsT: EventArgs,
        MatchesT: Matches<ComponentT>,
        StateMachineT: StateChangeMachine<ComponentT, Ctx, EventArgsT>,
        TransitionGroupComponentT: TransitionGroup
{
    pub(crate) entity_to_change: EntitiesStateTypes<ComponentT, StateMachineT, Ctx, EventArgsT>,
    pub(crate) filter_state: MatchesT,
    // filter for the component to be changed.
    pub(crate) current_state_filter: MatchesT,
    pub(crate) filter_component: PhantomData<TransitionGroupComponentT>
}

/// Sometimes you want to filter based on a component, and this allows for the EntityStateTransition
/// to only propagate to certain children that contain the component.
pub trait TransitionGroup: Component {
}

#[derive(Component, Default, Clone, Debug)]
pub struct PropagateDisplay;
impl TransitionGroup for PropagateDisplay {}

#[derive(Component, Default, Clone, Debug)]
pub struct SelectOptions;
impl TransitionGroup for SelectOptions {}

#[derive(Debug)]
pub struct EntitiesStateTypes<T, StateMachine, Ctx, Args>
where
    StateMachine: StateChangeMachine<T, Ctx, Args> + Send + Sync + 'static,
    Ctx: Context,
    Args: EventArgs
{
    /// In the update function, the EntityComponentStateTransitionComponent will iterate through
    /// each of the states and get the related components to calculate the value to be passed to
    /// the StateUpdate function.
    pub(crate) states: Vec<(Entity, Relationship, StateChangeActionType<StateMachine, T, Ctx, Args>)>
}

#[derive(Component, Debug)]
pub struct UiEntityComponentStateTransitions<StateMachineT, ComponentT, MatchesT, Ctx, EventArgsT, TransitionGroupComponentT>
where
    Ctx: Context,
    EventArgsT: EventArgs,
    MatchesT: Matches<ComponentT>,
    StateMachineT: StateChangeMachine<ComponentT, Ctx, EventArgsT>,
    TransitionGroupComponentT: TransitionGroup
{
    pub(crate) transitions: Vec<EntityComponentStateTransition<StateMachineT, ComponentT, MatchesT, Ctx, EventArgsT, TransitionGroupComponentT>>,
}

#[derive(Component, Debug)]
pub struct UiComponentStateTransitions {
    pub(crate) transitions: Vec<UiComponentStateTransition>,
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

#[derive(Clone, Debug)]
pub struct UiComponentFilters {
    pub(crate) exclude: Option<Vec<f32>>,
}

impl <T, Ctx, C, Args> EventData for StateChangeActionType<T, C, Ctx, Args>
    where
        Ctx: Context,
        Args: EventArgs,
        T: StateChangeMachine<C, Ctx, Args> + Send + Sync,
        C: Send + Sync
{

}

#[derive(Clone, Debug)]
pub enum StateChangeActionType<T, C, Ctx, Args>
where
    Ctx: Context,
    Args: EventArgs,
    T: StateChangeMachine<C, Ctx, Args> + Send + Sync
{
    Hover { value: T, p: PhantomData<C>, p1: PhantomData<Ctx>, p2: PhantomData<Args> },
    Clicked { value: T, p: PhantomData<C>, p1: PhantomData<Ctx>, p2: PhantomData<Args> },
    Dragged { value: T, p: PhantomData<C>, p1: PhantomData<Ctx>, p2: PhantomData<Args> },
}
