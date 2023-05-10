use std::marker::PhantomData;
use std::os::macos::raw::stat;
use bevy::prelude::{Button, Changed, Commands, Entity, EventWriter, Interaction, Query, ResMut, Resource, Style, With};
use bevy::hierarchy::{Children, Parent};
use bevy::utils::HashMap;
use bevy::log::info;
use bevy::ecs::query::QueryEntityError;
use bevy::ui::Size;
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_propagation::{ChangePropagation, PropagateComponentEvent, StartingState};
use crate::event::event_actions::RetrieveState;
use crate::event::event_state::StateChange;
use crate::menu::ui_menu_event::change_style::StyleNode;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionType, StyleContext, UiComponent, UiComponentStateTransition, UiComponentStateTransitions, UiEventArgs};
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

#[derive(Default, Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever;

impl RetrieveState<
    UiEventArgs, StateChangeActionType, Style, StyleContext,
    (Entity, &UiComponent, &UiComponentStateTransitions, &Style, &UiIdentifiableComponent),
    (Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style),
    (Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style),
    (With<UiComponent>, With<Style>),
    (With<UiComponent>, With<Parent>, With<Style>),
    (With<UiComponent>, With<Children>, With<Style>),
>
for StateChangeActionTypeStateRetriever
{
    fn create_event(
        mut commands: &mut Commands,
        entity: Entity,
        mut style_context: &mut ResMut<StyleContext>,
        entity_query: &Query<
            (Entity, &UiComponent, &UiComponentStateTransitions, &Style, &UiIdentifiableComponent),
            (With<UiComponent>, With<Style>)
        >,
        with_parent_query: &Query<
            (Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style),
            (With<UiComponent>, With<Parent>, With<Style>)
        >,
        with_children_query: &Query<
            (Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style),
            (With<UiComponent>, With<Children>, With<Style>)
        >
    ) -> Vec<(EventDescriptor<StateChangeActionType, UiEventArgs, Style>,
          Option<PropagateComponentEvent>)>
    {
        let events = entity_query.get(entity.clone())
            .iter()
            .flat_map(|(_, ui_component, state_transitions, style, updateable_value)|
                state_transitions.transitions.iter()
                    .map(move |state_change_action| {
                        info!("Found state change action:\n {:?}\n", state_change_action);
                        (entity, ui_component, style, updateable_value, state_change_action)
                    })
            )
            .flat_map(|(entity, ui_component, style, updateable_value, state_change_action)|
                Self::create_ui_event(&entity_query, &with_parent_query, &with_children_query, entity, style, updateable_value, &state_change_action)
                    .into_iter()
                    .map(|event| (event, None))
            )
            .collect::<Vec<(EventDescriptor<StateChangeActionType, UiEventArgs, Style>, Option<PropagateComponentEvent>)>>();

        events

    }
}

impl StateChangeActionTypeStateRetriever {
    fn create_ui_event(
        entity_query: &Query<
            (Entity, &UiComponent, &UiComponentStateTransitions, &Style, &UiIdentifiableComponent),
            (With<UiComponent>, With<Style>)
        >,
        with_parent_query: &Query<
            (Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style),
            (With<UiComponent>, With<Parent>, With<Style>)
        >,
        with_children_query: &Query<
            (Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style),
            (With<UiComponent>, With<Children>, With<Style>)
        >,
        entity: Entity,
        style: &Style,
        updateable_value: &UiIdentifiableComponent,
        state_transition: &UiComponentStateTransition
    ) -> Vec<EventDescriptor<StateChangeActionType, UiEventArgs, Style>> {

        info!("found state change action: {:?}", &state_transition);

        let mut entities = HashMap::new();

        let propagation = &state_transition.propagation;

        if propagation.includes_parent() {
            parent_entities(&with_children_query, &with_parent_query, entity, &mut entities);
        }

        if propagation.includes_self() {
            let node_style = StyleNode::SelfNode(style.clone().clone(), updateable_value.0);
            entities.insert(entity.clone(), node_style);
        }

        if propagation.includes_children() {
            child_entities(&entity_query, &with_children_query, entity, &mut entities)
        }

        if propagation.includes_sibling() {
            info!("Including siblings.");
            sibling_entities(&with_children_query, &with_parent_query, entity, &mut entities);
        }

        if propagation.includes_siblings_children() {
            info!("Including siblings.");
            siblings_children_entities(&with_children_query, &with_parent_query, entity, &mut entities);
        }

        if let ChangePropagation::CustomPropagation { to, from } = propagation {
            custom_propagation_entities(&entity_query, &with_children_query, &with_parent_query, entity, style, updateable_value, &mut entities, &to, &from);

            info!("Custom propagation event with {:?}.", entities);
        }

        state_transition.state_change.iter()
            .flat_map(|state_change_action_type| {
                match state_change_action_type {
                    StateChangeActionType::Hover(_) => {
                        vec![]
                    }
                    StateChangeActionType::Clicked(clicked) => {
                        clicked.get_ui_event(
                            &entities,
                            state_transition.propagation.get_starting_state().clone(),
                            )
                            .map(|args| {
                                EventDescriptor {
                                    component: PhantomData::default(),
                                    event_data: state_change_action_type.clone(),
                                    event_args: args,
                                }
                            })
                            .map(|arg| vec![arg])
                            .or(Some(vec![]))
                            .unwrap()
                    }
                }
            })
            .collect()

    }
}

