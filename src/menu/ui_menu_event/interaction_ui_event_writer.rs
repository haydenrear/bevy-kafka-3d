use std::marker::PhantomData;
use std::os::macos::raw::stat;
use bevy::prelude::{Button, Changed, Commands, Entity, EventWriter, Interaction, Query, Resource, Style, With};
use bevy::hierarchy::{Children, Parent};
use bevy::utils::HashMap;
use bevy::log::info;
use bevy::ecs::query::QueryEntityError;
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_propagation::{ChangePropagation, StartingState};
use crate::event::event_actions::RetrieveState;
use crate::menu::ui_menu_event::change_style::StyleNode;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionType, UiComponent, UiEventArgs};
use crate::visualization::UiIdentifiableComponent;

#[derive(Default, Resource, Debug)]
pub struct StateChangeActionTypeStateRetriever;

impl RetrieveState<
    UiEventArgs, StateChangeActionType, Style,
    (Entity, &UiComponent, &Style, &UiIdentifiableComponent),
    (Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style),
    (Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style),
    (With<UiComponent>, With<Style>),
    (With<UiComponent>, With<Parent>, With<Style>),
    (With<UiComponent>, With<Children>, With<Style>),
>
for StateChangeActionTypeStateRetriever
{
    fn retrieve_state(
        mut commands: &mut Commands,
        entity: Entity,
        entity_query: &Query<
            (Entity, &UiComponent, &Style, &UiIdentifiableComponent),
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
    ) -> Option<EventDescriptor<StateChangeActionType, UiEventArgs, Style>> {

        entity_query.get(entity.clone())
            .iter()
            .flat_map(|(_, ui_component, style, updateable_value)|
                ui_component.get_state_change_types().iter()
                    .map(move |state_change_action| (entity, ui_component, style, updateable_value, state_change_action))
            )
            .flat_map(|(entity, ui_component, style, updateable_value, state_change_action)|
                Self::create_ui_event(&entity_query, &with_parent_query, &with_children_query, entity, style, updateable_value, &state_change_action)
            )
            .next()
    }
}

impl StateChangeActionTypeStateRetriever {
    fn create_ui_event(
        entity_query: &Query<(Entity, &UiComponent, &Style, &UiIdentifiableComponent), (With<UiComponent>, With<Style>)>,
        with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>,
        with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
        entity: Entity,
        style: &Style,
        updateable_value: &UiIdentifiableComponent,
        state_change_action: &StateChangeActionType
    ) -> Option<EventDescriptor<StateChangeActionType, UiEventArgs, Style>> {
        info!("found state change action: {:?}", state_change_action);
        let clicked = &state_change_action.clicked;

        let style_change_types = clicked.propagation();
        let mut entities = HashMap::new();

        if style_change_types
            .filter(|propagation| propagation.includes_parent())
            .is_some() {
            parent_entities(&with_children_query, &with_parent_query, entity, &mut entities);
        }

        if style_change_types
            .filter(|any| any.includes_self())
            .is_some() {
            let node_style = StyleNode::SelfNode(style.clone().clone(), updateable_value.0);
            entities.insert(entity.clone(), node_style);
        }

        if style_change_types.iter()
            .any(|any| any.includes_children()) {
            child_entities(&entity_query, &with_children_query, entity, &mut entities)
        }

        if let Some(ChangePropagation::CustomPropagation { to, from }) = style_change_types {
            custom_propagation_entities(&entity_query, &with_children_query, &with_parent_query, entity, style, updateable_value, &mut entities, to, from);

            info!("Custom propagation event with {:?}.", entities);
        }

        info!("propagation event with {:?}.", entities);
        return clicked.get_ui_event(entities)
            .map(|args| {
                EventDescriptor {
                    component: PhantomData::default(),
                    event_data: state_change_action.clone(),
                    event_args: args,
                }
            });
    }
}

fn parent_entities(with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>, with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>, entity: Entity, mut entities: &mut HashMap<Entity, StyleNode>) {
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

fn child_entities(entity_query: &Query<(Entity, &UiComponent, &Style, &UiIdentifiableComponent), (With<UiComponent>, With<Style>)>, with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>, entity: Entity, mut entities: &mut HashMap<Entity, StyleNode>) {

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
                let node_style = StyleNode::Child(entity.2.clone(), entity.3.0);
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
    entity_query: &Query<(Entity, &UiComponent, &Style, &UiIdentifiableComponent), (With<UiComponent>, With<Style>)>,
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
            info!("Checking {}", &found.3.0);
            to.contains(&found.3.0)
        })
        .for_each(|found| {
            entities.insert(entity, StyleNode::Child(found.4.clone(), found.3.0));
        });

    with_parent_query.get(entity)
        .iter()
        .flat_map(|with_parent| entity_query.get(with_parent.2.get()))
        .filter(|found| {
            info!("Checking {}", &found.3.0);
            to.contains(&found.3.0)
        })
        .for_each(|found| {
            entities.insert(entity, StyleNode::Parent(found.2.clone(), found.3.0));
        });

    entity_query.iter()
        .filter(|(_, _, _, updateable)| {
            info!("Checking {}", &updateable.0);
            to.contains(&updateable.0)
        })
        .for_each(|(entity, component, style, updateable)| {
            entities.insert(entity, StyleNode::Other(style.clone(), updateable.0));
        });
}

