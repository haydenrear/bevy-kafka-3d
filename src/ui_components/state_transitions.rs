use std::collections::{HashMap, HashSet};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy::tasks::ParallelSlice;
use crate::event::event_propagation::Relationship;
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponentStateTransitions;
use crate::menu::{Entities, GetStateTransitions, UiComponent};
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::collapsable_menu::{CollapsableMenuBuilder, DrawCollapsableMenuResult};
use crate::ui_components::menu_components::dropdown_menu::DrawDropdownMenuResult;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

type EntityQuery<'a> = (Entity, &'a UiComponent, &'a Style, &'a UiIdentifiableComponent);
type EntityFilter = (With<UiComponent>, With<Style>);
type ChildQuery<'a> = (Entity, &'a UiComponent, &'a Children, &'a UiIdentifiableComponent, &'a Style);
type ChildFilter = (With<UiComponent>, With<Children>, With<Style>);
type ParentQuery<'a> = (Entity, &'a UiComponent, &'a Parent, &'a UiIdentifiableComponent, &'a Style);
type ParentFilter = (With<UiComponent>, With<Parent>, With<Style>);

pub(crate) fn insert_state_transitions(
    mut commands: Commands,
    build_menu_result: Res<BuildMenuResult>,
    entity_query: Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: Query<ChildQuery<'_>, ChildFilter>,
    with_parent_query: Query<ParentQuery<'_>, ParentFilter>,
) {
    for (entity, dropdown) in build_menu_result.base_menu_results.iter() {
        let children = child_entities(&entity_query, &with_children_query, *entity);
        let siblings_children_recursive = siblings_children_recursive(&entity_query, &with_children_query, &with_parent_query, *entity);
        let children_recursive = children_recursive(*entity, &entity_query, &with_children_query, &with_parent_query);
        let entities = Entities {
            siblings: vec![],
            siblings_children: vec![],
            children,
            siblings_children_recursive,
            parent: vec![],
            self_state: *entity,
            children_recursive,
        };
        DrawDropdownMenuResult::get_state_transitions(dropdown, &entities)
            .map(|entity_state_transitions| commands.get_entity(*entity)
                .map(|mut entity| {
                    info!("Adding state transition: {:?} from dropdown base.", &entity_state_transitions);
                    entity.insert(entity_state_transitions);
                })
            );
    }

    for (entity, dropdown) in build_menu_result.collapsable.iter() {
        let children = child_entities(&entity_query, &with_children_query, *entity);
        let children_entity_recurs = children_recursive( *entity, &entity_query,  &with_children_query, &with_parent_query);
        println!("{:?} are children  of {:?}.", &children, entity);
        let entities = Entities {
            siblings: vec![],
            children: children,
            siblings_children: vec![],
            siblings_children_recursive: vec![],
            parent: vec![],
            self_state: *entity,
            children_recursive: children_entity_recurs,
        };
        DrawCollapsableMenuResult::get_state_transitions(dropdown, &entities)
            .map(|entity_state_transitions| commands.get_entity(*entity)
                .map(|mut entity_commands| {
                    info!("Adding state transition: {:?} from collapsable: {:?}.", &entity_state_transitions, entity);
                    entity_commands.insert(entity_state_transitions);
                })
            );
    }



}

fn get_parent(
    with_parent_query: &Query<ParentQuery<'_>, ParentFilter>,
    entity: Entity,
) -> Vec<Entity> {
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style)| parent.get())
        .ok();

    parent.iter().flat_map(|p| vec![*p])
        .collect()

}

fn sibling_entities(
    entity_query: &Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: &Query<ChildQuery<'_>, ChildFilter>,
    with_parent_query: &Query<ParentQuery<'_>, ParentFilter>,
    entity: Entity,
) -> Vec<Entity> {
    let mut entities = HashSet::new();
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style)| parent.get())
        .ok();

    parent.map(|parent| {
        child_entities(entity_query, with_children_query, parent).iter()
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
    entity_query: &Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: &Query<ChildQuery<'_>, ChildFilter>,
    with_parent_query: &Query<ParentQuery<'_>, ParentFilter>,
    entity: Entity,
) -> Vec<Entity> {
    let mut entities = HashSet::new();
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style)| parent.get())
        .ok();

    parent.map(|parent| {
        sibling_entities(entity_query, with_children_query, with_parent_query, entity)
            .iter()
            .for_each(|sibling| {
                child_entities(entity_query, with_children_query, *sibling).iter()
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
    entity_query: &Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: &Query<ChildQuery<'_>, ChildFilter>,
    with_parent_query: &Query<ParentQuery<'_>, ParentFilter>,
    entity: Entity,
) -> Vec<Entity>{
    let mut entities = HashSet::new();

    let mut siblings_children = siblings_children_entities(entity_query, with_children_query, with_parent_query, entity);
    let mut siblings_children: HashSet<Entity> = siblings_children.into_iter().collect();

    for sibling_child in siblings_children.iter() {
        info!("Adding sibling child: {:?}", sibling_child);
        add_children_recursive(
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
    entity_query: &Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: &Query<ChildQuery<'_>, ChildFilter>,
    with_parent_query: &Query<ParentQuery<'_>, ParentFilter>,
) -> Vec<Entity> {
    let mut to_return = HashSet::new();
    add_children_recursive(&mut to_return, &mut HashSet::new(), entity, entity_query, with_children_query, with_parent_query);
    to_return.into_iter().filter(|e| *e != entity).collect()
}

fn add_children_recursive(
    mut entities_to_return: &mut HashSet<Entity>,
    mut entities_processed: &mut HashSet<Entity>,
    entity: Entity,
    entity_query: &Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: &Query<ChildQuery<'_>, ChildFilter>,
    with_parent_query: &Query<ParentQuery<'_>, ParentFilter>,
) {

    let mut next = HashSet::new();
    entities_to_return.insert(entity);

    child_entities(entity_query, with_children_query, entity)
        .iter()
        .for_each(|child| {
            if !entities_processed.contains(child) {
                entities_to_return.insert(*child);
                entities_processed.insert(*child);
                next.insert(*child);
            }
        });

    for child_entity in next.iter() {
        add_children_recursive(
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
    entity_query: &Query<EntityQuery<'_>, EntityFilter>,
    with_children_query: &Query<ChildQuery<'_>, ChildFilter>,
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


