use std::collections::{HashMap, HashSet};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy::tasks::ParallelSlice;
use crate::event::event_propagation::Relationship;
use crate::event::state_transition::get_state_transitions::{Entities, GetStateTransitions};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{PropagateDisplay, TransitionGroup, UiComponentStateTransitions};
use crate::menu::UiComponent;
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

type EntityQuery<'a, TransitionGroupT> = (Entity, &'a UiComponent, &'a Style, &'a UiIdentifiableComponent, &'a TransitionGroupT);
type EntityFilter<TransitionGroupT> = (With<UiComponent>, With<Style>, With<TransitionGroupT>);
type ChildQuery<'a, TransitionGroupT> = (Entity, &'a UiComponent, &'a Children, &'a UiIdentifiableComponent, &'a Style, &'a TransitionGroupT);
type ChildFilter<TransitionGroupT> = (With<UiComponent>, With<Children>, With<Style>, With<TransitionGroupT>);
type ParentQuery<'a, TransitionGroupT> = (Entity, &'a UiComponent, &'a Parent, &'a UiIdentifiableComponent, &'a Style, &'a TransitionGroupT);
type ParentFilter<TransitionGroupT> = (With<UiComponent>, With<Parent>, With<Style>, With<TransitionGroupT>);

pub fn insert_state_transitions<TransitionGroupT: TransitionGroup>(
    mut commands: Commands,
    build_menu_result: Res<BuildMenuResult>,
    entity_query: Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
    with_children_query: Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
    with_parent_query: Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
) {
    add_dropdown_state_transitions(&mut commands, &build_menu_result, &entity_query, &with_children_query, &with_parent_query);

    add_collapsable_state_transitions(&mut commands, &build_menu_result, &entity_query, &with_children_query, &with_parent_query);

    add_dropdown_menu_options_transitions(commands, build_menu_result);

}

fn add_dropdown_menu_options_transitions(mut commands: Commands, build_menu_result: Res<BuildMenuResult>) {
    for (entity, dropdown) in build_menu_result.dropdown_menu_option_results.iter() {
        let entities = Entities {
            self_state: Some(*entity),
            ..default()
        };
        DropdownMenuOptionResult::get_state_transitions(dropdown, &entities)
            .map(|entity_state_transitions| commands.get_entity(*entity)
                .map(|mut entity_commands| {
                    info!("Adding state transition: {:?} from collapsable: {:?}.", &entity_state_transitions, entity);
                    entity_commands.insert(entity_state_transitions);
                })
            );
    }
}

