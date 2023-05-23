use std::marker::PhantomData;
use bevy::prelude::*;
use crate::event::event_descriptor::EventData;
use crate::event::event_propagation::Relationship;
use crate::event::event_state::{ComponentChangeEventData, StyleStateChangeEventData};
use crate::graph::{GraphConfigurationResource, GraphingMetricsResource};
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::next_action::{DisplayState, Matches, SizeState, UiComponentState, VisibilityIdentifier};
use crate::menu::ui_menu_event::type_alias::state_change_action_retriever::{StyleStateChange, UiStateChange};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::graphing_menu::graph_menu::{ChangeGraphingMenu, GraphMenuPotential};
use crate::menu::ui_menu_event::entity_component_state_transition::{EntityComponentStateTransition, UiEntityComponentStateTransitions};
use crate::menu::ui_menu_event::state_change_factory::{EntitiesStateTypes, StateChangeActionType};
use crate::menu::ui_menu_event::transition_groups::{PropagateCreateMenu, PropagateDisplay, PropagateDraggable, PropagateSelect, PropagateVisible, TransitionGroup};
use crate::menu::ui_menu_event::type_alias::state_transitions::{ChangeMenuStateTransitions, ComponentStateTransitions, DraggableStateTransitions, UiSelectedComponentStateTransition, UiSelectedComponentStateTransitions, UiStyleComponentStateTransitions, VisibilityStateTransitions};
use crate::pickable_events::{ComponentSpawned, PickableComponentState};
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_options::slider_menu_option::SliderMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::DrawDropdownMenuResult;

#[derive(Default, Debug)]
pub struct Entities {
    pub siblings: Vec<Entity>,
    pub children: Vec<Entity>,
    pub siblings_children: Vec<Entity>,
    pub siblings_children_recursive: Vec<Entity>,
    pub parent: Vec<Entity>,
    pub self_state: Option<Entity>,
    pub children_recursive: Vec<Entity>,
}

pub trait GetStateTransitions<
    TransitionGroupT: TransitionGroup,
    ResultT: Resource,
    EventDataT: EventData,
    StateComponentT: Component,
    FilterMatchesT: Matches<StateComponentT>,
    UpdateComponentT: Component = StateComponentT,
    MatchesT: Matches<UpdateComponentT> = FilterMatchesT
> {

    fn get_state_transitions(
        builder_result: &Res<ResultT>,
        entities: &Entities,
    ) -> Option<ComponentStateTransitions<TransitionGroupT, EventDataT, StateComponentT, FilterMatchesT, UpdateComponentT, MatchesT>>;

    fn get_entities(builder_result: &Res<ResultT>) -> Vec<Entity>;

}

impl GetStateTransitions<
    PropagateCreateMenu,
    GraphingMetricsResource,
    ComponentChangeEventData,
    GraphMenuPotential,
    PickableComponentState,
    ChangeGraphingMenu
> for GraphingMetricsResource {
    fn get_state_transitions(
        builder_result: &Res<GraphingMetricsResource>,
        entities: &Entities
    ) -> Option<ChangeMenuStateTransitions> {
        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: vec![(
                                entities.self_state.unwrap(),
                                Relationship::SelfState,
                                StateChangeActionType::Clicked {
                                    value: ComponentChangeEventData::ChangeGraphingMenu,
                                    p: PhantomData::default(),
                                    p1: PhantomData::default(),
                                    p2: PhantomData::default(),
                                }
                            )]
                        },
                        filter_state: PickableComponentState::Spawned(ComponentSpawned::Any),
                        current_state_filter: PickableComponentState::Spawned(ComponentSpawned::Any),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    }
                ],
                state_component: Default::default(),
            }
        )
    }

    fn get_entities(builder_result: &Res<GraphingMetricsResource>) -> Vec<Entity> {
        builder_result.metric_indices.values()
            .flat_map(|v| v)
            .map(|&e| e)
            .collect()
    }
}

impl GetStateTransitions<
    PropagateVisible,
    NetworkMenuResultBuilder,
    ComponentChangeEventData,
    MetricsConfigurationOption<Menu>,
    UiComponentState,
    Visibility
