use std::env::Args;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{PropagateDisplay, PropagateDraggable, PropagateScrollable};
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::menu::PropagateVisible;
use std::os::macos::raw::stat;
use crate::menu::ui_menu_event::ui_menu_event_plugin::PropagateSelect;
use bevy::prelude::{Button, Changed, Commands, Component, Display, Entity, EventWriter, Interaction, Query, ResMut, Resource, Style, Vec2, Visibility, With};
use bevy::hierarchy::{Children, Parent};
use bevy::utils::{HashMap, HashSet};
use bevy::log::info;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::input::mouse::MouseScrollUnit;
use bevy::ui::Size;
use crate::cursor_adapter::RayCastActionable;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{SideEffectWriter, Relationship};
use crate::event::event_actions::{EventsSystem, RetrieveState};
use crate::event::event_state::{ComponentChangeEventData, Context, StyleStateChangeEventData};
use crate::menu::{DraggableComponent, Menu, MetricsConfigurationOption, ScrollableComponent, UiComponent};
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::next_action::{Matches, UiComponentState};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::types::{ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, DraggableUiComponentFilter, DraggableUiComponentIxnFilter, PropagateStateTransitionsQuery, PropagationQuery, PropagationQueryFilter, RaycastActionableEventRetriever, VisibleComponentStateTransitionsQuery, RaycastFilter, RaycastIxnFilter, ScrollableIxnFilterQuery, ScrollableStateChangeRetriever, ScrollableUiComponentFilter, StateTransitionsQuery, StyleUiComponentStateTransitionsQuery, UiComponentStyleFilter, UiComponentStyleIxnFilter, UiSelectedComponentStateTransitionsQuery, VisibleFilter, VisibleIxnFilter, ChangeVisibleEventRetriever};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionType, TransitionGroup, UiEventArgs};
use crate::menu::ui_menu_event::ui_state_change::{ChangeVisible, StateChangeMachine};
use crate::network::NetworkMember;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever<
    SelfQueryFilter,
    InteractionQueryFilterT,
    ComponentStateT,
    ComponentChangeT,
    Ctx, Args, StateMachine,
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
    PhantomData<Args>,
    PhantomData<StateMachine>,
    PhantomData<FilterMatchesT>,
    PhantomData<UpdateComponentMatchesT>,
    PhantomData<TransitionGroupT>,
)
    where
        SelfQueryFilter: ReadOnlyWorldQuery,
        InteractionQueryFilterT: ReadOnlyWorldQuery,
        ComponentStateT: Component,
        Ctx: Context,
        Args: EventArgs,
        StateMachine: StateChangeMachine<ComponentChangeT, Ctx, Args>,
        FilterMatchesT: Matches<ComponentStateT>,
        TransitionGroupT: TransitionGroup;

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

#[derive(Default, Resource, Debug)]
pub struct VisibilitySystems<T: ChangeVisible> {
    v: PhantomData<T>
}

impl EventsSystem<
    ChangeVisibleEventRetriever<MetricsConfigurationOption<Menu>, Visibility>,
    UiEventArgs, ComponentChangeEventData, MetricsConfigurationOption<Menu>, Visibility, UiContext,
    // self query
    VisibleComponentStateTransitionsQuery<'_, MetricsConfigurationOption<Menu>, Visibility>,
    // self filter
    VisibleFilter<MetricsConfigurationOption<Menu>>,
    // propagation query
    PropagationQuery<'_, Visibility>,
    // propagation filter
    PropagationQueryFilter<Visibility>,
    // interaction filter
    VisibleIxnFilter<MetricsConfigurationOption<Menu>>
> for VisibilitySystems<MetricsConfigurationOption<Menu>> {}

impl<SelfFilterQuery, SelfIxnFilter, ComponentT, ComponentChangeT, Ctx, EventArgsT, StateChangeMachineT, FilterMatchesT, MatchesT, TransitionGroupT> RetrieveState<
    EventArgsT,
    StateChangeMachineT,
    ComponentT,
    ComponentChangeT,
    Ctx,
    StateTransitionsQuery<'_, ComponentT, ComponentChangeT, StateChangeMachineT, MatchesT, FilterMatchesT, Ctx, EventArgsT, TransitionGroupT>,
    PropagationQuery<'_, ComponentChangeT>,
    SelfFilterQuery,
    PropagationQueryFilter<ComponentChangeT>,
>
for StateChangeActionTypeStateRetriever<
    SelfFilterQuery, SelfIxnFilter,
    ComponentT, ComponentChangeT, Ctx, EventArgsT, StateChangeMachineT,
    FilterMatchesT, MatchesT, TransitionGroupT
