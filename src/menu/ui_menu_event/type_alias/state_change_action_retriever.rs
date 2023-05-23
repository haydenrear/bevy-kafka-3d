use bevy::prelude::Style;
use crate::cursor_adapter::PickableComponent;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::state_change_factory::StateChangeActionType;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateDraggable, PropagateCreateMenu, PropagateScrollable, PropagateSelect, PropagateVisible};
use crate::menu::ui_menu_event::type_alias::event_reader_writer_filter::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, PickableFilter, PickableIxnFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_event_writer::action_retriever::state_change_action_retriever::StateChangeActionTypeStateRetriever;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::menu::UiComponent;
use crate::pickable_events::PickableComponentState;

pub type UiStateChange<C, S> = StateChangeActionType<S, C, UiContext, UiEventArgs>;
pub type StyleStateChange = StateChangeActionType<StyleStateChangeEventData, Style, UiContext, UiEventArgs>;

pub type DraggableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    UiComponent, DraggableUiComponentFilter, DraggableUiComponentIxnFilter,
    UiContext, UiEventArgs, StyleStateChangeEventData, PropagateDraggable,
    Style, UiComponentState
>;

pub type ScrollableStateChangeRetriever = StateChangeActionTypeStateRetriever<
    UiComponent,ScrollableUiComponentFilter, ScrollableIxnFilterQuery, UiContext,
    UiEventArgs, StyleStateChangeEventData, PropagateScrollable,
    Style, UiComponentState
>;

pub type ClickEvents = StateChangeActionTypeStateRetriever<
    UiComponent, UiComponentStyleFilter, UiComponentStyleIxnFilter,
    UiContext, UiEventArgs, StyleStateChangeEventData,
    PropagateDisplay, Style, UiComponentState
>;


pub type ClickSelectionEventRetriever = StateChangeActionTypeStateRetriever<
    UiComponent,UiComponentStyleFilter, UiComponentStyleIxnFilter,
    UiContext, UiEventArgs, StyleStateChangeEventData, PropagateSelect,
    Style, UiComponentState
>;

pub type ChangeVisibleEventRetriever<StateComponentT, ChangeComponentT> = StateChangeActionTypeStateRetriever<
    UiComponent, VisibleFilter<StateComponentT>, VisibleIxnFilter<StateComponentT>,
    UiContext, UiEventArgs, ComponentChangeEventData, PropagateVisible,
    StateComponentT, UiComponentState, ChangeComponentT
>;

pub type CreateMenuPickableEventRetriever<StateComponentT, ChangeComponentT> = StateChangeActionTypeStateRetriever<
    PickableComponent, PickableFilter<StateComponentT>, PickableIxnFilter<StateComponentT>,
    UiContext, UiEventArgs, ComponentChangeEventData, PropagateCreateMenu,
    StateComponentT, PickableComponentState, ChangeComponentT
>;
