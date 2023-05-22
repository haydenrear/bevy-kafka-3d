use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::event::event_descriptor::EventArgs;
use crate::event::event_state::Context;
use crate::menu::ui_menu_event::next_action::Matches;
use crate::menu::ui_menu_event::transition_groups::TransitionGroup;
use crate::menu::ui_menu_event::ui_state_change::StateChangeMachine;

use crate::menu::ui_menu_event::transition_groups::PropagateVisible;
use crate::menu::ui_menu_event::transition_groups::PropagateSelect;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::next_action::UiComponentState;
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateDraggable, PropagateScrollable};
use crate::menu::ui_menu_event::type_alias::event_reader_writer::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, PropagationQuery, PropagationQueryFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiEventArgs;

#[derive(Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever<
    SelfQueryFilter,
    InteractionQueryFilterT,
    ComponentStateT,
    ComponentChangeT,
    Ctx,
    EventArgsT,
    StateMachineT,
    FilterMatchesT,
    UpdateComponentMatchesT,
    TransitionGroupT
> (
    PhantomData<SelfQueryFilter>,
    /// may be able to remove the below parameter at some point.
    PhantomData<InteractionQueryFilterT>,
    PhantomData<ComponentStateT>,
    PhantomData<ComponentChangeT>,
    PhantomData<Ctx>,
    PhantomData<EventArgsT>,
    PhantomData<StateMachineT>,
    PhantomData<FilterMatchesT>,
    PhantomData<UpdateComponentMatchesT>,
    PhantomData<TransitionGroupT>,
)
    where
        SelfQueryFilter: ReadOnlyWorldQuery,
        InteractionQueryFilterT: ReadOnlyWorldQuery,
        ComponentStateT: Component,
        Ctx: Context,
        EventArgsT: EventArgs,
        StateMachineT: StateChangeMachine<ComponentChangeT, Ctx, EventArgsT>,
        FilterMatchesT: Matches<ComponentStateT>,
        TransitionGroupT: TransitionGroup;


macro_rules! state_change_action_retriever_default {
    ($($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty),*) => {
        $(
            impl Default for StateChangeActionTypeStateRetriever<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, StyleStateChangeEventData, UiComponentState, UiComponentState, $ty7>  {
                fn default() -> Self {
                    Self(
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default()
                    )
                }
            }
        )*
    }
}

state_change_action_retriever_default!(
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, Style, UiContext, UiEventArgs, PropagateDisplay,
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style, Style, UiContext, UiEventArgs, PropagateDraggable,
    ScrollableUiComponentFilter, ScrollableIxnFilterQuery, Style, Style, UiContext, UiEventArgs, PropagateScrollable,
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, Style, UiContext, UiEventArgs, PropagateSelect
);

macro_rules! state_change_action_component_change {
    ($($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty, $ty7:ty),*) => {
        $(
            impl Default for StateChangeActionTypeStateRetriever<$ty1, $ty2, $ty3, $ty4, $ty5, $ty6, ComponentChangeEventData, UiComponentState, UiComponentState, $ty7>  {
                fn default() -> Self {
                    Self(
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default(),
                        PhantomData::default()
                    )
                }
            }
        )*
    }
}

state_change_action_component_change!(
    VisibleFilter<MetricsConfigurationOption<Menu>>,
    VisibleIxnFilter<MetricsConfigurationOption<Menu>>,
    MetricsConfigurationOption<Menu>,
    Visibility, UiContext, UiEventArgs, PropagateVisible
);
