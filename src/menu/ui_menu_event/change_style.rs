use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use bevy::log::info;
use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Component, Display, Entity, ResMut, Size, Style, Val};
use crate::event::event_state::Update;
use crate::event::event_propagation::{ChangePropagation, StartingState};
use crate::menu::ui_menu_event::ui_state_change::UiClickStateChange;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StyleContext, UiComponentFilters, UiComponentState, UiEventArgs};
use crate::Node;

/// Need to translate StyleChangeType to UiComponentFilters, and then pass the UiComponentFilters to
/// a general function or trait to execute behavior for all UiComponentFilters.
/// UiComponentType (Type of node) -> UiComponentFilter
/// UiComponentFilter -> ChangeStyleBehavior
impl ChangeStyleTypes {

    fn is_excluded(component_id: f32, exclude: &Option<Vec<f32>>) -> bool {
        if exclude.is_some() && exclude.as_ref().unwrap().iter().any(|e| *e == component_id) {
            info!("Was excluded");
            return true;
        }
        false
    }

    fn do_create_change_size_ui_event(update_display: HashMap<Entity, Update<Size>>) -> Option<UiEventArgs> {
        return Some(UiEventArgs::Event(UiClickStateChange::ChangeSize {
            update_display,
        }));
    }

    fn do_create_display_ui_event(update_display: HashMap<Entity, Update<Display>>) -> Option<UiEventArgs> {
        return Some(UiEventArgs::Event(UiClickStateChange::ChangeDisplay {
            update_display,
        }));
    }

    fn do_create_updates<T: Clone + Debug + Default + Send + Sync>(
        entities: HashMap<&Entity, &StyleNode>,
        to_update: HashMap<Entity, Style>,
        get_component: &dyn Fn(&StyleNode) -> T,
        get_next_state_component: &dyn Fn(&Style) -> Option<T>,
    ) -> (HashMap<Entity, Update<T>>, HashMap<Entity, T>) {
        let update_display = entities.keys()
            .flat_map(|entity| {
                to_update.get(entity)
                    .iter()
                    .flat_map(|component| get_next_state_component(*component)
                            .map(|component| (**entity, Update {update_to: Some(component)}))
                    )
                    .collect::<Vec<(Entity, Update<T>)>>()
            })
            .collect::<HashMap<Entity, Update<T>>>();
        let current_display = entities.iter()
            .map(|(entity, node)| {
                (**entity, get_component(node))
            })
            .collect::<HashMap<Entity, T>>();
        (update_display, current_display)
    }

    fn get_change_display(style: &Style) -> Display {
        let mut display = style.display.clone();
        if display == Display::None {
            display = Display::Flex;
        } else {
            display = Display::None;
        }
        display
    }
}

pub trait ChangeStyle<T, S>: Send + Sync + Debug + Clone {
    fn do_create_ui_event(&self,
                          update_display: HashMap<Entity, T>,
                          current_display: HashMap<Entity, T>
    ) -> Option<UiEventArgs>;
    fn get_change(&self, value: &S) -> T;
    fn get_value(self, node: &StyleNode) -> T;
}

