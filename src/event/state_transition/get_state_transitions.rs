use std::marker::PhantomData;
use bevy::prelude::*;
use crate::event::event_propagation::Relationship;
use crate::event::event_state::StyleStateChangeEventData;
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::next_action::{DisplayState, SizeState, UiComponentState};
use crate::menu::ui_menu_event::types::{StyleStateChange, UiStateChange, UiStyleComponentStateTransitions};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntitiesStateTypes, EntityComponentStateTransition, StateChangeActionType, UiEntityComponentStateTransitions};
use crate::ui_components::menu_components::BuilderResult;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;

pub trait GetStateTransitions<T: BuilderResult> {

    fn get_state_transitions(builder_result: &T, entities: &Entities) -> Option<UiStyleComponentStateTransitions>;

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

#[derive(Default)]
pub struct Entities {
    pub siblings: Vec<Entity>,
    pub children: Vec<Entity>,
    pub siblings_children: Vec<Entity>,
    pub siblings_children_recursive: Vec<Entity>,
    pub parent: Vec<Entity>,
    pub self_state: Option<Entity>,
    pub children_recursive: Vec<Entity>
}

impl GetStateTransitions<BuildBaseMenuResult> for DrawDropdownMenuResult {
    fn get_state_transitions(
        builder_result: &BuildBaseMenuResult,
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
}

impl GetStateTransitions<DropdownMenuOptionResult> for DrawDropdownMenuResult {
    fn get_state_transitions(
        builder_result: &DropdownMenuOptionResult,
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
}


impl GetStateTransitions<DrawCollapsableMenuResult> for DrawCollapsableMenuResult {
    fn get_state_transitions(
        builder_result: &DrawCollapsableMenuResult,
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
}

impl GetStateTransitions<DropdownMenuOptionResult> for DropdownMenuOptionResult {
    fn get_state_transitions(
        builder_result: &DropdownMenuOptionResult,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {
        let mut transitions = vec![];
        builder_result.selected_checkmark_entity.map(|selected_entity| {
            let transition = EntityComponentStateTransition {
                entity_to_change: EntitiesStateTypes { states:
                    change_entity_component_style(UiChangeTypes::ChangeVisible{value: ()}, selected_entity)
                },
                filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                filter_component: Default::default(),
            };
            transitions.push(transition);
        });
        Some(
            UiEntityComponentStateTransitions {
                transitions,
            }
        )
    }
}
