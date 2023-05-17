use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use bevy::log::{error, info};
use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Component, Display, Entity, ResMut, Size, Style, Val, Vec2};
use bevy::ui::UiRect;
use crate::event::event_state::Update;
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::menu::Position;
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

    fn do_create_change_size_ui_event(update_display: Update<Size>, entity: Entity) -> Option<UiEventArgs> {
        return Some(UiEventArgs::Event(UiClickStateChange::ChangeSize {
            update_display,
            entity
        }));
    }

    fn do_create_display_ui_event(update_display: Update<Display>, entity: Entity) -> Option<UiEventArgs> {
        return Some(UiEventArgs::Event(UiClickStateChange::ChangeDisplay {
            update_display,
            entity
        }));
    }

    fn do_create_updates<T: Clone + Debug + Default + Send + Sync>(
        to_update: &Style,
        get_next_state_component: &dyn Fn(&Style) -> Option<T>,
    ) -> Option<Update<T>> {
        let opt: Option<T> = get_next_state_component(to_update);
        opt.map(|update_to| Update { update_to: Some(update_to) })
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
    RemoveVisible,
    AddVisible,
    ChangeVisible,
    ChangeSize {
        height_1: f32,
        height_2: f32,
        width_1: f32,
        width_2: f32,
    },
    UpdateSize {
        height_1: f32,
        width_1: f32,
    },
    Selected {
    },
    DragX,
    DragY,
    ScrollX,
    ScrollY,
}

pub enum SelectionType {

}

impl ChangeStyleTypes {
    pub(crate) fn do_change(
        &self,
        starting_state: &Style,
        entity: Entity,
        style_context: &mut ResMut<StyleContext>
    ) -> Option<UiEventArgs> {
        match self {
            ChangeStyleTypes::RemoveVisible => {
                let values = Self::do_create_updates(
                    starting_state,
                        &|style| {
                        if style.display == Display::Flex {
                            return Some(Display::None);
                        }
                        None
                    }
                );
                if values.is_none() {
                    return None;
                }
                Self::do_create_display_ui_event(values.unwrap(), entity)
            }
            ChangeStyleTypes::AddVisible => {
                let values = Self::do_create_updates(
                    starting_state,
                    &|style| {
                        if style.display == Display::None {
                            return Some(Display::Flex);
                        }
                        None
                    }
                );
                if values.is_none() {
                    return None;
                }
                Self::do_create_display_ui_event(values.unwrap(), entity)
            }
            ChangeStyleTypes::ChangeVisible => {
                let values = Self::do_create_updates(
                    starting_state,
                    &|style| {
                        if style.display == Display::Flex {
                            return Some(Display::None);
                        }
                        Some(Display::Flex)
                    }
                );
                if values.is_none() {
                    return None;
                }
                Self::do_create_display_ui_event(values.unwrap(), entity)
            }
            ChangeStyleTypes::ChangeSize { width_1,width_2, height_1,height_2, .. } => {
                let created = Self::do_create_updates(
                        starting_state,
                        &|style| Self::size(height_1, height_2, width_1, width_2, style)
                );
                if created.is_none() {
                    return None;
                }
                Self::do_create_change_size_ui_event(created.unwrap(), entity)
            }
            ChangeStyleTypes::UpdateSize { width_1, height_1, .. } => {
                let size = Size::new(Val::Percent(*width_1), Val::Percent(*height_1));
                let created = Self::do_create_updates(
                    starting_state,
                    &|style| Some(size)
                );
                if created.is_none() {
                    return None;
                }
                Self::do_create_change_size_ui_event(created.unwrap(), entity)
            }
            ChangeStyleTypes::DragX => {
                if !style_context.is_dragging || style_context.delta.is_none() {
                    return None;
                }
                let mut style = starting_state.clone();
                let updated_drag = match &style.position.left {
                    Val::Px(mut px) => {
                        let prev = px;
                        px += style_context.delta.unwrap().x;
                        style_context.delta = None;
                        let mut pos = style.position.clone();
                        info!("Updating from {} to {}", prev, px);
                        pos.left = Val::Px(px);
                        Some(pos)
                    }
                    Val::Percent(mut percent) => {
                        error!("Drag implemented with percentages. Not good.");
                        None
                    }
                    _ => {
                        None
                    }
                }.map(|updated_drag| {
                    Update::<UiRect> {
                        update_to: Some(updated_drag),
                    }
                }).map(|updated| {
                    Some(UiEventArgs::Event(UiClickStateChange::Slider {
                        update_scroll: updated,
                        entity
                    }))
                }).flatten();
                updated_drag
            }
            ChangeStyleTypes::Selected { .. } => {
                error!("In selected!");
                None
            },
            ChangeStyleTypes::DragY => {
                error!("In drag!");
                None
            }
            ChangeStyleTypes::ScrollX => {
                error!("In drag!");
                None
            }
            ChangeStyleTypes::ScrollY => {
                error!("In drag!");
                None
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