macro_rules! gen_style_types {

    ($($name:ident, $value:ty),*) => {

        #[derive(Clone, Debug)]
        pub enum ChangeStyleTypes {
            RemoveVisible(Option<UiComponentFilters>),
            AddVisible(Option<UiComponentFilters>),
            ChangeVisible(Option<UiComponentFilters>),
            ChangeSize {
                height_1: f32,
                height_2: f32,
                width_1: f32,
                width_2: f32,
                filters: Option<UiComponentFilters>
            },
            $(
               $name($value, Option<UiComponentFilters>),
            )*
        }

        impl ChangeStyleTypes {
            pub(crate) fn do_change(&self, starting_state: &Style, entities: HashMap<Entity, StyleNode>) -> Option<UiEventArgs> {
                info!("Creating UI event for {:?}.", &entities);
                match self {
                    ChangeStyleTypes::RemoveVisible(_) => {
                        let values = Self::do_create_updates(&entities, &Display::None, &|node| node.get_style().display);
                        Self::do_create_display_ui_event(values.0)
                    }
                    ChangeStyleTypes::AddVisible(_) => {
                        let values = Self::do_create_updates(&entities, &Display::Flex, &|node| node.get_style().display);
                        Self::do_create_display_ui_event(values.0)
                    }
                    ChangeStyleTypes::ChangeVisible(_) => {
                        let display = Self::get_change_display(starting_state);
                        let values = Self::do_create_updates(&entities, &display, &|node| node.get_style().display);
                        info!("Found values: {:?} for changin visible.", values);
                        Self::do_create_display_ui_event(values.0)
                    }
                    ChangeStyleTypes::ChangeSize { width_1,width_2, height_1,height_2, .. } => {
                        let size = Self::size(height_1, height_2, width_1, width_2, starting_state);
                        size.map(|new_size| {
                                Self::do_create_updates(&entities, &new_size, &|node| node.get_style().size)
                            })
                            .map(|created| Self::do_create_change_size_ui_event(created.0))
                            .flatten()
                            .or_else(|| {
                                info!("Sizes did not match.");
                                None
                            })
                    }
                    $(
                       ChangeStyleTypes::$name(value, _) => {
                           let changed = value.get_change(starting_state);
                           let values = Self::do_create_updates(&entities, &changed, &value.get_value);
                           value.do_create_ui_event(values.0)
                       }
                    )*
                }
            }


            pub(crate) fn filter_entities(&self,
                                          entities: HashMap<Entity, StyleNode>
            ) ->  HashMap<Entity, StyleNode> {
                match self {
                    ChangeStyleTypes::RemoveVisible(remove_visible) => {
                        Self::get_filter(entities, remove_visible)
                    }
                    ChangeStyleTypes::AddVisible(add_visible) => {
                        Self::get_filter(entities, add_visible)
                    }
                    ChangeStyleTypes::ChangeVisible(change_visible) => {
                        Self::get_filter(entities, change_visible)
                    }
                    ChangeStyleTypes::ChangeSize { filters, .. } => {
                        Self::get_filter(entities, filters)
                    }
                    $(
                       ChangeStyleTypes::$name(_, filters) => {
                            Self::get_filter(entities, filters)
                       }
                    )*
                    _ => {
                        entities
                    }
                }
            }

        }
    }
}

// gen_style_types!();

#[derive(Clone, Debug)]
pub enum ChangeStyleTypes {
    RemoveVisible(Option<UiComponentFilters>),
    AddVisible(Option<UiComponentFilters>),
    ChangeVisible(Option<UiComponentFilters>),
    ChangeSize {
        height_1: f32,
        height_2: f32,
        width_1: f32,
        width_2: f32,
        filters: Option<UiComponentFilters>
    },
    UpdateSize {
        height_1: f32,
        width_1: f32,
        filters: Option<UiComponentFilters>
    },
}

impl ChangeStyleTypes {
    pub(crate) fn do_change(&self, starting_state: HashMap<Entity, Style>, entities: HashMap<&Entity, &StyleNode>) -> Option<UiEventArgs> {
        info!("Creating UI event for {:?}.", &entities);
        info!("{} is size of staring and {} is size of entities.", starting_state.len(), entities.len());
        match self {
            ChangeStyleTypes::RemoveVisible(_) => {
                let values = Self::do_create_updates(
                    entities,
                    starting_state,
                    &|node| node.get_style().display,
                        &|style| {
                        if style.display == Display::Flex {
                            return Some(Display::None);
                        }
                        None
                    }
                );
                Self::do_create_display_ui_event(values.0)
            }
            ChangeStyleTypes::AddVisible(_) => {
                let values = Self::do_create_updates(
                    entities,
                    starting_state,
                    &|node| node.get_style().display,
                    &|style| {
                        if style.display == Display::None {
                            return Some(Display::Flex);
                        }
                        None
                    }
                );

                Self::do_create_display_ui_event(values.0)
            }
            ChangeStyleTypes::ChangeVisible(_) => {
                let values = Self::do_create_updates(
                    entities,
                    starting_state,
                    &|node| node.get_style().display,
                    &|style| {
                        if style.display == Display::Flex {
                            return Some(Display::None);
                        }
                        Some(Display::Flex)
                    }
                );

                info!("Found values: {:?} for changin visible.", values);
                Self::do_create_display_ui_event(values.0)
            }
            ChangeStyleTypes::ChangeSize { width_1,width_2, height_1,height_2, .. } => {
                let created = Self::do_create_updates(
                        entities,
                        starting_state,
                        &|node| node.get_style().size,
                        &|style| Self::size(height_1, height_2, width_1, width_2, style)
                );
                Self::do_create_change_size_ui_event(created.0)
            }
            ChangeStyleTypes::UpdateSize { width_1, height_1, .. } => {
                info!("Doing size update with {} and {}", width_1, height_1);
                info!("Doing size Change with entities: {:?} and starting state: {:?}", entities, &starting_state);
                let size = Size::new(Val::Percent(*width_1), Val::Percent(*height_1));
                let created = Self::do_create_updates(
                    entities,
                    starting_state,
                    &|node| node.get_style().size,
                    &|style| Some(size)
                );
                info!("Size updates: {:?}", created);
                Self::do_create_change_size_ui_event(created.0)
            }
        }
    }


