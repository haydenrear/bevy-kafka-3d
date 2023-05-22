use std::collections::{HashSet};
use std::fmt::Debug;
use bevy::prelude::*;
use crate::event::event_descriptor::EventData;
use crate::event::state_transition::get_state_transitions::{ChildFilter, ChildQuery, Entities, EntityFilter, EntityQuery, GetStateTransitions, ParentFilter, ParentQuery};
use crate::menu::ui_menu_event::next_action::Matches;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{TransitionGroup};
use crate::ui_components::menu_components::{BuildMenuResult};

pub fn insert_state_transitions<
    TransitionGroupT,
    GetStateTransitionsT,
    ResultT: Resource,
    EventDataT: EventData + Debug + 'static,
    StateComponentT: Component,
    FilterMatchesT: Matches<StateComponentT>,
    UpdateComponentT: Component,
    MatchesT: Matches<UpdateComponentT>
>
(
    mut commands: Commands,
    build_menu_result: Res<ResultT>,
    entity_query: Query<EntityQuery<'_, TransitionGroupT>, EntityFilter<TransitionGroupT>>,
    with_children_query: Query<ChildQuery<'_, TransitionGroupT>, ChildFilter<TransitionGroupT>>,
    with_parent_query: Query<ParentQuery<'_, TransitionGroupT>, ParentFilter<TransitionGroupT>>,
)
where
    TransitionGroupT: TransitionGroup + Debug,
    GetStateTransitionsT: GetStateTransitions<TransitionGroupT, ResultT, EventDataT, StateComponentT, FilterMatchesT, UpdateComponentT, MatchesT> + Debug,
{

    for entity in GetStateTransitionsT::get_entities(&build_menu_result).iter() {

        let entities = populate_entities_filtered::<TransitionGroupT, GetStateTransitionsT, ResultT, EventDataT, StateComponentT, FilterMatchesT, UpdateComponentT, MatchesT>(&entity_query, &with_children_query, &with_parent_query, entity);

        GetStateTransitionsT::get_state_transitions(&build_menu_result, &entities)
            .map(|entity_state_transitions| commands.get_entity(*entity)
                .map(|mut entity_commands| {
                    // info!("Adding state transition: {:?} from collapsable: {:?}.", &entity_state_transitions, entity);
                    entity_commands.insert(entity_state_transitions);
                })
            );
    }

}

fn populate_entities_filtered<TransitionGroupT, GetStateTransitionsT, BuildResultT, EventDataT, StateComponentT, FilterMatchesT, UpdateComponentT, MatchesT>(
    entity_query: &Query<EntityQuery<TransitionGroupT>, EntityFilter<TransitionGroupT>>,
    with_children_query: &Query<ChildQuery<TransitionGroupT>, ChildFilter<TransitionGroupT>>,
    with_parent_query: &Query<ParentQuery<TransitionGroupT>, ParentFilter<TransitionGroupT>>,
    entity: &Entity
) -> Entities
    where
        TransitionGroupT: TransitionGroup + Debug,
        GetStateTransitionsT: GetStateTransitions<TransitionGroupT, BuildResultT, EventDataT, StateComponentT, FilterMatchesT, UpdateComponentT, MatchesT> + Debug,
        BuildResultT: Resource,
        EventDataT: EventData + Debug,
        StateComponentT: Component,
        FilterMatchesT: Matches<StateComponentT>,
        UpdateComponentT: Component,
        MatchesT: Matches<UpdateComponentT>
{
    let children = get_filter_components::<TransitionGroupT>(&entity_query, child_entities(&entity_query, &with_children_query, *entity));
    let children_recursive = get_filter_components::<TransitionGroupT>(
        &entity_query,
        children_recursive(*entity, &entity_query, &with_children_query, &with_parent_query)
    );
    let siblings_children_recursive = get_filter_components::<TransitionGroupT>(
        &entity_query,
        siblings_children_recursive(&entity_query, &with_children_query, &with_parent_query, *entity)
    );
    let parent = get_filter_components::<TransitionGroupT>(
        &entity_query,
        get_parent(&with_parent_query, *entity)
    );
    let siblings = get_filter_components::<TransitionGroupT>(
        &entity_query,
        sibling_entities(&entity_query, &with_children_query, &with_parent_query, *entity)
    );
    let siblings_children = get_filter_components::<TransitionGroupT>(
        &entity_query,
        siblings_children_entities(&entity_query, &with_children_query, &with_parent_query, *entity)
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


