use std::fmt::{Debug, Formatter, Pointer, Write};
use std::marker::PhantomData;
use bevy::prelude::{BackgroundColor, Button, Changed, Children, Color, Commands, Component, Condition, Display, Entity, EventReader, EventWriter, Interaction, ParamSet, Parent, Query, Style, With, Without, World};
use bevy::app::{App, Plugin};
use bevy::ecs::component::ComponentId;
use bevy::ecs::query::QueryEntityError;
use bevy::log::info;
use bevy::ui::{Size, ui_focus_system, Val};
use bevy::utils::HashMap;
use bevy_mod_picking::Hover;
use crate::menu::{CollapsableMenu, Dropdown, DropdownOption};
use crate::menu::menu_event::HoverStateChange::ColoredHover;
use crate::visualization;
use crate::visualization::UpdateableComponent;

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(interaction_event_system)
            .add_system(event_read_event)
            .add_system(hover_event)
            .add_startup_system(visualization::create_dropdown)
            .add_event::<UiEvent>();
    }
}

#[derive(Debug)]
pub enum UiEvent {
    Event(ClickStateChangeState)
}

#[derive(Clone, Debug)]
pub enum HoverStateChange {
    ColoredHover {
        color: Color
    }, None
}

#[derive(Clone, Debug)]
pub enum ClickStateChange {
    ChangeColorStateChange(Color, Vec<StyleChangeType>),
    ChangeDisplay(ChangeStyle, Vec<StyleChangeType>),
    None
}

#[derive(Clone, Debug)]
pub enum StyleChangeType {
    Parent,
    Child,
    SelfChange,
    FilterDriven {
        exclude: Option<Vec<f32>>,
        include: Option<Vec<f32>>
    }
}

impl ClickStateChange {
    pub fn get_ui_event(&self, args: &HashMap<Entity, Node>) -> Option<UiEvent> {
        if let ClickStateChange::ChangeDisplay(change_style, _) = self {
            return change_style.get_ui_event(args);
        }
        None
    }

    pub fn style_change_types(&self) -> Option<&Vec<StyleChangeType>> {
        match self {
            ClickStateChange::ChangeColorStateChange(_, change_type) => {
                Some(&change_type)
            }
            ClickStateChange::ChangeDisplay(_, change_type) => {
                Some(&change_type)
            }
            ClickStateChange::None => {
                None
            }
        }
    }
}

pub trait StyleChange {
    fn get_ui_event(&self, style: &HashMap<Entity, Node>) -> Option<UiEvent>;
}

/// May consider adding a flag to signify that the state of that node should be the one to determine
/// the state of the others. For instance, if switching from visible to invisible, which node determines?
/// So you can use a flag here.
pub enum Node {
    Child(Style, f32),
    SelfNode(Style, f32),
    Parent(Style, f32)
}

impl ChangeStyle {
    fn do_change_size(width_1: &f32, width_2: &f32, height_1: &f32, height_2: &f32, style: &HashMap<Entity, Node>) -> Option<UiEvent> {
        let mut current_display = HashMap::new();
        let mut update_display = HashMap::new();
        style.iter().filter(|n| matches!(n.1, Node::SelfNode(_,_)))
            .for_each(|(entity, node)| {
                if let Node::SelfNode(style, _) = node {
                    let current_height = &style.size.height;
                    if let Val::Percent(value) = current_height {
                        if height_1 == value {
                            update_display.insert(entity.clone(), Size::new(Val::Percent(*width_2), Val::Percent(*height_2)));
                        } else {
                            update_display.insert(entity.clone(), Size::new(Val::Percent(*width_1), Val::Percent(*height_1)));
                        }
                    }
                    current_display.insert(entity.clone(), style.size);
                }
            });
        return Some(UiEvent::Event(ClickStateChangeState::ChangeSize {
            current_display,
            update_display
        }));
    }
}

impl StyleChange for ChangeStyle {
    /// Takes in the hashmap that describes the entity and the current style that it has, and then
    /// returns a UiEvent containing entities with current state and the calculated next state.
    ///
    /// The nodes are passed in as Node, which associates style with node type, so that the style
    /// of the child node can be set based on the state of the parent node. For instance, if both
    /// should be set according to the parent node, the state of the parent node can be checked in
    /// order to determine how to set the state of the child node.
    fn get_ui_event(&self, style: &HashMap<Entity, Node>) -> Option<UiEvent> {
        if let ChangeStyle::ChangeSize { width_1, width_2, height_1, height_2} = self {
            return Self::do_change_size(width_1, width_2, height_1, height_2, style);
        } else if let ChangeStyle::ChangeVisible(values) = self {

        }
        None
    }
}