    pub(crate) fn filter_entities<'a>(&'a self, entities: &'a HashMap<Entity, StyleNode>) ->  HashMap<&Entity, &StyleNode> {
        match self {
            ChangeStyleTypes::RemoveVisible(remove_visible) => {
                Self::get_filter(entities, remove_visible)
            }
            ChangeStyleTypes::AddVisible(add_visible) => {
                Self::get_filter(entities, add_visible)
            }
            ChangeStyleTypes::ChangeVisible(change_visible) => {
                Self::get_filter(entities, change_visible)
            }
            ChangeStyleTypes::ChangeSize { filters, .. } => {
                info!("{:?} are the entities being filtered for size.", entities);
                Self::get_filter(entities, filters)
            }
            ChangeStyleTypes::UpdateSize { filters, .. } => {
                info!("{:?} are the entities being filtered for size.", entities);
                Self::get_filter(entities, filters)
            }
        }
    }

}

impl ChangeStyleTypes {

    pub(crate) fn size(height_1: &f32, height_2: &f32, width_1: &f32, width_2: &f32, starting_state: &Style) -> Option<Size> {
        let mut size = None;
        info!("{:?} is starting and height_1: {}, height_2: {}, width_1: {}, width_2: {}", starting_state.size, height_1, height_2, width_1, width_2);
        if let Val::Percent(height)  = starting_state.size.height {
            info!("{} is height and {} is height_1", height, height_1);
            if &height  == height_1 {
                if let Val::Percent(width) = starting_state.size.width {
                    info!("{} is width and {} is width_1", width, width_1);
                    if &width == width_1 {
                        return Some(Size::new(Val::Percent(*width_2), Val::Percent(*height_2)));
                    } else if height_1 == height_2 {
                        if &width == width_2 {
                            return Some(Size::new(Val::Percent(*width_1), Val::Percent(*height_1)));
                        }
                    }
                }
            } else if &height  == height_2 {
                if let Val::Percent(width) = starting_state.size.width {
                    if &width == width_2 {
                        return Some(Size::new(Val::Percent(*width_1), Val::Percent(*height_1)));
                    } else if height_1 == height_2 {
                        info!("{} is width and {} is width_1", width, width_1);
                        if &width == width_1 {
                            return Some(Size::new(Val::Percent(*width_2), Val::Percent(*height_2)));
                        }
                    }
                }
            }
        }
        info!("Sizes did not match");
        size
    }

    fn get_style(style: Option<Style>, entities: &HashMap<Entity, StyleNode>) -> HashMap<Entity, Style> {
        info!("Fetching style for nodes: {:?}", entities) ;
        style.map(|style| {
                entities.iter()
                    .map(|(entity, _)| (*entity, style.clone()))
                    .collect()
            })
            .or(Some(HashMap::new()))
            .unwrap()
    }

    /// Get the state of the node that will determine what the next state will be.
    pub(crate) fn get_current_state(
        &self,
        entities: &HashMap<Entity, StyleNode>,
        propagation: &StartingState,
    ) -> HashMap<Entity, Style> {
        match propagation {
            StartingState::SelfState => {
                Self::get_style(
                    Self::get_this_style(entities, &|node| matches!(node, StyleNode::SelfNode(_, _)))
                            .or_else(|| {
                                info!("Failed to fetch style.");
                                None
                            }),
                    entities
                )
            }
            StartingState::Child => {
                Self::get_style(
                    Self::get_this_style(entities, &|node| matches!(node, StyleNode::Child(_, _)))
                        .or_else(|| {
                            info!("Failed to fetch child style.");
                            None
                        }),
                    entities
                )
            }
            StartingState::Parent => {
                Self::get_style(
                    Self::get_this_style(entities, &|node| matches!(node, StyleNode::Parent(_, _)))
                        .or_else(|| {
                            info!("Failed to fetch parent style.");
                            None
                        }),
                    entities
                )
            }
            StartingState::Other(val) => {
                Self::get_style(
                    entities.iter()
                        .filter(|(entity, node_val)| {
                            node_val.id() == *val
                        })
                        .map(|(entity, val)| {
                            val.get_style()
                        })
                        .next(),
                    entities
                )
            }
            StartingState::Sibling => {
                Self::get_style(
                    Self::get_this_style(entities, &|node| matches!(node, StyleNode::Sibling(_, _)))
                        .or_else(|| {
                            info!("Failed to fetch sibling style.");
                            None
                        }),
                    entities
                )
            }
            StartingState::SiblingChild => {
                Self::get_style(
                    Self::get_this_style(entities, &|node| matches!(node, StyleNode::SiblingChild(_, _)))
                        .or_else(|| {
                            info!("Failed to fetch sibling style.");
                            None
                        }),
                    entities
                )
            }
            StartingState::VisibleState(visible) => {
                let mut style = Style::default();
                style.display = visible.clone();
                Self::get_style(
                    Some(style),
                    entities
                )
            }
            StartingState::EachSelfState => {
                entities.iter()
                    .map(|entity| {
                        info!("Starting state: {:?}", entity.1);
                        (*entity.0, entity.1.get_style())
                    })
                    .collect()
            }
        }
    }

