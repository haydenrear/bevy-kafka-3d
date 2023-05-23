use bevy::prelude::{Entity, Style};
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::ui_menu_event::entity_component_state_transition::UiEntityComponentStateTransitions;
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateSelect, PropagateVisible};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::menu::UiComponent;
use crate::pickable_events::PickableComponentState;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub type StateTransitionsQuery<'a, StateComponentT, ChangeComponentT, StateMachineT, UpdateComponentMatchesT, FilterMatches, Ctx, EventArgsT, TransitionGroupT> =
(Entity,
 &'a UiComponent,
 &'a StateComponentT,
 &'a UiIdentifiableComponent,
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

pub type StyleUiComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateDisplay>;

pub type PropagateStateTransitionsQuery<'a, TransitionGroupT> = StateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, TransitionGroupT>;

pub type UiSelectedComponentStateTransitionsQuery<'a> = StateTransitionsQuery<'a, Style, Style, StyleStateChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateSelect>;

pub type VisibleComponentStateTransitionsQuery<'a, ComponentStateT, ComponentChangeT> = StateTransitionsQuery<'a, ComponentStateT, ComponentChangeT, ComponentChangeEventData, UiComponentState, UiComponentState, UiContext, UiEventArgs, PropagateVisible>;

pub type PickableComponentStateTransitionsQuery<'a, ComponentStateT, ComponentChangeT, TransitionGroupT> = StateTransitionsQuery<'a, ComponentStateT, ComponentChangeT, ComponentChangeEventData, PickableComponentState, PickableComponentState, UiContext, UiEventArgs, TransitionGroupT>;
