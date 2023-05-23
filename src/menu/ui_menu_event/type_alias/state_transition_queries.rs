use bevy::prelude::{Entity, Style};
use crate::cursor_adapter::PickableComponent;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::ui_menu_event::entity_component_state_transition::UiEntityComponentStateTransitions;
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateSelect, PropagateVisible};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::menu::UiComponent;
use crate::pickable_events::PickableComponentState;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub type StateTransitionsQuery<'a, IdComponent, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT> =
(Entity,
 &'a IdComponent,
 &'a StateComponentT,
 &'a UiEntityComponentStateTransitions<
     StateMachineT,
     Ctx,
     EventArgsT,
     TransitionGroupT,
     StateComponentT,
     FilterMatches,
     ChangeComponentT,
     UpdateComponentMatchesT,
 >);

pub type UiStateTransitionsQuery<'a, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT> =
 StateTransitionsQuery<'a, UiComponent, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT>;

pub type StyleUiComponentStateTransitionsQuery<'a> = UiStateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateDisplay>;

pub type PropagateStateTransitionsQuery<'a, TransitionGroupT> = UiStateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, TransitionGroupT>;

pub type UiSelectedComponentStateTransitionsQuery<'a> = UiStateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateSelect>;

pub type VisibleComponentStateTransitionsQuery<'a, ComponentStateT, ComponentChangeT> = UiStateTransitionsQuery<'a, ComponentStateT, ComponentChangeT, ComponentChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateVisible>;

pub type PickableStateTransitionsQuery<'a, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT> =
StateTransitionsQuery<'a, PickableComponent, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT>;

pub type PickableComponentStateTransitionsQuery<'a, ComponentStateT, ComponentChangeT, TransitionGroupT> = PickableStateTransitionsQuery<
    'a,
    ComponentStateT,
    ComponentChangeT,
    ComponentChangeEventData,
    PickableComponentState,
    PickableComponentState,
    UiContext,
    UiEventArgs,
    TransitionGroupT
>;