    fn get_this_style(entities: &HashMap<Entity, StyleNode>, filter: &dyn Fn(&StyleNode) -> bool) -> Option<Style> {
        entities.iter()
            .flat_map(|(entity, node)| {
                if filter(node) {
                    return vec!(node.get_style());
                }
                vec![]
            })
            .next()
    }

    fn get_filter<'a>(
        entities: &'a HashMap<Entity, StyleNode>,
        remove_visible: &Option<UiComponentFilters>,
    ) -> HashMap<&'a Entity, &'a StyleNode> {
        if remove_visible.is_none() || (remove_visible.is_some() && remove_visible.as_ref().unwrap().exclude.is_none()) {
            let entities_id = entities.values().map(|n| n.id()).collect::<Vec<f32>>();
            info!("Including entities: {:?}.", &entities_id);
            return entities.into_iter().collect();
        }
        remove_visible
            .as_ref()
            .map(|exclude| {
                exclude.exclude.as_ref().map(|excluded| {
                    entities.into_iter()
                        .filter(|(entity, node)| !excluded.contains(&node.id()))
                        .collect::<HashMap<&Entity, &StyleNode>>()
                })
            })
            .flatten()
            .unwrap()
    }

}

#[derive(Clone)]
pub enum StyleNode {
    Child(Style, f32),
    SelfNode(Style, f32),
    Parent(Style, f32),
    Sibling(Style, f32),
    SiblingChild(Style, f32),
    SiblingChildRecursive(Style, f32),
    Other(Style, f32)
}

impl Debug for StyleNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node enum: ");
        match self {
            StyleNode::Child(_, _) => {
                f.write_str(" Child ");
            }
            StyleNode::SelfNode(_, _) => {
                f.write_str(" Self ");
            }
            StyleNode::Parent(_, _) => {
                f.write_str(" Parent ");
            }
            StyleNode::Other(_, _) => {
                f.write_str(" Other ");
            }
            StyleNode::Sibling(_,_) => {
                f.write_str(" Sibling ");
            }
            StyleNode::SiblingChild(..) => {
                f.write_str(" Sibling Child ");
            }
            StyleNode::SiblingChildRecursive(..) => {
                f.write_str(" Sibling Child Recursive ");
            }
        }
        f.write_str(self.id().to_string().as_str())
    }
}

impl StyleNode {
    pub(crate) fn id(&self) -> f32 {
        match self {
            StyleNode::Child(_, id) => {
                *id
            }
            StyleNode::SelfNode(_, id) => {
                *id
            }
            StyleNode::Parent(_, id) => {
                *id
            }
            StyleNode::Other(_, id) => {
                *id
            }
            StyleNode::Sibling(_, id) => {
                *id
            }
            StyleNode::SiblingChild(_, id) => *id,
            StyleNode::SiblingChildRecursive(_, id) => *id
        }
    }

    pub(crate) fn get_style(&self) -> Style {
        match self {
            StyleNode::SiblingChild(style, ..) => style.clone(),
            StyleNode::SiblingChildRecursive(style, ..) => style.clone(),
            StyleNode::Child(style, _) => {
                style.clone()
            }
            StyleNode::SelfNode(style, id) => {
                style.clone()
            }
            StyleNode::Parent(style, id) => {
                style.clone()
            }
            StyleNode::Other(style, _) => {
                style.clone()
            }
            StyleNode::Sibling(style, _) => {
                style.clone()
            }
        }
    }
}
