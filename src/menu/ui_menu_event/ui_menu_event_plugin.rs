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
use crate::cursor_adapter::CursorResource;
use crate::event::event_actions::{ClickWriteEvents, InteractionEventReader};
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::{ClickContext, Context, HoverStateChange, NextStateChange, StyleStateChangeEventData, StateChangeFactory, StateUpdate, Update, UpdateStateInPlace};
use crate::event::event_state::StyleStateChangeEventData::ChangeComponentStyle;
use crate::menu::{CollapsableMenu, ConfigurationOptionEnum, DraggableComponent, Dropdown, DropdownOption, MenuItemMetadata, Radial, RadialButton, RadialButtonSelection, ScrollableComponent, Slider, SliderKnob, ui_menu_event, UiBundled, UiComponent};
use crate::menu::config_menu_event::interaction_config_event_writer::ConfigOptionContext;
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::menu::ui_menu_event::interaction_ui_event_reader::UiEventReader;
use crate::menu::ui_menu_event::interaction_ui_event_writer::{DragEvents, GlobalState, ScrollEvents, StateChangeActionTypeStateRetriever};
use crate::menu::ui_menu_event::style_context::UiContext;
use crate::menu::ui_menu_event::next_action::DisplayState::DisplayNone;
use crate::menu::ui_menu_event::next_action::{Matches, NextUiState, UiComponentState};
use crate::menu::ui_menu_event::ui_state_change;
use crate::menu::ui_menu_event::ui_state_change::{StateChangeMachine, UiClickStateChange};
use crate::ui_components::state_transitions::insert_state_transitions;
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::ui_menu_component::{create_menu, UiIdentifiableComponent};

pub struct UiEventPlugin;

pub type UiComponentStyleFilter = (With<UiComponent>, With<Style>);
pub type UiComponentStyleIxnFilter = (With<UiComponent>, With<Button>, Changed<Interaction>);
pub type DraggableUiComponentFilter = (With<UiComponent>, With<Style>, With<DraggableComponent>);
pub type DraggableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<DraggableComponent>);
pub type ScrollableUiComponentFilter = (With<UiComponent>, With<Style>, With<ScrollableComponent>);
pub type ScrollableUiComponentIxnFilter = (With<UiComponent>, With<Button>, With<ScrollableComponent>);
pub type ScrollableIxnFilterQuery = (With<UiComponent>, With<Button>, With<ScrollableComponent>);
pub type PropagationQueryFilter<C> = (With<C>);

pub type PropagationQuery<'a, C> = (Entity, &'a C, &'a UiIdentifiableComponent);
pub type UiComponentStateTransitionsQuery<'a, C, S, M> = (Entity, &'a UiComponent, &'a C, &'a UiIdentifiableComponent, &'a UiEntityComponentStateTransitions<S,C, M>);

pub type StyleUiComponentStateTransitionsQuery<'a> = UiComponentStateTransitionsQuery<'a, Style, StyleStateChangeEventData, UiComponentState>;
pub type StylePropagationQuery<'a> = PropagationQuery<'a, Style>;
pub type StylePropagationQueryFilter = PropagationQueryFilter<Style>;

pub type UiStateChange<C, S> = StateChangeActionType<S, C, UiContext, UiEventArgs>;
pub type StyleStateChange = StateChangeActionType<StyleStateChangeEventData, Style, UiContext, UiEventArgs>;

pub type DraggableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState>;
pub type ScrollableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, Style, UiContext,
    UiEventArgs, StyleStateChangeEventData, UiComponentState>;
pub type UiComponentStateChangeRetriever = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style,
    UiContext, UiEventArgs, StyleStateChangeEventData, UiComponentState
>;

pub type UiComponentEventDescriptor = EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>;

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
            .add_system(insert_state_transitions
                .in_schedule(OnEnter(CreateMenu::InsertStateTransitions))
            )
            .insert_resource(BuildMenuResult::default())
            .insert_resource(UiComponentStateChangeRetriever::default())
            // .insert_resource(DraggableStateChangeRetriever::default())
            // .insert_resource(ScrollableStateChangeRetriever::default())
            .add_system(UiComponentStateChangeRetriever::click_write_events)
            // .add_system(DragEvents::click_write_events)
            // .add_system(ScrollEvents::click_write_events)
            .insert_resource(UiContext::default())
            .add_system(UiEventReader::read_events)
            .add_system(ui_state_change::hover_event)
            .add_event::<UiEventArgs>()
            .add_event::<UiComponentEventDescriptor>()
            .add_event::<EventDescriptor<StyleStateChangeEventData, UiEventArgs, Style>>()
        ;
    }
}

#[derive(Resource, Default, Clone, Debug)]
pub struct StateChangeActionComponentStateFactory;

impl StateChangeFactory<StyleStateChange, UiEventArgs, Style, Style, UiContext, NextUiState>
for StateChangeActionComponentStateFactory {
    fn current_state(current: &EventDescriptor<StyleStateChange, UiEventArgs, Style>, context: &mut ResMut<UiContext>) -> Vec<NextStateChange<NextUiState, Style, UiContext>> {
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

pub trait UiComponentStateFilter<T> {
    fn matches(&self, other: &T) -> bool;
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
pub struct EntityComponentStateTransition<T: StateChangeMachine<C, UiContext, UiEventArgs>, C, M: Matches<C>> {
    pub(crate) entity_to_change: EntitiesStateTypes<C, T>,
    pub(crate) filter_state: M,
    // filter for the component to be changed.
    pub(crate) current_state_filter: M
}

#[derive(Debug)]
pub struct EntitiesStateTypes<T, StateMachine>
where
    StateMachine: StateChangeMachine<T, UiContext, UiEventArgs> + Send + Sync + 'static
{
    /// In the update function, the EntityComponentStateTransitionComponent will iterate through
    /// each of the states and get the related components to calculate the value to be passed to
    /// the StateUpdate function.
    pub(crate) states: Vec<(Entity, Relationship, UiStateChange<T, StateMachine>)>
}

#[derive(Component, Debug)]
pub struct UiEntityComponentStateTransitions<T: StateChangeMachine<C, UiContext, UiEventArgs>, C, M: Matches<C>>  {
    pub(crate) transitions: Vec<EntityComponentStateTransition<T, C, M>>,
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
