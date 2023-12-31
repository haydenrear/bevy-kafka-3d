use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Resource, Style, Visibility};
use crate::event::event_actions::EventsSystem;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::graph::Graph;
use crate::menu::graphing_menu::graph_menu::{ChangeGraphingMenu, GraphMenuPotential};
use crate::menu::ui_menu_event::transition_groups::{PropagateCreateMenu, PropagateDisplay, PropagateDraggable, PropagateScrollable};
use crate::menu::ui_menu_event::type_alias::event_reader_writer_filter::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, PickableFilter, PickableIxnFilter, PickingPropagationQuery, PickingPropagationQueryFilter, PropagationQuery, PropagationQueryFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, UiPropagationQuery, UiPropagationQueryFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::type_alias::state_change_action_retriever::{ChangeVisibleEventRetriever, ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, CreateMenuPickableEventRetriever, ScrollableStateChangeRetriever};
use crate::menu::ui_menu_event::type_alias::state_transition_queries::{PickableComponentStateTransitionsQuery, PropagateStateTransitionsQuery, StyleUiComponentStateTransitionsQuery, UiSelectedComponentStateTransitionsQuery, VisibleComponentStateTransitionsQuery};
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
    UiPropagationQuery<'_, Style>,
    // parent filter
    UiPropagationQueryFilter<Style>,
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
    UiPropagationQuery<'_, Style>,
    // parent filter
    UiPropagationQueryFilter<Style>,
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
    UiPropagationQuery<'_, Style>,
    // parent filter
    UiPropagationQueryFilter<Style>,
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
    UiPropagationQuery<'_, Style>,
    // parent filter
    UiPropagationQueryFilter<Style>,
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
    UiPropagationQuery<'_, Visibility>,
    // propagation filter
    UiPropagationQueryFilter<Visibility>,
    // interaction filter
    VisibleIxnFilter<T>
> for VisibilitySystems<T> {}

#[derive(Default, Debug, Resource)]
pub struct CreateGraphingMenuSystem;

impl EventsSystem<
    CreateMenuPickableEventRetriever<GraphMenuPotential, ChangeGraphingMenu>,
    UiEventArgs, ComponentChangeEventData, GraphMenuPotential, ChangeGraphingMenu, UiContext,
    // self query
    PickableComponentStateTransitionsQuery<'_, GraphMenuPotential, ChangeGraphingMenu, PropagateCreateMenu>,
    // self filter
    PickableFilter<GraphMenuPotential>,
    // propagation query
    PickingPropagationQuery<'_, ChangeGraphingMenu>,
    // propagation filter
    PickingPropagationQueryFilter<ChangeGraphingMenu>,
    // interaction filter
    PickableIxnFilter<GraphMenuPotential>
> for CreateGraphingMenuSystem {}
