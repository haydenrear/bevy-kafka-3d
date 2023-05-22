use std::collections::HashSet;
use std::fmt::Debug;
use bevy::prelude::*;
use crate::event::event_descriptor::EventData;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::event::state_transition::get_state_transitions::{ChildFilter, ChildQuery, Entities, EntityFilter, EntityQuery, GetStateTransitions, ParentFilter, ParentQuery};
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::ui_menu_event::next_action::{Matches, UiComponentState};
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateSelect, PropagateVisible, TransitionGroup};
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;

pub struct InsertCollapsableDisplayTransitions;
impl InsertStateTransitions<PropagateDisplay, DrawCollapsableMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState>
for InsertCollapsableDisplayTransitions {}

pub struct InsertSelectStateTransitions;
impl InsertStateTransitions<PropagateSelect, DropdownMenuOptionResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState>
for InsertSelectStateTransitions {}

pub struct InsertDropdownDisplayTransitions;
impl InsertStateTransitions<PropagateDisplay, DrawDropdownMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState>
for InsertDropdownDisplayTransitions {}

pub struct InsertBaseMenuDisplayTransitions;
impl InsertStateTransitions<PropagateDisplay, BuildBaseMenuResult, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState>
for InsertBaseMenuDisplayTransitions {}

pub struct InsertVisibleGraphStateTransitions;
impl InsertStateTransitions<PropagateVisible, GraphMenuResultBuilder, GraphMenuResultBuilder, ComponentChangeEventData, MetricsConfigurationOption<Menu>, UiComponentState, Visibility>
for InsertVisibleGraphStateTransitions {}

pub struct InsertVisibleNetworkStateTransitions;
impl InsertStateTransitions<PropagateVisible, NetworkMenuResultBuilder, NetworkMenuResultBuilder, ComponentChangeEventData, MetricsConfigurationOption<Menu>, UiComponentState, Visibility>
for InsertVisibleNetworkStateTransitions {}

pub trait InsertStateTransitions<
    TransitionGroupT,
    GetStateTransitionsT,
    ResultT: Resource,
    EventDataT: EventData + Debug + 'static,
    StateComponentT: Component,
    FilterMatchesT: Matches<StateComponentT>,
    UpdateComponentT: Component = StateComponentT,
    MatchesT: Matches<UpdateComponentT> = FilterMatchesT
