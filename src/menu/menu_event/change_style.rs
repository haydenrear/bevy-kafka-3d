use std::fmt::Debug;
use std::sync::Arc;
use bevy::log::info;
use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Component, Display, Entity, Size, Style, Val};
use crate::menu::menu_event::{ChangePropagation, ClickStateChangeState, StyleNode, StartingState, UiComponentFilters, UiEvent};
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

    fn do_create_change_size_ui_event(update_display: HashMap<Entity, Size>, current_display: HashMap<Entity, Size>) -> Option<UiEvent> {
        return Some(UiEvent::Event(ClickStateChangeState::ChangeSize {
            update_display,
            current_display
        }));
    }

    fn do_create_display_ui_event(update_display: HashMap<Entity, Display>, current_display: HashMap<Entity, Display>) -> Option<UiEvent> {
        return Some(UiEvent::Event(ClickStateChangeState::ChangeDisplay {
            update_display,
            current_display
        }));
    }

    fn do_create_updates<T: Clone>(entities: &HashMap<Entity, StyleNode>,
                                   component: &T,
                                   get_component: &dyn Fn(&StyleNode) -> T) -> (HashMap<Entity, T>, HashMap<Entity, T>) {
        let update_display = entities.keys()
            .map(|entity| {
                (entity.clone(), component.clone())
            })
            .collect::<HashMap<Entity, T>>();
        let current_display = entities.iter()
            .map(|(entity, node)| {
                (entity.clone(), get_component(node))
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
    fn do_create_ui_event(&self, update_display: HashMap<Entity, T>, current_display: HashMap<Entity, T>) -> Option<UiEvent>;
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
            pub(crate) fn do_change(&self, starting_state: &Style, entities: HashMap<Entity, StyleNode>) -> Option<UiEvent> {
                info!("Creating UI event for {:?}.", &entities);
                match self {
                    ChangeStyleTypes::RemoveVisible(_) => {
                        let values = Self::do_create_updates(&entities, &Display::None, &|node| node.get_style().display);
                        Self::do_create_display_ui_event(values.0, values.1)
                    }
                    ChangeStyleTypes::AddVisible(_) => {
                        let values = Self::do_create_updates(&entities, &Display::Flex, &|node| node.get_style().display);
                        Self::do_create_display_ui_event(values.0, values.1)
                    }
                    ChangeStyleTypes::ChangeVisible(_) => {
                        let display = Self::get_change_display(starting_state);
                        let values = Self::do_create_updates(&entities, &display, &|node| node.get_style().display);
                        info!("Found values: {:?} for changin visible.", values);
                        Self::do_create_display_ui_event(values.0, values.1)
                    }
                    ChangeStyleTypes::ChangeSize { width_1,width_2, height_1,height_2, .. } => {
                        let size = Self::size(height_1, height_2, width_1, width_2, starting_state);
                        size.map(|new_size| {
                                Self::do_create_updates(&entities, &new_size, &|node| node.get_style().size)
                            })
                            .map(|created| Self::do_create_change_size_ui_event(created.0, created.1))
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
                           value.do_create_ui_event(values.0, values.1)
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

gen_style_types!();

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

    /// Get the state of the node that will determine what the next state will be.
    pub(crate) fn get_current_state(
        &self,
        entities: &HashMap<Entity, StyleNode>,
        propagation: &ChangePropagation
    ) -> Option<Style> {
        match propagation.get_starting_state() {
            StartingState::SelfState => {
                Self::get_self_style(entities)
                    .or_else(|| {
                        info!("Failed to fetch style.");
                        None
                    })
            }
            StartingState::Child => {
                Self::get_child_style(entities)
                    .or_else(|| {
                        info!("Failed to fetch child style.");
                        None
                    })
            }
            StartingState::Parent => {
                Self::get_parent_style(entities)
                    .or_else(|| {
                        info!("Failed to fetch parent style.");
                        None
                    })
            }
            StartingState::Other(val) => {
                entities.iter()
                    .filter(|(entity, node_val)| {
                        node_val.id() == val
                    })
                    .map(|(entity, val)| {
                        val.get_style()
                    })
                    .next()
            }
        }
    }

    fn get_parent_style(entities: &HashMap<Entity, StyleNode>) -> Option<Style> {
        entities.iter()
            .filter(|(entity, node)| {
                if let StyleNode::Parent(_, _) = node {
                    return true
                }
                false
            })
            .flat_map(|(entity, node)| {
                if let StyleNode::Parent(style, _) = node {
                    return vec![style.clone()]
                }
                vec![]
            })
            .next()
    }

    fn get_child_style(entities: &HashMap<Entity, StyleNode>) -> Option<Style> {
        entities.iter()
            .flat_map(|(entity, node)| {
                info!("Checking style for {:?}.", node);
                if let StyleNode::Child(style, _) = node {
                    info!("Found child style.");
                    return vec![style.clone()]
                }
                vec![]
            })
            .next()
    }

    fn get_self_style(entities: &HashMap<Entity, StyleNode>) -> Option<Style> {
        entities.iter()
            .flat_map(|(entity, node)| {
                if let StyleNode::SelfNode(style, _) = node {
                    return vec![style.clone()]
                }
                vec![]
            })
            .next()
            .or_else(|| {
                info!("Could not find self style.");
                None
            })
    }

    fn get_filter(entities: HashMap<Entity, StyleNode>, remove_visible: &Option<UiComponentFilters>) -> HashMap<Entity, StyleNode> {
        if remove_visible.is_none() || (remove_visible.is_some() && remove_visible.as_ref().unwrap().exclude.is_none()) {
            let entities_id = entities.values().map(|n| n.id()).collect::<Vec<f32>>();
            info!("Including entities: {:?}.", &entities_id);
            return entities;
        }
        remove_visible
            .as_ref()
            .map(|exclude| {
                exclude.exclude.as_ref().map(|excluded| {
                    entities.into_iter()
                        .filter(|(entity, node)| !excluded.contains(&node.id()))
                        .collect::<HashMap<Entity, StyleNode>>()
                })
            })
            .flatten()
            .unwrap()
    }

}