fn parent_entities(
    with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>,
    entity: Entity,
    mut entities: &mut HashMap<Entity, StyleNode>
) {
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style)| parent.get())
        .ok();

    parent.map(|parent| {
        with_children_query.get(parent.clone())
            .map(|(_, _, children, update, style)| {
                info!("Found parent with id {}.", update.0);
                let node_style = StyleNode::Parent(style.clone(), update.0);
                entities.insert(parent, node_style);
            })
            .or_else(|_| {
                info!("Failed to fetch parent when parent was included in fetch.");
                Ok::<(), QueryEntityError>(())
            })
            .unwrap()
    })
        .or_else(|| {
            info!("Failed to fetch parent when parent was included in fetch.");
            None
        });
}

fn sibling_entities(
    with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>,
    entity: Entity,
    mut entities: &mut HashMap<Entity, StyleNode>
) {
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style)| parent.get())
        .ok();

    parent.map(|parent| {
        with_children_query.get(parent.clone())
            .map(|(_, _, children, update, style)| {
                info!("Found parent with id {}.", update.0);
                children.iter().for_each(|child| {
                    with_parent_query.get(child.clone())
                        .into_iter()
                        .for_each(|(entity, component, parent, id, style)| {
                            info!("Including sibling: {}.", id.0);
                            entities.insert(entity.clone(), StyleNode::Sibling(style.clone(), id.0));
                        });
                });
            })
            .or_else(|_| {
                info!("Failed to fetch parent when parent was included in fetch.");
                Ok::<(), QueryEntityError>(())
            })
            .unwrap()
    })
        .or_else(|| {
            info!("Failed to fetch parent when parent was included in fetch.");
            None
        });
}

fn siblings_children_entities(
    with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>,
    entity: Entity,
    mut entities: &mut HashMap<Entity, StyleNode>
) {
    let parent = with_parent_query.get(entity.clone())
        .map(|(_, _, parent, updateable, style)| parent.get())
        .ok();

    parent.map(|parent| {
        with_children_query.get(parent.clone())
            .map(|(_, _, children, update, style)| {
                info!("Found parent with id {}.", update.0);
                children.iter().for_each(|child| {
                    with_parent_query.get(child.clone())
                        .into_iter()
                        .filter(|(this_entity, _, _, _, _)| entity != *this_entity)
                        .for_each(|(sibling, _, _, _, _)| {
                            with_children_query.get(sibling)
                                .into_iter()
                                .for_each(|(_, _, children, update, style)| {
                                    children.iter().for_each(|child| {
                                        entities.insert(child.clone(), StyleNode::SiblingChild(style.clone(), update.0));
                                    });
                                });
                            info!("Including sibling: {}.", update.0);
                        });
                });
            })
            .or_else(|_| {
                info!("Failed to fetch parent when parent was included in fetch.");
                Ok::<(), QueryEntityError>(())
            })
            .unwrap()
    })
        .or_else(|| {
            info!("Failed to fetch parent when parent was included in fetch.");
            None
        });
}

fn child_entities(
    entity_query: &Query<
        (Entity, &UiComponent, &UiComponentStateTransitions, &Style, &UiIdentifiableComponent),
        (With<UiComponent>, With<Style>)
    >,
    with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    entity: Entity,
    mut entities: &mut HashMap<Entity, StyleNode>
) {

    info!("Including children.");

    let child_entities = with_children_query.get(entity.clone())
        .map(|(_, _, child, update, _)| child.iter()
            .map(|child| child.clone()).collect::<Vec<Entity>>()
        )
        .ok()
        .or(Some(vec![]))
        .unwrap();

    child_entities.iter().for_each(|child| {
        info!("Fetching child entity: {:?}.", child);
        let _ = entity_query.get(child.clone())
            .map(|entity| {
                let node_style = StyleNode::Child(entity.3.clone(), entity.4.0);
                entities.insert(entity.0.clone(), node_style);
            })
            .or_else(|_| {
                info!("Error fetching query for child.");
                Ok::<(), QueryEntityError>(())
            });
    });

    info!("{:?} are the entities after adding child.", entities);
}


fn custom_propagation_entities(
    entity_query: &Query<
        (Entity, &UiComponent, &UiComponentStateTransitions, &Style, &UiIdentifiableComponent),
        (With<UiComponent>, With<Style>)
    >,
    with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>,
    entity: Entity,
    style: &Style,
    updateable_value: &UiIdentifiableComponent,
    mut entities: &mut HashMap<Entity, StyleNode>,
    to: &Vec<f32>,
    from: &StartingState
) {
    if let StartingState::SelfState = from {
        entities.insert(entity.clone(), StyleNode::SelfNode(style.clone(), updateable_value.0));
    }

    info!("{:?} are the to.", to);

    with_children_query.get(entity)
        .iter()
        .flat_map(|found| {
            found.2.iter().map(|child| with_parent_query.get(child.clone()))
        })
        .map(|ok| ok.ok())
        .flat_map(|ok| ok.map(|value| vec![value]).or(Some(vec![])).unwrap())
        .filter(|found| {
            to.contains(&found.3.0)
        })
        .for_each(|found| {
            entities.insert(entity, StyleNode::Child(found.4.clone(), found.3.0));
        });

    with_parent_query.get(entity)
        .iter()
        .flat_map(|with_parent| entity_query.get(with_parent.2.get()))
        .filter(|found| {
            to.contains(&found.4.0)
        })
        .for_each(|found| {
            entities.insert(entity, StyleNode::Parent(found.3.clone(), found.4.0));
        });

    entity_query.iter()
        .filter(|(_, _, _, _,updateable )| {
            info!("Checking {}", &updateable.0);
            to.contains(&updateable.0)
        })
        .for_each(|(entity, component, state_transition, style, updateable)| {
            entities.insert(entity, StyleNode::Other(style.clone(), updateable.0));
        });
}