> for NetworkMenuResultBuilder {
    fn get_state_transitions(
        builder_result: &Res<NetworkMenuResultBuilder>,
        build_menu_result: &Entities,
    ) -> Option<VisibilityStateTransitions> {


        let change_visible
            = change_entity_component(
            ComponentChangeEventData::ChangeVisible {
                to_change: builder_result.network_parent_entity.unwrap(),
                adviser_component: builder_result.network_menu_config_option.unwrap(),
            },
            builder_result.network_menu_config_option.unwrap(),
        );

        info!("Inserting change visible state transition: {:?}.", change_visible);

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: change_visible
                        },
                        filter_state: UiComponentState::StateVisible(VisibilityIdentifier::Any),
                        current_state_filter: UiComponentState::StateVisible(VisibilityIdentifier::Any),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    }
                ],
                state_component: Default::default(),
            }
        )
    }

    fn get_entities(builder_result: &Res<NetworkMenuResultBuilder>) -> Vec<Entity> {
        builder_result.network_menu_config_option
            .map(|e| vec![e])
            .or(Some(vec![]))
            .unwrap()
    }
}

impl GetStateTransitions<
    PropagateVisible,
    GraphMenuResultBuilder,
    ComponentChangeEventData,
    MetricsConfigurationOption<Menu>,
    UiComponentState,
    Visibility
> for GraphMenuResultBuilder {
    fn get_state_transitions(
        builder_result: &Res<GraphMenuResultBuilder>,
        build_menu_result: &Entities,
    ) -> Option<VisibilityStateTransitions> {
        info!("building state transitions.");


        let change_visible
            = change_entity_component(
            ComponentChangeEventData::ChangeVisible {
                to_change: builder_result.graph_parent_entity.unwrap(),
                adviser_component: builder_result.graph_menu_config_option.unwrap(),
            },
            builder_result.graph_menu_config_option.unwrap(),
        );

        info!("Inserting change visible state transition: {:?}.", change_visible);

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: change_visible
                        },
                        filter_state: UiComponentState::StateVisible(VisibilityIdentifier::Any),
                        current_state_filter: UiComponentState::StateVisible(VisibilityIdentifier::Any),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    }
                ],
                state_component: Default::default(),
            }
        )
    }

    fn get_entities(builder_result: &Res<GraphMenuResultBuilder>) -> Vec<Entity> {
        builder_result.graph_menu_config_option
            .map(|e| vec![e])
            .or(Some(vec![]))
            .unwrap()
    }
}


impl GetStateTransitions<
    PropagateDisplay,
    BuildMenuResult,
    StyleStateChangeEventData,
    Style,
    UiComponentState
> for BuildBaseMenuResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {
        info!("building state transitions.");


        let remove_visible: Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)>
            = change_child(UiChangeTypes::RemoveVisible { value: () }, &build_menu_result.children_recursive);
        let change_visible
            = change_child(UiChangeTypes::ChangeVisible { value: () }, &build_menu_result.children);

        let mut siblings: Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)> = build_menu_result.siblings_children_recursive
            .iter()
            .map(|entity| (
                *entity,
                Relationship::SiblingChild,
                StateChangeActionType::Clicked {
                    value: StyleStateChangeEventData::ChangeComponentStyle(UiChangeTypes::RemoveVisible { value: {} }),
                    p: PhantomData::default(),
                    p1: PhantomData::default(),
                    p2: PhantomData::default(),
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
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: siblings
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                ],
                state_component: Default::default(),
            }
        )
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result.base_menu_results.keys()
            .map(|&e| e)
            .into_iter().collect()
    }
}


impl GetStateTransitions<PropagateDisplay, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState> for DrawDropdownMenuResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {
        let remove_visible = change_child(UiChangeTypes::RemoveVisible { value: () }, &build_menu_result.children_recursive);
        let change_visible = change_child(UiChangeTypes::ChangeVisible { value: () }, &build_menu_result.children);

        let mut siblings: Vec<(Entity, Relationship, StyleStateChange)> = build_menu_result.siblings_children_recursive
            .iter()
            .map(|entity| (
                *entity,
                Relationship::SiblingChild,
                StateChangeActionType::Clicked {
                    value: StyleStateChangeEventData::ChangeComponentStyle(UiChangeTypes::RemoveVisible { value: () }),
                    p: PhantomData::default(),
                    p1: PhantomData::default(),
                    p2: PhantomData::default(),
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
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: siblings
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                ],
                state_component: Default::default(),
            }
        )
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result.dropdown_menu_option_results.keys()
            .map(|&e| e)
            .into_iter().collect()
    }
}

