use bevy::prelude::{Style, Visibility};
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::entity_component_state_transition::{EntityComponentStateTransition, UiEntityComponentStateTransitions};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateDraggable, PropagateSelect, PropagateVisible};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;

pub type UiStyleComponentStateTransitions = ComponentStateTransitions<PropagateDisplay, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>;
pub type SelectOptionsStateTransitions = ComponentStateTransitions<PropagateSelect, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>;
pub type DraggableStateTransitions = ComponentStateTransitions<PropagateDraggable, StyleStateChangeEventData, Style, UiComponentState, Style, UiComponentState>;

pub type VisibilityStateTransitions = UiEntityComponentStateTransitions<ComponentChangeEventData, UiContext, UiEventArgs, PropagateVisible, MetricsConfigurationOption<Menu>, UiComponentState, Visibility, UiComponentState>;

pub type ComponentStateTransitions<TransitionGroupT, EventData, StateComponentT, FilterMatchesT, UpdateComponentT, UpdateMatchesT> = UiEntityComponentStateTransitions<EventData, UiContext, UiEventArgs, TransitionGroupT, StateComponentT, FilterMatchesT, UpdateComponentT, UpdateMatchesT>;

pub type UiSelectedComponentStateTransitions = UiEntityComponentStateTransitions<StyleStateChangeEventData, UiContext, UiEventArgs, PropagateSelect, Style, UiComponentState>;
pub type UiSelectedComponentStateTransition = EntityComponentStateTransition<StyleStateChangeEventData, UiContext, UiEventArgs, PropagateSelect, Style, UiComponentState>;

pub type UiStyleEntityComponentStateTransitions = UiEntityComponentStateTransitions<StyleStateChangeEventData, UiContext, UiEventArgs, PropagateDisplay, Style, UiComponentState>;
