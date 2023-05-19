use std::collections::HashSet;
use std::marker::PhantomData;
use bevy::prelude::*;
use crate::event::event_propagation::Relationship;
use crate::event::event_state::StyleStateChangeEventData;
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::next_action::{DisplayState, SizeState, UiComponentState};
use crate::menu::ui_menu_event::types::{ComponentStateTransitions, SelectOptionsStateTransitions, StyleStateChange, UiSelectedComponentStateTransition, UiSelectedComponentStateTransitions, UiStateChange, UiStyleComponentStateTransitions};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntitiesStateTypes, EntityComponentStateTransition, PropagateDisplay, SelectOptions, StateChangeActionType, TransitionGroup, UiEntityComponentStateTransitions};
use crate::menu::UiComponent;
use crate::ui_components::menu_components::{BuildMenuResult};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub(crate) type EntityQuery<'a, TransitionGroupT> = (Entity, &'a UiComponent, &'a Style, &'a UiIdentifiableComponent, &'a TransitionGroupT);
pub(crate) type EntityFilter<TransitionGroupT> = (With<UiComponent>, With<Style>, With<TransitionGroupT>);
pub(crate) type ChildQuery<'a, TransitionGroupT> = (Entity, &'a UiComponent, &'a Children, &'a UiIdentifiableComponent, &'a Style, &'a TransitionGroupT);
pub(crate) type ChildFilter<TransitionGroupT> = (With<UiComponent>, With<Children>, With<Style>, With<TransitionGroupT>);
pub(crate) type ParentQuery<'a, TransitionGroupT> = (Entity, &'a UiComponent, &'a Parent, &'a UiIdentifiableComponent, &'a Style, &'a TransitionGroupT);
pub(crate) type ParentFilter<TransitionGroupT> = (With<UiComponent>, With<Parent>, With<Style>, With<TransitionGroupT>);


pub trait GetStateTransitions<TransitionGroupT: TransitionGroup> {

    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        entities: &Entities
    ) -> Option<ComponentStateTransitions<TransitionGroupT>>;

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity>;

}

fn change_child(style_type: UiChangeTypes, entities: &Vec<Entity>) -> Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)> {
    let mut change_visisble = entities
        .iter()
        .map(|e| {
            info!("Adding child for change visible: {:?}, {:?}.", e, &style_type);
            e
        })
        .map(|entity| (
            *entity,
            Relationship::Child,
            StateChangeActionType::Clicked{
                value: StyleStateChangeEventData::ChangeComponentStyle(style_type.clone()),
                p: PhantomData::default(),
                p1: PhantomData::default(),
                p2: PhantomData::default()
            }
        ))
        .collect::<Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)>>();
    change_visisble
}

fn change_entity_component_style(
    style_type: UiChangeTypes, entity: Entity
) -> Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)> {
    vec![(
        entity,
        Relationship::SelfState,
        StateChangeActionType::Clicked{
            value: StyleStateChangeEventData::ChangeComponentStyle(style_type.clone()),
            p: PhantomData::default(),
            p1: PhantomData::default(),
            p2: PhantomData::default()
        }
    )]
}

#[derive(Default, Debug)]
pub struct Entities {
    pub siblings: Vec<Entity>,
    pub children: Vec<Entity>,
    pub siblings_children: Vec<Entity>,
    pub siblings_children_recursive: Vec<Entity>,
    pub parent: Vec<Entity>,
    pub self_state: Option<Entity>,
    pub children_recursive: Vec<Entity>
}

impl GetStateTransitions<PropagateDisplay> for BuildBaseMenuResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {
        info!("building state transitions.");


