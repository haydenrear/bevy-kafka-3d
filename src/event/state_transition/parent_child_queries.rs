use std::collections::HashSet;
use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::{Component, Entity, info, Query, Visibility};
use bevy::ui::Style;
use crate::cursor_adapter::PickableComponent;
use crate::event::state_transition::state_transition_types::{ChildFilterType, ChildQueryType, EntityFilterType, EntityQueryType, ParentFilterType, ParentQueryType, UiEntityQuery};
use crate::menu::ui_menu_event::transition_groups::{PropagateDisplay, PropagateCreateMenu, PropagateVisible, TransitionGroup};
use crate::menu::{MetricsConfigurationOption, UiComponent};
use crate::menu::graphing_menu::graph_menu::GraphMenuPotential;
use crate::menu::ui_menu_event::ui_state_change::ChangeVisible;

pub struct StyleUiComponentQueries<TransitionGroupT>
    where
        TransitionGroupT: TransitionGroup + Debug {
    t: PhantomData<TransitionGroupT>
}

impl <'a, TransitionGroupT> ParentChildQueries<'a, TransitionGroupT, Style, UiComponent> for StyleUiComponentQueries<TransitionGroupT>
    where
        TransitionGroupT: TransitionGroup + Debug {}

pub struct VisibilityComponentQueries<ChangeVisibleT: ChangeVisible> {
    change_visible: PhantomData<ChangeVisibleT>
}

impl <'a, ChangeVisibleT: ChangeVisible> ParentChildQueries<'a, PropagateVisible, ChangeVisibleT, UiComponent>
for VisibilityComponentQueries<ChangeVisibleT> {}

pub struct CreateMenuQueries;

impl <'a> ParentChildQueries<'a, PropagateCreateMenu, GraphMenuPotential, PickableComponent>
for CreateMenuQueries {}

pub trait ParentChildQueries<'a, TransitionGroupT, StateComponentT, ComponentTypeT>
where
    TransitionGroupT: TransitionGroup + Debug,
    StateComponentT: Component,
    ComponentTypeT: Component
{

    fn get_parent(
        with_parent_query: &Query<
            ParentQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            ParentFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        entity: Entity,
    ) -> Vec<Entity> {
        let parent = with_parent_query.get(entity.clone())
            .map(|(_, _, parent, updateable, style)| parent.get())
            .ok();

        parent.iter().flat_map(|p| vec![*p])
            .collect()

    }

    fn sibling_entities(
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
        entity: Entity,
    ) -> Vec<Entity> {
        let mut entities = HashSet::new();
        let parent = with_parent_query.get(entity.clone())
            .map(|(_, _, parent, updateable, _)| parent.get())
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
        entity: Entity,
    ) -> Vec<Entity> {
        let mut entities = HashSet::new();
        let parent = with_parent_query.get(entity.clone())
            .map(|(_, _, parent, updateable, _)| parent.get())
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
    ) -> Vec<Entity> {
        let mut to_return = HashSet::new();
        Self::add_children_recursive(&mut to_return, &mut HashSet::new(), entity, entity_query, with_children_query, with_parent_query);
        to_return.into_iter().filter(|e| *e != entity).collect()
    }

    fn add_children_recursive(
        mut entities_to_return: &mut HashSet<Entity>,
        mut entities_processed: &mut HashSet<Entity>,
        entity: Entity,
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
        entity_query: &Query<
            EntityQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            EntityFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        with_children_query: &Query<
            ChildQueryType<'_, TransitionGroupT, StateComponentT, ComponentTypeT>,
            ChildFilterType<TransitionGroupT, StateComponentT, ComponentTypeT>
        >,
        entity: Entity,
    ) -> Vec<Entity> {
        let mut entities = vec![];

        info!("Including children.");

        let child_entities = with_children_query.get(entity.clone())
            .map(|(_, _, child, update, _)| child.iter()
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