>
    where
        SelfIxnFilter: ReadOnlyWorldQuery + Send + Sync + 'static,
        SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
        ComponentT: Component + Send + Sync + 'static + Debug,
        ComponentChangeT: Component + Send + Sync + 'static + Debug,
        Ctx: Context,
        EventArgsT: EventArgs + Debug + 'static,
        StateChangeMachineT: StateChangeMachine<ComponentChangeT, Ctx, EventArgsT> + EventData + 'static + Clone,
        FilterMatchesT: Matches<ComponentT>,
        MatchesT: Matches<ComponentChangeT>,
        TransitionGroupT: TransitionGroup
{
    fn create_event(
        mut commands: &mut Commands,
        entity: Entity,
        mut style_context: &mut ResMut<Ctx>,
        entity_query: &Query<
            StateTransitionsQuery<'_, ComponentT, ComponentChangeT, StateChangeMachineT, MatchesT, FilterMatchesT, Ctx, EventArgsT, TransitionGroupT>,
            SelfFilterQuery
        >,
        propagation_query: &Query<
            PropagationQuery<'_, ComponentChangeT>,
            PropagationQueryFilter<ComponentChangeT>
        >,
    ) -> Vec<EventDescriptor<StateChangeMachineT, EventArgsT, ComponentChangeT>>
    {
        let mut event_descriptors = vec![];

        Self::create_event(&entity_query, &propagation_query, &mut style_context, entity)
            .into_iter()
            .for_each(|prop| {
                event_descriptors.push(prop);
            });

        event_descriptors
    }
}

impl<SelfFilterQuery, IXN, ComponentStateComponentT, ComponentUpdateComponentT, StateMachine, MatchesT, FilterMatchesT, Ctx, EventArgsT, TransitionGroupT>
StateChangeActionTypeStateRetriever<SelfFilterQuery, IXN, ComponentStateComponentT, ComponentUpdateComponentT, Ctx, EventArgsT, StateMachine, FilterMatchesT, MatchesT, TransitionGroupT>
where
    SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
    IXN: ReadOnlyWorldQuery + Send + Sync + 'static,
    ComponentStateComponentT: Component + Debug,
    ComponentUpdateComponentT: Component + Debug,
    StateMachine: StateChangeMachine<ComponentUpdateComponentT, Ctx, EventArgsT> + Send + Sync + EventData + 'static + Clone,
    MatchesT: Matches<ComponentUpdateComponentT>,
    FilterMatchesT: Matches<ComponentStateComponentT>,
    Ctx: Context,
    EventArgsT: EventArgs + 'static,
    TransitionGroupT: TransitionGroup
{
    fn create_event(
        entity_query: &Query<
            StateTransitionsQuery<'_, ComponentStateComponentT, ComponentUpdateComponentT, StateMachine, MatchesT, FilterMatchesT, Ctx, EventArgsT, TransitionGroupT>,
            SelfFilterQuery
        >,
        propagation_query: &Query<
            PropagationQuery<'_, ComponentUpdateComponentT>,
            PropagationQueryFilter<ComponentUpdateComponentT>
        >,
        mut style_context: &mut ResMut<Ctx>,
        entity: Entity,
    ) -> Vec<EventDescriptor<StateMachine, EventArgsT, ComponentUpdateComponentT>> {
        entity_query.get(entity)
            .iter()
            .flat_map(|entity| {
                entity.4.transitions.iter()
                    .map(|transition| (entity.0, entity.1, entity.2, entity.3, transition))
            })
            .flat_map(|entity| {
                let mut descriptors = vec![];

                if !entity.4.filter_state.matches(entity.2) {
                    return vec![];
                }

                for (related_entity, _, state_change_action_type) in entity.4.entity_to_change.states.iter() {
                    let (_, related_style, _) = propagation_query.get(*related_entity).unwrap();

                    if !entity.4.current_state_filter.matches(related_style) {
                        info!("Did not match.");
                        continue;
                    }

                    Self::create_add_event(&mut style_context, &mut descriptors, state_change_action_type, &related_style, *related_entity);

                }

                descriptors
            })
            .collect()
    }

    fn create_add_event(
        mut style_context: &mut ResMut<Ctx>,
        mut descriptors: &mut Vec<EventDescriptor<StateMachine, EventArgsT, ComponentUpdateComponentT>>,
        state_change_action_type: &StateChangeActionType<StateMachine, ComponentUpdateComponentT, Ctx, EventArgsT>,
        related_style: &ComponentUpdateComponentT,
        entity: Entity,
    ) {
        let state_machine = state_change_action_type.get_state_machine();
        state_machine.state_machine_event(related_style, style_context, entity)
                .map(|args| {
                    EventDescriptor {
                        component: PhantomData::default(),
                        event_data: state_machine.clone(),
                        event_args: args,
                    }
                })
                .map(|descriptor| {
                    descriptors.push(descriptor);
                });
    }
}


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
    VisibleFilter<MetricsConfigurationOption<Menu>>, VisibleIxnFilter<MetricsConfigurationOption<Menu>>, MetricsConfigurationOption<Menu>, Visibility, UiContext, UiEventArgs, PropagateVisible
);
