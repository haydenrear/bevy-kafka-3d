use std::env::Args;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{PropagateDisplay, PropagateDraggable, PropagateScrollable};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::os::macos::raw::stat;
use crate::menu::ui_menu_event::ui_menu_event_plugin::SelectOptions;
use bevy::prelude::{Button, Changed, Commands, Component, Display, Entity, EventWriter, Interaction, Query, ResMut, Resource, Style, Vec2, Visibility, With};
use bevy::hierarchy::{Children, Parent};
use bevy::utils::{HashMap, HashSet};
use bevy::log::info;
use bevy::ecs::query::{QueryEntityError, ReadOnlyWorldQuery, WorldQuery};
use bevy::input::mouse::MouseScrollUnit;
use bevy::ui::Size;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_propagation::{ChangePropagation, SideEffectWriter, Relationship};
use crate::event::event_actions::{ClickWriteEvents, RetrieveState};
use crate::event::event_state::{Context, StyleStateChangeEventData};
use crate::menu::{DraggableComponent, ScrollableComponent, UiComponent};
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::next_action::{Matches, UiComponentState};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::types::{
    ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, DraggableUiComponentFilter,
    DraggableUiComponentIxnFilter, PropagateStateTransitionsQuery, PropagationQuery, PropagationQueryFilter,
    ScrollableIxnFilterQuery, ScrollableStateChangeRetriever, ScrollableUiComponentFilter, StateTransitionsQuery,
    StyleUiComponentStateTransitionsQuery, UiComponentStyleFilter, UiComponentStyleIxnFilter,
    UiSelectedComponentStateTransitionsQuery
};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntityComponentStateTransition, StateChangeActionType, TransitionGroup, UiComponentStateTransition, UiComponentStateTransitions, UiEventArgs};
use crate::menu::ui_menu_event::ui_state_change::StateChangeMachine;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever<SelfQueryFilter, InteractionQueryFilterT, S, Ctx, Args, StateMachine, MatchesT, TransitionGroupT> (
    PhantomData<SelfQueryFilter>,
    PhantomData<InteractionQueryFilterT>,
    PhantomData<S>,
    PhantomData<Ctx>,
    PhantomData<Args>,
    PhantomData<StateMachine>,
    PhantomData<MatchesT>,
    PhantomData<TransitionGroupT>,
)
where
    SelfQueryFilter: ReadOnlyWorldQuery,
    InteractionQueryFilterT: ReadOnlyWorldQuery,
    S: Component,
    Ctx: Context,
    Args: EventArgs,
    StateMachine: StateChangeMachine<S, Ctx, Args>,
    MatchesT: Matches<S>,
    TransitionGroupT: TransitionGroup;

impl ClickWriteEvents<
    ClickEvents,
    UiEventArgs, StyleStateChangeEventData, Style, UiContext,
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

impl ClickWriteEvents<
    DraggableStateChangeRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, UiContext,
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

impl ClickWriteEvents<
    ScrollableStateChangeRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, UiContext,
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
> for ScrollEvents {
}

#[derive(Default, Resource, Debug)]
pub struct ClickSelectOptions;

impl ClickWriteEvents<
    ClickSelectionEventRetriever,
    UiEventArgs, StyleStateChangeEventData, Style, UiContext,
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
> for ClickSelectOptions {
}

impl <SelfFilterQuery, SelfIxnFilter, ComponentT, Ctx, EventArgsT, EventDataT, MatchesT, TransitionGroupT> RetrieveState<
    EventArgsT,
    EventDataT,
    ComponentT,
    Ctx,
    StateTransitionsQuery<'_, ComponentT, EventDataT, MatchesT, Ctx, EventArgsT, TransitionGroupT>,
    PropagationQuery<'_, ComponentT>,
    SelfFilterQuery,
    PropagationQueryFilter<ComponentT>,
>
for StateChangeActionTypeStateRetriever<
    SelfFilterQuery, SelfIxnFilter,
    ComponentT, Ctx, EventArgsT, EventDataT,
    MatchesT, TransitionGroupT
