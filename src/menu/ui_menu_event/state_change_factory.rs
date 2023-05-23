use std::fmt::Debug;
use bevy::prelude::{BackgroundColor, Color, Commands, Entity, ResMut, Resource, Style, Visibility};
use std::marker::PhantomData;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::Relationship;
use crate::event::event_state::{ComponentChangeEventData, Context, InsertComponentChangeFactory, NextComponentInsert, NextStateChange, StateChangeFactory, StyleStateChangeEventData, UpdateStateInPlace};
use crate::graph::Graph;
use crate::menu::graphing_menu::graph_menu::{ChangeGraphingMenu, GraphMenuPotential};
use crate::menu::ui_menu_event::next_action::NextUiState;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::menu::ui_menu_event::ui_state_change::{ChangeVisible, StateChangeMachine, UiClickStateChange};

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


impl<T: ChangeVisible + Clone + Debug> InsertComponentChangeFactory<
    ComponentChangeEventData,
    UiEventArgs,
    Visibility,
    T,
    UiContext,
    NextComponentInsert<Visibility, T, UiContext>
>
for StateChangeActionComponentStateFactory
{
    fn current_state(
        current: &EventDescriptor<ComponentChangeEventData, UiEventArgs, Visibility>,
        ctx: &mut ResMut<UiContext>
    ) -> Vec<NextComponentInsert<Visibility, T, UiContext>> {
        if let UiEventArgs::Event(ui) = &current.event_args {
            return match ui {
                UiClickStateChange::ChangeVisible { entity, update_component, adviser_component } => {
                    vec![
                        NextComponentInsert {
                            insert_component_entity: *entity,
                            adviser_component_entity: *adviser_component,
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

impl InsertComponentChangeFactory<
    ComponentChangeEventData,
    UiEventArgs,
    ChangeGraphingMenu,
    GraphMenuPotential,
    UiContext,
    NextComponentInsert<ChangeGraphingMenu, GraphMenuPotential, UiContext>
>
for StateChangeActionComponentStateFactory
{
    fn current_state(
        current: &EventDescriptor<ComponentChangeEventData, UiEventArgs, ChangeGraphingMenu>,
        ctx: &mut ResMut<UiContext>
    ) -> Vec<NextComponentInsert<ChangeGraphingMenu, GraphMenuPotential, UiContext>> {
        if let UiEventArgs::Event(ui) = &current.event_args {
            return match ui {
                UiClickStateChange::ChangeGraphingMenu { entity, graph_menu_change } => {
                    vec![
                        NextComponentInsert {
                            insert_component_entity: *entity,
                            adviser_component_entity: *entity,
                            next_state: *graph_menu_change,
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
        C: Send + Sync {}

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