fn add_dropdown_state_transitions<TransitionGroupT: TransitionGroup>(
    mut commands: &mut Commands,
    build_menu_result: &Res<BuildMenuResult>,
    entity_query: &Query<EntityQuery<TransitionGroupT>, EntityFilter<TransitionGroupT>>,
    with_children_query: &Query<ChildQuery<TransitionGroupT>, ChildFilter<TransitionGroupT>>,
    with_parent_query: &Query<ParentQuery<TransitionGroupT>, ParentFilter<TransitionGroupT>>
) {
    for (entity, dropdown) in build_menu_result.base_menu_results.iter() {
        let children = get_filter_components::<TransitionGroupT>(&entity_query, child_entities(&entity_query, &with_children_query, *entity));
        let children_recursive = get_filter_components::<TransitionGroupT>(
            &entity_query,
            children_recursive(*entity, &entity_query, &with_children_query, &with_parent_query));
        let siblings_children_recursive = get_filter_components::<TransitionGroupT>(
            &entity_query,
            siblings_children_recursive(&entity_query, &with_children_query, &with_parent_query, *entity));
        let entities = Entities {
            siblings: vec![],
            siblings_children: vec![],
            children,
            siblings_children_recursive,
            parent: vec![],
            self_state: Some(*entity),
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
}

fn add_collapsable_state_transitions<TransitionGroupT: TransitionGroup>(
    mut commands: &mut Commands,
    build_menu_result: &Res<BuildMenuResult>,
    entity_query: &Query<EntityQuery<TransitionGroupT>, EntityFilter<TransitionGroupT>>,
    with_children_query: &Query<ChildQuery<TransitionGroupT>, ChildFilter<TransitionGroupT>>,
    with_parent_query: &Query<ParentQuery<TransitionGroupT>, ParentFilter<TransitionGroupT>>
) {
    for (entity, dropdown) in build_menu_result.collapsable.iter() {
        let children = get_filter_components::<TransitionGroupT>(&entity_query, child_entities(&entity_query, &with_children_query, *entity));
        let children_recursive = get_filter_components::<TransitionGroupT>(&entity_query, children_recursive(*entity, &entity_query, &with_children_query, &with_parent_query));
        println!("{:?} are children  of {:?}.", &children, entity);
        let entities = Entities {
            siblings: vec![],
            children,
            siblings_children: vec![],
            siblings_children_recursive: vec![],
            parent: vec![],
            self_state: Some(*entity),
            children_recursive,
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

fn get_filter_components<T: TransitionGroup>(entity_query: &Query<EntityQuery<T>, EntityFilter<T>>, entities: Vec<Entity>) -> Vec<Entity> {
    let children = entities
        .into_iter()
        .filter(|child| {
            entity_query.get_component::<T>(*child).or_else(|e| {
                error!("Error: {:?}", e);
                Err(e)
            }).is_ok()
        })
        .collect();
    children
}

fn get_parent<T: TransitionGroup>(
    with_parent_query: &Query<ParentQuery<'_, T>, ParentFilter<T>>,
    entity: Entity,
) -> Vec<Entity> {
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style, _)| parent.get())
        .ok();

    parent.iter().flat_map(|p| vec![*p])
        .collect()

}

fn sibling_entities<T: TransitionGroup>(
    entity_query: &Query<EntityQuery<'_, T>, EntityFilter<T>>,
    with_children_query: &Query<ChildQuery<'_, T>, ChildFilter<T>>,
    with_parent_query: &Query<ParentQuery<'_, T>, ParentFilter<T>>,
    entity: Entity,
) -> Vec<Entity> {
    let mut entities = HashSet::new();
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style, _)| parent.get())
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

fn siblings_children_entities<T: TransitionGroup>(
    entity_query: &Query<EntityQuery<'_, T>, EntityFilter<T>>,
    with_children_query: &Query<ChildQuery<'_, T>, ChildFilter<T>>,
    with_parent_query: &Query<ParentQuery<'_, T>, ParentFilter<T>>,
    entity: Entity,
) -> Vec<Entity> {
    let mut entities = HashSet::new();
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style, _)| parent.get())
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

fn siblings_children_recursive<T: TransitionGroup>(
    entity_query: &Query<EntityQuery<'_, T>, EntityFilter<T>>,
    with_children_query: &Query<ChildQuery<'_, T>, ChildFilter<T>>,
    with_parent_query: &Query<ParentQuery<'_, T>, ParentFilter<T>>,
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

fn children_recursive<T: TransitionGroup>(
    entity: Entity,
    entity_query: &Query<EntityQuery<'_, T>, EntityFilter<T>>,
    with_children_query: &Query<ChildQuery<'_, T>, ChildFilter<T>>,
    with_parent_query: &Query<ParentQuery<'_, T>, ParentFilter<T>>,
) -> Vec<Entity> {
    let mut to_return = HashSet::new();
    add_children_recursive(&mut to_return, &mut HashSet::new(), entity, entity_query, with_children_query, with_parent_query);
    to_return.into_iter().filter(|e| *e != entity).collect()
}

fn add_children_recursive<T: TransitionGroup>(
    mut entities_to_return: &mut HashSet<Entity>,
    mut entities_processed: &mut HashSet<Entity>,
    entity: Entity,
    entity_query: &Query<EntityQuery<'_, T>, EntityFilter<T>>,
    with_children_query: &Query<ChildQuery<'_, T>, ChildFilter<T>>,
    with_parent_query: &Query<ParentQuery<'_, T>, ParentFilter<T>>,
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

fn child_entities<T: TransitionGroup>(
    entity_query: &Query<EntityQuery<'_, T>, EntityFilter<T>>,
    with_children_query: &Query<ChildQuery<'_, T>, ChildFilter<T>>,
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


