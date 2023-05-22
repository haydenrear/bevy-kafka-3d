use std::fmt::Debug;
use std::marker::PhantomData;
use crate::menu::ui_menu_event::transition_groups::PropagateVisible;
use crate::menu::ui_menu_event::transition_groups::PropagateSelect;
use bevy::prelude::{Resource, Style, Visibility};
use crate::event::event_actions::EventsSystem;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateDraggable, PropagateScrollable};
use crate::menu::ui_menu_event::type_alias::event_reader_writer::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, PropagationQuery, PropagationQueryFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::type_alias::state_change_action_retriever::{ChangeVisibleEventRetriever, ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, ScrollableStateChangeRetriever};
use crate::menu::ui_menu_event::type_alias::state_transition_queries::{PropagateStateTransitionsQuery, StyleUiComponentStateTransitionsQuery, UiSelectedComponentStateTransitionsQuery, VisibleComponentStateTransitionsQuery};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;
use crate::menu::ui_menu_event::ui_state_change::ChangeVisible;

impl EventsSystem<
    ClickEvents,
    UiEventArgs, StyleStateChangeEventData, Style, Style, UiContext,
    // self query
    StyleUiComponentStateTransitionsQuery<'_>,
    // self filter
    UiComponentStyleFilter,
    // parent query
    PropagationQuery<'_, Style>,
    // parent filter
    PropagationQueryFilter<Style>,
    // interaction filter
    UiComponentStyleIxnFilter
> for ClickEvents {}

#[derive(Default, Resource, Debug)]
pub struct DragEvents;

impl EventsSystem<
    DraggableStateChangeRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, Style, UiContext,
    // self query
    PropagateStateTransitionsQuery<'_, PropagateDraggable>,
    // self filter
    DraggableUiComponentFilter,
    // parent query
    PropagationQuery<'_, Style>,
    // parent filter
    PropagationQueryFilter<Style>,
    // interaction filter
    DraggableUiComponentIxnFilter
> for DragEvents {}

#[derive(Default, Resource, Debug)]
pub struct ScrollEvents;

impl EventsSystem<
    ScrollableStateChangeRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, Style, UiContext,
    // self query
    PropagateStateTransitionsQuery<'_, PropagateScrollable>,
    // self filter
    ScrollableUiComponentFilter,
    // parent query
    PropagationQuery<'_, Style>,
    // parent filter
    PropagationQueryFilter<Style>,
    // interaction filter
    ScrollableIxnFilterQuery
> for ScrollEvents {}

#[derive(Default, Resource, Debug)]
pub struct ClickSelectOptions;

impl EventsSystem<
    ClickSelectionEventRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, Style, UiContext,
    // self query
    UiSelectedComponentStateTransitionsQuery<'_>,
    // self filter
    UiComponentStyleFilter,
    // parent query
    PropagationQuery<'_, Style>,
    // parent filter
    PropagationQueryFilter<Style>,
    // interaction filter
    UiComponentStyleIxnFilter
> for ClickSelectOptions {}

#[derive(Default, Debug, Resource)]
pub struct VisibilitySystems<T: ChangeVisible> {
    v: PhantomData<T>
}

impl<T: ChangeVisible + Debug> EventsSystem<
    ChangeVisibleEventRetriever<T, Visibility>,
    UiEventArgs, ComponentChangeEventData, T, Visibility, UiContext,
    // self query
    VisibleComponentStateTransitionsQuery<'_, T, Visibility>,
    // self filter
    VisibleFilter<T>,
    // propagation query
    PropagationQuery<'_, Visibility>,
    // propagation filter
    PropagationQueryFilter<Visibility>,
    // interaction filter
    VisibleIxnFilter<T>
> for VisibilitySystems<T> {}

