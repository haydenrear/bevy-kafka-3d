use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use bevy::log::info;
use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Component, Display, Entity, Size, Style, Val};
use crate::event::event_state::Update;
use crate::event::event_propagation::{ChangePropagation, StartingState};
use crate::menu::ui_menu_event::ui_state_change::UiClickStateChange;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiComponentFilters, UiEventArgs};
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

    fn do_create_updates<T: Clone + Debug + Default + Send + Sync>(entities: &HashMap<Entity, StyleNode>,
                                   component: &T,
                                   get_component: &dyn Fn(&StyleNode) -> T) -> (HashMap<Entity, Update<T>>, HashMap<Entity, T>) {
        let update_display = entities.keys()
            .map(|entity| {
                (entity.clone(), Update {update_to: Some(component.clone())})
            })
            .collect::<HashMap<Entity, Update<T>>>();
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
            _ => { entities }
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
                        node_val.id() == *val
                    })
                    .map(|(entity, val)| {
                        val.get_style()
                    })
                    .next()
            }
            StartingState::Sibling => {
                Self::get_sibling_style(entities)
                    .or_else(|| {
                        info!("Failed to fetch sibling style.");
                        None
                    })
            }
            StartingState::SiblingChild => {
                Self::get_sibling_child_style(entities)
                    .or_else(|| {
                        info!("Failed to fetch sibling style.");
                        None
                    })
            }
        }
    }

    fn get_sibling_child_style(entities: &HashMap<Entity, StyleNode>) -> Option<Style> {
        entities.iter()
            .flat_map(|(entity, node)| {
                if let StyleNode::SiblingChild(style, _) = node {
                    return vec![style.clone()]
                }
                vec![]
            })
            .next()
    }

    fn get_sibling_style(entities: &HashMap<Entity, StyleNode>) -> Option<Style> {
        entities.iter()
            .flat_map(|(entity, node)| {
                if let StyleNode::Sibling(style, _) = node {
                    return vec![style.clone()]
                }
                vec![]
            })
            .next()
    }

    fn get_parent_style(entities: &HashMap<Entity, StyleNode>) -> Option<Style> {
        entities.iter()
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

    fn get_filter(
        entities: HashMap<Entity, StyleNode>,
        remove_visible: &Option<UiComponentFilters>,
    ) -> HashMap<Entity, StyleNode> {
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

/// May consider adding a flag to signify that the state of that node should be the one to determine
/// the state of the others. For instance, if switching from visible to invisible, which node determines?
/// So you can use a flag here.
#[derive(Clone)]
pub enum StyleNode {
    Child(Style, f32),
    SelfNode(Style, f32),
    Parent(Style, f32),
    Sibling(Style, f32),
    SiblingChild(Style, f32),
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
            StyleNode::SiblingChild(_, id) => *id
        }
    }

    pub(crate) fn get_style(&self) -> Style {
        match self {
            StyleNode::SiblingChild(style, ..) => style.clone(),
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