>
where
    TransitionGroupT: TransitionGroup + Debug,
    GetStateTransitionsT: GetStateTransitions<TransitionGroupT, ResultT, EventDataT, StateComponentT, FilterMatchesT, UpdateComponentT, MatchesT> + Debug
{
    fn insert_state_transitions(
        mut commands: Commands,
        build_menu_result: Res<ResultT>,
        entity_query: Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
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
        entity_query: &Query<EntityQuery<TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: &Query<ParentQuery<TransitionGroupT>, ParentFilter<TransitionGroupT>>,
        entity: &Entity
    ) -> Entities
    {
        let children = Self::get_filter_components(&entity_query, Self::child_entities(&entity_query, &with_children_query, *entity));
        let children_recursive = Self::get_filter_components(
            &entity_query,
            Self::children_recursive(*entity, &entity_query, &with_children_query, &with_parent_query)
        );
        let siblings_children_recursive = Self::get_filter_components(
            &entity_query,
            Self::siblings_children_recursive(&entity_query, &with_children_query, &with_parent_query, *entity)
        );
        let parent = Self::get_filter_components(
            &entity_query,
            Self::get_parent(&with_parent_query, *entity)
        );
        let siblings = Self::get_filter_components(
            &entity_query,
            Self::sibling_entities(&entity_query, &with_children_query, &with_parent_query, *entity)
        );
        let siblings_children = Self::get_filter_components(
            &entity_query,
            Self::siblings_children_entities(&entity_query, &with_children_query, &with_parent_query, *entity)
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

    fn get_filter_components(entity_query: &Query<EntityQuery<TransitionGroupT>, EntityFilter<TransitionGroupT>>, entities: Vec<Entity>) -> Vec<Entity> {
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

    fn get_parent(
        with_parent_query: &Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
        entity: Entity,
    ) -> Vec<Entity> {
        let parent = with_parent_query.get(entity.clone())
            .map(|(_, _, parent, updateable, style, _)| parent.get())
            .ok();

        parent.iter().flat_map(|p| vec![*p])
            .collect()

    }

    fn sibling_entities(
        entity_query: &Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: &Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
        entity: Entity,
    ) -> Vec<Entity> {
        let mut entities = HashSet::new();
        let parent = with_parent_query.get(entity.clone())
            .map(|(_, _, parent, updateable, style, _)| parent.get())
            .ok();

        parent.map(|parent| {
            Self::child_entities(entity_query, with_children_query, parent).iter()
                .filter(|e| **e != entity)
                .for_each(|sibling| {
                    entities.insert(*sibling);
                });
        })
            .or_else(|| {
                info!("Failed to fetch parent when parent was included in fetch.");
                None
            });

        entities.into_iter().collect()
    }

    fn siblings_children_entities(
        entity_query: &Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: &Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
        entity: Entity,
    ) -> Vec<Entity> {
        let mut entities = HashSet::new();
        let parent = with_parent_query.get(entity.clone())
            .map(|(_, _, parent, updateable, style, _)| parent.get())
            .ok();

        parent.map(|parent| {
            Self::sibling_entities(entity_query, with_children_query, with_parent_query, entity)
                .iter()
                .for_each(|sibling| {
                    Self::child_entities(entity_query, with_children_query, *sibling).iter()
                        .for_each(|sibling_child| {
                            info!("Including sibling: {:?}.", sibling_child);
                            entities.insert(*sibling_child);
                        });
                });
        })
            .or_else(|| {
                info!("Failed to fetch parent when parent was included in fetch.");
                None
            });

        entities.into_iter().collect()
    }

    fn siblings_children_recursive(
        entity_query: &Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: &Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
        entity: Entity,
    ) -> Vec<Entity>{
        let mut entities = HashSet::new();

        let mut siblings_children = Self::siblings_children_entities(entity_query, with_children_query, with_parent_query, entity);
        let mut siblings_children: HashSet<Entity> = siblings_children.into_iter().collect();

        for sibling_child in siblings_children.iter() {
            info!("Adding sibling child: {:?}", sibling_child);
            Self::add_children_recursive(
                &mut entities,
                &mut HashSet::new(),
                *sibling_child,
                entity_query,
                with_children_query,
                with_parent_query
            );
        }

        entities.into_iter().collect()
    }

    fn children_recursive(
        entity: Entity,
        entity_query: &Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: &Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
    ) -> Vec<Entity> {
        let mut to_return = HashSet::new();
        Self::add_children_recursive(&mut to_return, &mut HashSet::new(), entity, entity_query, with_children_query, with_parent_query);
        to_return.into_iter().filter(|e| *e != entity).collect()
    }

    fn add_children_recursive(
        mut entities_to_return: &mut HashSet<Entity>,
        mut entities_processed: &mut HashSet<Entity>,
        entity: Entity,
        entity_query: &Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        with_parent_query: &Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
    ) {

        let mut next = HashSet::new();
        entities_to_return.insert(entity);

        Self::child_entities(entity_query, with_children_query, entity)
            .iter()
            .for_each(|child| {
                if !entities_processed.contains(child) {
                    entities_to_return.insert(*child);
                    entities_processed.insert(*child);
                    next.insert(*child);
                }
            });

        for child_entity in next.iter() {
            Self::add_children_recursive(
                entities_to_return,
                &mut entities_processed,
                *child_entity,
                entity_query,
                with_children_query,
                with_parent_query
            );
            entities_to_return.insert(*child_entity);
        }
    }

    fn child_entities(
        entity_query: &Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
        with_children_query: &Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
        entity: Entity,
    ) -> Vec<Entity> {
        let mut entities = vec![];

        info!("Including children.");

        let child_entities = with_children_query.get(entity.clone())
            .map(|(_, _, child, update, _, _)| child.iter()
                .filter(|child| **child != entity)
                .map(|child| child.clone())
                .collect::<Vec<Entity>>()
            )
            .ok()
            .or(Some(vec![]))
            .unwrap();

        child_entities.iter()
            .filter(|c| entity_query.get(**c).is_ok())
            .for_each(|child| {
                info!("Fetching child entity: {:?}.", child);
                entities.push(*child);
            });

        entities
    }



}