        let remove_visible: Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)>
            = change_child(UiChangeTypes::RemoveVisible{value: ()}, &build_menu_result.children_recursive);
        let change_visible = change_child(UiChangeTypes::ChangeVisible{value: ()}, &build_menu_result.children);

        let mut siblings: Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)> = build_menu_result.siblings_children_recursive
            .iter()
            .map(|entity| (
                *entity,
                Relationship::SiblingChild,
                StateChangeActionType::Clicked {
                    value:StyleStateChangeEventData::ChangeComponentStyle(UiChangeTypes::RemoveVisible{value: {}}),
                    p: PhantomData::default(),
                    p1: PhantomData::default(),
                    p2: PhantomData::default()
                }
            ))
            .collect();

        info!("{:?} are the sibling recursive.", &siblings);

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: change_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: siblings
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                    },
                ],
            }
        )
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result.base_menu_results.keys()
            .map(|&e| e)
            .into_iter().collect()
    }
}



impl GetStateTransitions<PropagateDisplay> for DrawDropdownMenuResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {

        let remove_visible = change_child(UiChangeTypes::RemoveVisible{value: ()}, &build_menu_result.children_recursive);
        let change_visible = change_child(UiChangeTypes::ChangeVisible{value: ()}, &build_menu_result.children);

        let mut siblings: Vec<(Entity, Relationship, StyleStateChange)> = build_menu_result.siblings_children_recursive
            .iter()
            .map(|entity| (
                *entity,
                Relationship::SiblingChild,
                StateChangeActionType::Clicked{
                    value: StyleStateChangeEventData::ChangeComponentStyle(UiChangeTypes::RemoveVisible{value: ()}),
                    p: PhantomData::default(),
                    p1: PhantomData::default(),
                    p2: PhantomData::default()
                }
            ))
            .collect();

        info!("{:?} are the sibling recursive.", &siblings);

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: change_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: siblings
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                    },
                ],
            }
        )
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result.dropdown_menu_option_results.keys()
            .map(|&e| e)
            .into_iter().collect()
    }
}

impl GetStateTransitions<SelectOptions> for DropdownMenuOptionResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiSelectedComponentStateTransitions> {
        info!("Getting state transitions for select options: {:?}.", build_menu_result);
        let transitions = build_menu_result.self_state
            .iter()
            .flat_map(|selected_entity| {
                if build_menu_result.children.len() != 0 {
                    info!("Adding selected to {:?}.", selected_entity);
                    return vec![EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: change_child(UiChangeTypes::ChangeVisible { value: () },
                                            &build_menu_result.children)
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                    }];
                }
                vec![]
            })
            .collect::<Vec<UiSelectedComponentStateTransition>>();
        Some(
            UiSelectedComponentStateTransitions {
                transitions,
            }
        )
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result
            .dropdown_menu_option_results
            .keys()
            .map(|&e| e)
            .collect()
    }
}

impl GetStateTransitions<PropagateDisplay> for DrawCollapsableMenuResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {

        if build_menu_result.self_state.is_none() {
            panic!("Build menu did not have entity associated in build menu result.");
        }

        let mut add_visible = change_child(UiChangeTypes::AddVisible{value: ()}, &build_menu_result.children);

        let mut remove_visible_recurs = change_child(UiChangeTypes::RemoveVisible{value: ()}, &build_menu_result.children_recursive);

        let mut self_change_minimize = change_entity_component_style(UiChangeTypes::UpdateSize {
            value: (
                100.0, 20.0
            )
        }, build_menu_result.self_state.unwrap());

        let mut self_change_maximize = change_entity_component_style(UiChangeTypes::UpdateSize {
            value: (
                100.0, 4.0
            )
        }, build_menu_result.self_state.unwrap());

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Expanded{
                            height: 100,
                            width: 20
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible_recurs,
                        },
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: add_visible,
                        },
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayNone),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Minimized{
                            height: 100,
                            width: 4
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: self_change_minimize
                        },
                        current_state_filter: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4
                        }),
                        filter_component: Default::default(),
                    },
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: self_change_maximize,
                        },
                        current_state_filter: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20
                        }),
                        filter_component: Default::default(),
                    },
                ],
            }
        )
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result.collapsable.keys()
            .into_iter()
            .map(|&e| e)
            .collect()
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