>
where
    SelfIxnFilter: ReadOnlyWorldQuery + Send + Sync + 'static,
    SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
    ComponentT: Component + Send + Sync + 'static + Debug,
    Ctx: Context,
    EventArgsT: EventArgs + Debug + 'static,
    EventDataT: StateChangeMachine<ComponentT, Ctx, EventArgsT> + EventData + 'static + Clone,
    MatchesT: Matches<ComponentT>,
    TransitionGroupT: TransitionGroup
{
    fn create_event(
        mut commands: &mut Commands,
        entity: Entity,
        mut style_context: &mut ResMut<Ctx>,
        entity_query: &Query<
            StateTransitionsQuery<'_, ComponentT, EventDataT, MatchesT, Ctx, EventArgsT, TransitionGroupT>,
            SelfFilterQuery
        >,
        propagation_query: &Query<
            PropagationQuery<'_, ComponentT>,
            PropagationQueryFilter<ComponentT>
        >
    ) -> (Vec<EventDescriptor<EventDataT, EventArgsT, ComponentT>>, Vec<SideEffectWriter>)
    {
        let mut event_descriptors = vec![];
        let mut propagate_events = vec![];

        Self::create_event(&entity_query, &propagation_query, &mut style_context, entity)
            .into_iter()
            .for_each(|prop| {
                event_descriptors.push(prop);
            });

        (event_descriptors, propagate_events)

    }
}

impl<SelfFilterQuery, IXN, C, StateMachine, MatchesT, Ctx, EventArgsT, TransitionGroupT>
StateChangeActionTypeStateRetriever<SelfFilterQuery, IXN, C, Ctx, EventArgsT, StateMachine, MatchesT, TransitionGroupT>
    where
        SelfFilterQuery: ReadOnlyWorldQuery + Send + Sync + 'static,
        IXN: ReadOnlyWorldQuery + Send + Sync + 'static,
        C: Component + Debug,
        StateMachine: StateChangeMachine<C, Ctx, EventArgsT> + Send + Sync + EventData + 'static + Clone,
        MatchesT: Matches<C>,
        Ctx: Context,
        EventArgsT: EventArgs + 'static,
        TransitionGroupT: TransitionGroup
{
    fn create_event(
        entity_query: &Query<
            StateTransitionsQuery<'_, C, StateMachine, MatchesT, Ctx, EventArgsT, TransitionGroupT>,
            SelfFilterQuery
        >,
        propagation_query: &Query<
            PropagationQuery<'_, C>,
            PropagationQueryFilter<C>
        >,
        mut style_context: &mut ResMut<Ctx>,
        entity: Entity
    ) -> Vec<EventDescriptor<StateMachine, EventArgsT, C>> {
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
        mut descriptors: &mut Vec<EventDescriptor<StateMachine, EventArgsT, C>>,
        state_change_action_type: &StateChangeActionType<StateMachine, C, Ctx, EventArgsT>,
        related_style: &C,
        entity: Entity
    ) {
        match state_change_action_type {
            StateChangeActionType::Clicked{ value, ..} => {
                value.state_machine_event(related_style, style_context, entity)
                    .map(|args| {
                        EventDescriptor {
                            component: PhantomData::default(),
                            event_data: value.clone(),
                            event_args: args,
                        }
                    })
                    .map(|descriptor| {
                        descriptors.push(descriptor);
                    });
            }
            _ => {

            }
        }
    }
}


macro_rules! state_change_action_retriever_default {
    ($($ty1:ty, $ty2:ty, $ty3:ty, $ty4:ty, $ty5:ty, $ty6:ty),*) => {
        use crate::menu::Menu;
        $(
            impl Default for StateChangeActionTypeStateRetriever<$ty1, $ty2, $ty3, $ty4, $ty5, StyleStateChangeEventData, UiComponentState, $ty6>  {
                fn default() -> Self {
                    Self(
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
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, UiContext, UiEventArgs, PropagateDisplay,
    DraggableUiComponentFilter, DraggableUiComponentIxnFilter, Style, UiContext, UiEventArgs, PropagateDraggable,
    ScrollableUiComponentFilter, ScrollableIxnFilterQuery, Style, UiContext, UiEventArgs, PropagateScrollable,
    UiComponentStyleFilter, UiComponentStyleIxnFilter, Style, UiContext, UiEventArgs, SelectOptions
);
