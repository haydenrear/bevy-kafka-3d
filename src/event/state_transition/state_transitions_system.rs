use std::collections::HashSet;
use std::fmt::Debug;
use bevy::prelude::*;
use crate::event::event_descriptor::EventData;
use crate::event::state_transition::get_state_transitions::{Entities, GetStateTransitions};
use crate::event::state_transition::parent_child_queries::ParentChildQueries;
use crate::event::state_transition::state_transition_types::{ChildFilterType, ChildQueryType, EntityFilterType, EntityQueryType, ParentFilterType, ParentQueryType, UiChildFilter, UiChildQuery, UiEntityFilter, UiEntityQuery, UiParentFilter, UiParentQuery};
use crate::menu::ui_menu_event::next_action::Matches;
use crate::menu::ui_menu_event::transition_groups::TransitionGroup;

pub trait InsertStateTransitions<
    'a,
    TransitionGroupT,
    GetStateTransitionsT,
    ResultT: Resource,
    EventDataT: EventData + Debug + 'static,
    ParentChildQueriesT: ParentChildQueries<'a, TransitionGroupT, StateComponentT, ComponentTypeT>,
    ComponentTypeT: Component,
    StateComponentT: Component,
    FilterMatchesT: Matches<StateComponentT>,
    UpdateComponentT: Component = StateComponentT,
    MatchesT: Matches<UpdateComponentT> = FilterMatchesT
>
where
    TransitionGroupT: TransitionGroup + Debug,
    GetStateTransitionsT: GetStateTransitions<
        TransitionGroupT,
        ResultT,
        EventDataT,
        StateComponentT,
        FilterMatchesT,
        UpdateComponentT,
        MatchesT
    > + Debug
{
    fn insert_state_transitions(
        mut commands: Commands,
        build_menu_result: Res<ResultT>,
        entity_query: Query<
            EntityQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            EntityFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        with_children_query: Query<
            ChildQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            ChildFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        with_parent_query: Query<
            ParentQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            ParentFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
    ) {
        for entity in GetStateTransitionsT::get_entities(&build_menu_result).iter() {

            let entities = Self::populate_entities_filtered(&entity_query, &with_children_query, &with_parent_query, entity);

            GetStateTransitionsT::get_state_transitions(&build_menu_result, &entities)
                .map(|entity_state_transitions| commands.get_entity(*entity)
                    .map(|mut entity_commands| {
                        // info!("Adding state transition: {:?} from collapsable: {:?}.", &entity_state_transitions, entity);
                        entity_commands.insert(entity_state_transitions);
                    })
                );
        }
    }


    fn populate_entities_filtered(
        entity_query: &Query<
            EntityQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            EntityFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        with_children_query: &Query<
            ChildQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            ChildFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        with_parent_query: &Query<
            ParentQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            ParentFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        entity: &Entity
    ) -> Entities
    {
        let children = Self::get_filter_components(&entity_query, ParentChildQueriesT::child_entities(&entity_query, &with_children_query, *entity));
        let children_recursive = Self::get_filter_components(
            &entity_query,
            ParentChildQueriesT::children_recursive(*entity, &entity_query, &with_children_query, &with_parent_query)
        );
        let siblings_children_recursive = Self::get_filter_components(
            &entity_query,
            ParentChildQueriesT::siblings_children_recursive(&entity_query, &with_children_query, &with_parent_query, *entity)
        );
        let parent = Self::get_filter_components(
            &entity_query,
            ParentChildQueriesT::get_parent(&with_parent_query, *entity)
        );
        let siblings = Self::get_filter_components(
            &entity_query,
            ParentChildQueriesT::sibling_entities(&entity_query, &with_children_query, &with_parent_query, *entity)
        );
        let siblings_children = Self::get_filter_components(
            &entity_query,
            ParentChildQueriesT::siblings_children_entities(&entity_query, &with_children_query, &with_parent_query, *entity)
        );
        let entities = Entities {
            siblings,
            siblings_children,
            children,
            siblings_children_recursive,
            parent,
            self_state: Some(*entity),
            children_recursive,
        };
        entities
    }

    fn get_filter_components(
        entity_query: &Query<
            EntityQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            EntityFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        entities: Vec<Entity>
    ) -> Vec<Entity> {
        let children = entities
            .into_iter()
            .filter(|child| {
                entity_query.get_component::<TransitionGroupT>(*child).or_else(|e| {
                    error!("Error: {:?}", e);
                    Err(e)
                }).is_ok()
            })
            .collect();
        children
    }

}


