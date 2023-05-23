use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::{Commands, Component, Entity, Query, ResMut};
use std::fmt::Debug;
use bevy::log::info;
use std::marker::PhantomData;
use crate::event::event_actions::RetrieveState;
use crate::event::event_descriptor::{EventArgs, EventData, EventDescriptor};
use crate::event::event_state::Context;
use crate::menu::ui_menu_event::next_action::Matches;
use crate::menu::ui_menu_event::state_change_factory::StateChangeActionType;
use crate::menu::ui_menu_event::transition_groups::TransitionGroup;
use crate::menu::ui_menu_event::type_alias::event_reader_writer_filter::{PropagationQuery, PropagationQueryFilter};
use crate::menu::ui_menu_event::type_alias::state_transition_queries::StateTransitionsQuery;
use crate::menu::ui_menu_event::ui_event_writer::action_retriever::state_change_action_retriever::StateChangeActionTypeStateRetriever;
use crate::menu::ui_menu_event::ui_state_change::StateChangeMachine;

impl<SelfFilterQuery,
 SelfIxnFilter,
 ComponentT,
 ComponentChangeT,
 Ctx,
 EventArgsT,
 StateChangeMachineT,
 FilterMatchesT,
 MatchesT,
 TransitionGroupT
> RetrieveState<
    EventArgsT,
    StateChangeMachineT,
    Ctx,
    StateTransitionsQuery<'_, ComponentT, ComponentChangeT, StateChangeMachineT, MatchesT, FilterMatchesT, Ctx, EventArgsT, TransitionGroupT>,
    PropagationQuery<'_, ComponentChangeT>,
    ComponentT,
    ComponentChangeT,
    SelfFilterQuery,
    PropagationQueryFilter<ComponentChangeT>,
>
for StateChangeActionTypeStateRetriever<
    SelfFilterQuery, SelfIxnFilter,
    Ctx, EventArgsT, StateChangeMachineT,
    TransitionGroupT, ComponentT, FilterMatchesT,
    ComponentChangeT, MatchesT,
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

impl<SelfQueryT, InteractionQueryT, ComponentStateComponentT, ComponentUpdateComponentT, StateMachine, MatchesT, FilterMatchesT, Ctx, EventArgsT, TransitionGroupT>
StateChangeActionTypeStateRetriever<
    SelfQueryT,
    InteractionQueryT,
    Ctx,
    EventArgsT,
    StateMachine,
    TransitionGroupT,
    ComponentStateComponentT,
    FilterMatchesT,
    ComponentUpdateComponentT,
    MatchesT,
>
where
    SelfQueryT: ReadOnlyWorldQuery + Send + Sync + 'static,
    InteractionQueryT: ReadOnlyWorldQuery + Send + Sync + 'static,
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
            SelfQueryT
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

                    info!("Testing if {:?} matches.", related_style);
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