/// Contains the state data needed in order to generate the UIEvents from the state change required.
#[derive(Clone, Debug)]
pub enum ClickStateChangeState {
    ChangeColor{
        current_display: HashMap<Entity, Color>,
        update_display: HashMap<Entity, Color>,
    },
    ChangeDisplay{
        current_display: HashMap<Entity, Display>,
        update_display: HashMap<Entity, Display>,
    },
    ChangeSize {
        current_display: HashMap<Entity, Size>,
        update_display: HashMap<Entity, Size>,
    },
    None
}

#[derive(Clone, Debug)]
pub enum ColorChange {
    ChangeColor(Color),
    SwapColor {
        color_1: Color,
        color_2: Color
    }
}

impl ColorChange {
    fn change_color(&self, mut display: &mut BackgroundColor) {
        match &self {
            ColorChange::ChangeColor(color) => {
                display.0 = color.clone();
            }
            ColorChange::SwapColor{ color_1, color_2 } => {
                if &display.0 == color_1 {
                    display.0 = color_2.clone();
                } else {
                    display.0 = color_1.clone();
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ChangeStyle {
    RemoveVisible(Option<UiComponentFilters>),
    AddVisible(Option<UiComponentFilters>),
    ChangeVisible(Option<UiComponentFilters>),
    ChangeSize {
        height_1: f32,
        height_2: f32,
        width_1: f32,
        width_2: f32
    }
}

#[derive(Clone, Debug)]
pub struct UiComponentFilters {
    pub(crate) exclude: Option<Vec<f32>>,
    pub(crate) include: Option<Vec<f32>>
}

impl ClickStateChangeState {

    fn change_display(&self, mut style: &mut Style, entity: Entity) {
        match self {
            ClickStateChangeState::ChangeDisplay { update_display, .. } => {
                update_display.get(&entity)
                    .map(|found_entity| style.display = found_entity.clone());
            }
            ClickStateChangeState::None => {}
            _ => {}
        };
    }

    fn change_background_color(&self, mut background_color: &mut BackgroundColor, entity: Entity) {
        match self {
            ClickStateChangeState::ChangeColor { update_display, .. } => {
                update_display.get(&entity)
                    .map(|found_entity| background_color.0 = found_entity.clone());
            }
            ClickStateChangeState::None => {
            }
            _ => {
            }
        };
    }

    fn change_visible(style: &mut &mut Style) {
        match &style.display {
            Display::Flex => {
                style.display = Display::None;
            }
            Display::None => {
                style.display = Display::Flex;
            }
        }
    }

    fn is_included(component_id: f32, include: &Option<Vec<f32>>) -> bool {
        include.is_some() && include.as_ref().unwrap().iter().any(|i| *i == component_id)
    }

    fn is_excluded(component_id: f32, exclude: &Option<Vec<f32>>) -> bool {
        if exclude.is_some() && exclude.as_ref().unwrap().iter().any(|e| *e == component_id) {
            info!("Was excluded");
            return true;
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct StateChangeActionType {
    pub(crate) hover: HoverStateChange,
    pub(crate) clicked: ClickStateChange
}

#[derive(Component, Debug, Clone)]
pub enum UiComponent {
    Dropdown(Dropdown, Vec<StateChangeActionType>),
    DropdownOption(DropdownOption, Vec<StateChangeActionType>),
    CollapsableMenuComponent(CollapsableMenu, Vec<StateChangeActionType>)
}

impl UiComponent {
    pub fn get_state_change_types(&self) -> &Vec<StateChangeActionType> {
        match self {
            UiComponent::Dropdown(_, events) => {
                events
            }
            UiComponent::DropdownOption(_, events) => {
                events
            }
            UiComponent::CollapsableMenuComponent(_, events) => {
                events
            }
        }
    }
}

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
fn interaction_event_system(
    mut commands: Commands,
    mut event_write: EventWriter<UiEvent>,
    entity_query: Query<(Entity, &UiComponent, &Style, &UpdateableComponent), (With<UiComponent>)>,
    child_query: Query<(Entity, &UiComponent, &Children, &UpdateableComponent), (With<Button>, With<UiComponent>, With<Children>)>,
    parent_query: Query<(Entity, &UiComponent, &Parent, &UpdateableComponent), (With<Button>, With<UiComponent>, With<Parent>)>,
    interaction_query: Query<(Entity, &Interaction), (With<Button>, Changed<Interaction>, With<UiComponent>)>,
) {
    let _ = interaction_query
        .iter()
        .flat_map(|(entity, interaction)|
            entity_query.get(entity)
                .map(|(entity, ui_component, style, updateable)|
                    (entity, ui_component, style, updateable, interaction)
                )
        )
        .map(|(entity, ui_component, style, updateable, interaction)| {

            if let Interaction::Clicked = interaction {

                let state_change_action = ui_component.get_state_change_types();

                for state_change_action in state_change_action.iter() {
                    let clicked = state_change_action.clicked.clone();

                    let style_change_types = clicked.style_change_types();
                    let mut entities = HashMap::new();

                    if style_change_types
                        .iter()
                        .flat_map(|v| v.iter())
                        .any(|any| matches!(any, StyleChangeType::Parent)) {

                        let parent = parent_query.get(entity.clone())
                            .map(|(_, _, parent, updateable)| parent.get())
                            .ok();

                        parent.map(|parent| {
                            entity_query.get(parent.clone())
                                .map(|(_, _, style, update)| {
                                    let node_style = Node::Parent(style.clone(), update.0);
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

                    if style_change_types.iter()
                        .flat_map(|v| v.iter())
                        .any(|any| matches!(any, StyleChangeType::SelfChange)) {

                        let node_style = Node::SelfNode(style.clone().clone(), updateable.0);
                        entities.insert(entity.clone(), node_style);

                    }

                    if style_change_types.iter()
                        .flat_map(|v| v.iter())
                        .any(|any| matches!(any, StyleChangeType::Child)) {

                        let child_entities = child_query.get(entity.clone())
                            .map(|(_, _, child, update)| child.iter()
                                .map(|child| child.clone()).collect::<Vec<Entity>>()
                            )
                            .ok()
                            .or(Some(vec![]))
                            .unwrap();

                        child_entities.iter().for_each(|child| {
                            let _ = entity_query.get(child.clone())
                                .map(|entity| {
                                    let node_style = Node::Child(entity.2.clone(), entity.3.0);
                                    entities.insert(entity.0.clone(), node_style);
                                })
                                .or_else(|_| {
                                    info!("Error fetching query for child.");
                                    Ok::<(), QueryEntityError>(())
                                });
                        })
                    }

                    return clicked.get_ui_event(&entities);
                }
            }
            None
        })
        .map(|ui_event| {
            if ui_event.is_none() {
                info!("Failed to fetch ui event.");
                return;
            }
            event_write.send(ui_event.unwrap());
        });

}

fn hover_event(
    mut query: Query<(&mut Style, &mut BackgroundColor, &Interaction), (With<UiComponent>, With<Button>, Changed<Interaction>)>,

) {
    for (_, mut color, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
            }
            Interaction::Hovered => {
                color.0 = Color::YELLOW;
            }
            Interaction::None => {
                color.0 = Color::GREEN;
            }
        }
    }
}

fn event_read_event(
    mut commands: Commands,
    mut event_write: EventReader<UiEvent>,
    mut query: ParamSet<(
        Query<(Entity, &UiComponent, &mut Style, &UpdateableComponent), (With<UiComponent>)>,
        Query<(Entity, &UiComponent, &mut BackgroundColor, &UpdateableComponent), (With<UiComponent>)>
    )>
) {
    for event in event_write.iter() {
        if let UiEvent::Event(ClickStateChangeState::ChangeColor{ current_display, update_display}) = event {
            update_display.iter().for_each(|(entity, color)| {
                let _ = query.p1()
                    .get_mut(entity.clone())
                    .map(|(_, _, mut color_update, _)| {
                        color_update.0 = color.clone();
                    })
                    .or_else(|_| {
                        info!("Failed to update color.");
                        Ok::<(), QueryEntityError>(())
                    });
            });
        } else if let UiEvent::Event(ClickStateChangeState::ChangeSize{ current_display, update_display}) = event {
            update_display.iter().for_each(|(entity, size)| {
                let _ = query.p0()
                    .get_mut(entity.clone())
                    .map(|(_, _, mut style, _)| {
                        style.size = size.clone();
                    })
                    .or_else(|_| {
                        info!("Failed to update color.");
                        Ok::<(), QueryEntityError>(())
                    });
            });
        } else if let UiEvent::Event(ClickStateChangeState::ChangeDisplay {current_display, update_display}) = event {
            update_display.iter().for_each(|(entity, display)| {
                let _ = query.p0()
                    .get_mut(entity.clone())
                    .map(|(_, _, mut style, _)| {
                        style.display = display.clone();
                    })
                    .or_else(|_| {
                        info!("Failed to update color.");
                        Ok::<(), QueryEntityError>(())
                    });
            });

        }
    }
}