impl GetStateTransitions<PropagateSelect, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState> for DropdownMenuOptionResult {
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
                        state_component: Default::default(),
                    }];
                }
                vec![]
            })
            .collect::<Vec<UiSelectedComponentStateTransition>>();
        Some(
            UiSelectedComponentStateTransitions {
                transitions,
                state_component: Default::default(),
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

impl GetStateTransitions<PropagateDisplay, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState> for DrawCollapsableMenuResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<UiStyleComponentStateTransitions> {
        if build_menu_result.self_state.is_none() {
            panic!("Build menu did not have entity associated in build menu result.");
        }

        let mut add_visible = change_child(UiChangeTypes::AddVisible { value: () }, &build_menu_result.children);

        let mut remove_visible_recurs = change_child(UiChangeTypes::RemoveVisible { value: () }, &build_menu_result.children_recursive);

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
                    EntityComponentStateTransition {
                        filter_state: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20,
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible_recurs,
                        },
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        filter_state: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4,
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: add_visible,
                        },
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayNone),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        filter_state: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4,
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: self_change_minimize
                        },
                        current_state_filter: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4,
                        }),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                    EntityComponentStateTransition {
                        filter_state: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20,
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: self_change_maximize,
                        },
                        current_state_filter: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20,
                        }),
                        filter_component: Default::default(),
                        state_component: Default::default(),
                    },
                ],
                state_component: Default::default(),
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

impl GetStateTransitions<PropagateDraggable, BuildMenuResult, StyleStateChangeEventData, Style, UiComponentState> for SliderMenuOptionResult {
    fn get_state_transitions(
        builder_result: &Res<BuildMenuResult>,
        build_menu_result: &Entities,
    ) -> Option<DraggableStateTransitions> {

        builder_result.slider.get(&build_menu_result.self_state.unwrap())
            .map(|slider| slider.slider_knob_entity)
            .map(|slider_knob| {
                UiEntityComponentStateTransitions {
                    transitions: vec![
                        EntityComponentStateTransition {
                            entity_to_change: EntitiesStateTypes {
                                states: drag_self_component(
                                    UiChangeTypes::DragXPosition { value: (), },
                                    build_menu_result.self_state.unwrap()
                                )
                            },
                            filter_state: UiComponentState::Any,
                            current_state_filter: UiComponentState::Any,
                            filter_component: Default::default(),
                            state_component: Default::default(),
                        }
                    ],
                    state_component: Default::default(),
                }
            })
    }

    fn get_entities(builder_result: &Res<BuildMenuResult>) -> Vec<Entity> {
        builder_result.slider.keys()
            .into_iter()
            .map(|&e| e)
            .collect()
    }
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
            StateChangeActionType::Clicked {
                value: StyleStateChangeEventData::ChangeComponentStyle(style_type.clone()),
                p: PhantomData::default(),
                p1: PhantomData::default(),
                p2: PhantomData::default(),
            }
        ))
        .collect::<Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)>>();
    change_visisble
}

fn change_entity_component_style(
    style_type: UiChangeTypes, entity: Entity,
) -> Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)> {
    vec![(
        entity,
        Relationship::SelfState,
        StateChangeActionType::Clicked {
            value: StyleStateChangeEventData::ChangeComponentStyle(style_type.clone()),
            p: PhantomData::default(),
            p1: PhantomData::default(),
            p2: PhantomData::default(),
        }
    )]
}

fn change_entity_component(
    style_type: ComponentChangeEventData,
    entity: Entity,
) -> Vec<(Entity, Relationship, UiStateChange<Visibility, ComponentChangeEventData>)> {
    vec![(
        entity,
        Relationship::SelfState,
        StateChangeActionType::Clicked {
            value: style_type,
            p: PhantomData::default(),
            p1: PhantomData::default(),
            p2: PhantomData::default(),
        }
    )]
}

fn drag_self_component(
    style_type: UiChangeTypes, entity: Entity,
) -> Vec<(Entity, Relationship, UiStateChange<Style, StyleStateChangeEventData>)> {
    vec![(
        entity,
        Relationship::SelfState,
        StateChangeActionType::Dragged {
            value: StyleStateChangeEventData::ChangeComponentStyle(style_type.clone()),
            p: PhantomData::default(),
            p1: PhantomData::default(),
            p2: PhantomData::default(),
        }
    )]
}

