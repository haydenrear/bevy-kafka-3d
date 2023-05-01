use bevy::prelude::{Button, Changed, Commands, Entity, EventWriter, Interaction, Query, Style, With};
use bevy::hierarchy::{Children, Parent};
use bevy::utils::HashMap;
use bevy::log::info;
use bevy::ecs::query::QueryEntityError;
use crate::menu::menu_event::{ChangePropagation, StartingState, StyleNode, UiComponent, UiEvent};
use crate::visualization::UiIdentifiableComponent;

/// Some events can affect multiple UiComponents. Therefore, when an Interaction happens, that Interaction
/// needs to be translated into a bunch of UiEvents. The UiEvents that are created will be based on
/// the state of the component and it's children, and so all of these components needs to be available
/// to query, to determine what UiEvent to propagate.
/// So the state event translator will take in the components and the state associated with those components
/// and the event that happened, and output a number of UiEvents.
///
/// First, there needs to be a translation from the Interaction and the UiComponent into which components
/// are affected by the UiComponent when that event happens. The UiComponent is built with information
/// about what happens to it's parents and children, and itself, and so there first is a metadata-translator
/// that takes that information about the UiComponent, queries to get the correct components, and then
/// passes them.
///
/// So then once those components are retrieved, the components for which the event happened, then
/// they are passed to the UiEventFactory, which will generate the UiEvents based on the type of UiEvent
/// that happened, and the state of the components that were involved in the event.
///
/// Every component state change that happens contains multiple events that need to be created, and
/// those events affect different components.
pub fn write_ui_events(
    mut commands: Commands,
    mut event_write: EventWriter<UiEvent>,
    entity_query: Query<(Entity, &UiComponent, &Style, &UiIdentifiableComponent), (With<UiComponent>, With<Style>)>,
    with_children_query: Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    with_parent_query: Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>,
    interaction_query: Query<(Entity, &Interaction), (With<Button>, Changed<Interaction>)>,
) {
    let _ = interaction_query
        .iter()
        .flat_map(|(entity, interaction)| {
            entity_query.get(entity)
                .map(|(entity, ui_component, style, updateable)| {
                    (entity, ui_component, style, updateable, interaction)
                })
        })
        .flat_map(|(entity, ui_component, style, updateable, interaction)| {
            if let Interaction::Clicked = interaction {
                return ui_component.get_state_change_types()
                    .iter()
                    .map(|state_change_action| {
                        return (entity, ui_component, style, updateable, interaction, state_change_action);
                    })
                    .collect();
            }

            vec![]
        })
        .map(|(entity, ui_component, style, updateable_value, interaction, state_change_action)| {
            info!("found state change action: {:?}", state_change_action);
            let clicked = state_change_action.clicked.clone();

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

            if let Some(ChangePropagation::CustomPropagation {  to, from }) = style_change_types {
                custom_propagation_entities(&entity_query, &with_children_query, &with_parent_query, entity, style, updateable_value, &mut entities, to, from);

                info!("Custom propagation event with {:?}.", entities);
            }

            info!("propagation event with {:?}.", entities);
            return clicked.get_ui_event(entities);
        })
        .for_each(|ui_event| {
            if ui_event.is_none() {
                info!("Failed to fetch ui event.");
                return;
            }
            info!("Sending UI event: {:?}.", ui_event);
            event_write.send(ui_event.unwrap());
        });
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
    })
}

fn custom_propagation_entities(
    entity_query: &Query<(Entity, &UiComponent, &Style, &UiIdentifiableComponent), (With<UiComponent>, With<Style>)>,
    with_children_query: &Query<(Entity, &UiComponent, &Children, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Children>, With<Style>)>,
    with_parent_query: &Query<(Entity, &UiComponent, &Parent, &UiIdentifiableComponent, &Style), (With<UiComponent>, With<Parent>, With<Style>)>, entity: Entity, style: &Style, updateable_value: &UiIdentifiableComponent, mut entities: &mut HashMap<Entity, StyleNode>, to: &Vec<f32>, from: &StartingState) {
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

