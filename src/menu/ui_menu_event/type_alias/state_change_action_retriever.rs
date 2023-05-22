use bevy::prelude::Style;
use crate::cursor_adapter::RayCastActionable;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::state_change_factory::StateChangeActionType;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateDraggable, PropagateRaycast, PropagateScrollable, PropagateSelect, PropagateVisible};
use crate::menu::ui_menu_event::type_alias::event_reader_writer::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, RaycastFilter, RaycastIxnFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_event_writer::action_retriever::state_change_action_retriever::StateChangeActionTypeStateRetriever;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;

pub type UiStateChange<C, S> = StateChangeActionType<S, C, UiContext, UiEventArgs>;
pub type StyleStateChange = StateChangeActionType<StyleStateChangeEventData, Style, UiContext, UiEventArgs>;

pub type DraggableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter,
    UiContext, UiEventArgs, StyleStateChangeEventData, PropagateDraggable,
    Style, UiComponentState
>;

pub type ScrollableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    ScrollableUiComponentFilter, ScrollableIxnFilterQuery, UiContext,
    UiEventArgs, StyleStateChangeEventData, PropagateScrollable,
    Style, UiComponentState
>;

pub type ClickEvents = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter,
    UiContext, UiEventArgs, StyleStateChangeEventData,
    PropagateDisplay, Style, UiComponentState
>;


pub type ClickSelectionEventRetriever = StateChangeActionTypeStateRetriever<
    UiComponentStyleFilter, UiComponentStyleIxnFilter,
    UiContext, UiEventArgs, StyleStateChangeEventData, PropagateSelect,
    Style, UiComponentState
>;

pub type RaycastActionableEventRetriever = StateChangeActionTypeStateRetriever<
    RaycastFilter, RaycastIxnFilter,
    UiContext, UiEventArgs, ComponentChangeEventData, UiComponentState, PropagateRaycast,
    RayCastActionable, UiComponentState, RayCastActionable,
>;

pub type ChangeVisibleEventRetriever<StateComponentT, ChangeComponentT> = StateChangeActionTypeStateRetriever<
    VisibleFilter<StateComponentT>, VisibleIxnFilter<StateComponentT>,
    UiContext, UiEventArgs, ComponentChangeEventData, PropagateVisible,
    StateComponentT, UiComponentState, ChangeComponentT
>;
