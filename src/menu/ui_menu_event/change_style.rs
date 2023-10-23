use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use bevy::log::{error, info};
use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Component, Display, Entity, ResMut, Style, Val, Vec2};
use bevy::text::Text;
use bevy::ui::UiRect;
use crate::event::event_state::Update;
use crate::menu::Position;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_state_change::UiClickStateChange;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiEventArgs};
use crate::Node;
use crate::ui_components::Size;


#[derive(Clone, Debug)]
pub enum UiChangeTypes {
    RemoveVisible{value: ()},
    AddVisible{value: ()},
    ChangeVisible{value: ()},
    ChangeSize {
        value: (
            // height_1
            f32,
            // height_2
            f32,
            // width_1
            f32,
            // width_2
            f32
        )
    },
    UpdateSize {
        value: (
            // height
            f32,
            // width
            f32
        )
    },
    Selected {
        value: ()
    },
    DragXPosition{
        value: ()
    },
    DragYPosition{
        value: ()
    },
    ScrollX{
        value: ()
    },
    ScrollY{
        value: ()
    },
}


pub trait DoChange<T> {
    fn do_change(&self, starting_state: &T, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs>;
}

macro_rules! updates {
    ($state:ty, $($update_fn:ident, $enum_variant:ident),*) => {
            impl DoChange<$state> for UiChangeTypes {
                    fn do_change(&self, starting_state: &$state, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
                        info!("Creating UI event for {:?}.", &entity);
                        match self {
                            $(
                                UiChangeTypes::$enum_variant { value } => {
                                    $update_fn(*value, starting_state, entity, style_context)
                                }
                            )*
                            _ => {
                                None
                            }
                        }
                    }
            }
    }
}

updates!(
    Style,
    do_remove_visible, RemoveVisible,
    do_add_visible, AddVisible,
    do_change_visible, ChangeVisible,
    create_update_size_value, UpdateSize,
    create_change_size_value, ChangeSize,
    create_drag_x, DragXPosition
);

#[derive(Clone, Debug)]
pub struct TextEventArgsFactory;
impl TextEventArgsFactory {
    fn do_create_ui_event(&self, starting_state: &Text, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
        todo!()
    }

    fn get_change(&self, starting_state: &Text, entity: Entity, style_context: &mut ResMut<UiContext>) -> Text {
        todo!()
    }
}

fn do_remove_visible(value: (), starting_state: &Style, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
    if starting_state.display != Display::None {
        let mut s = starting_state.clone();
        s.display = Display::None;
        return Some(UiEventArgs::Event(UiClickStateChange::ChangeDisplay {
            entity,
            update_display: Update {
                update_to: Some(Display::None),
            }}
        ))
    }
    None
}

fn do_add_visible(value: (), starting_state: &Style, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
    if starting_state.display != Display::Flex {
        let mut s = starting_state.clone();
        s.display = Display::None;
        return Some(UiEventArgs::Event(UiClickStateChange::ChangeDisplay {
            entity,
            update_display: Update {
                update_to: Some(Display::Flex),
            }}
        ))
    }
    None
}

fn do_change_visible(value: (), starting_state: &Style, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
    if starting_state.display == Display::None {
        do_add_visible(value, starting_state, entity, style_context)
    } else if starting_state.display == Display::Flex {
        do_remove_visible(value, starting_state, entity, style_context)
    } else {
        None
    }
}

fn create_update_size_value(value: (f32, f32), starting_state: &Style, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
    let created = do_create_updates(
        starting_state,
        &|style| Some(Size::new(Val::Percent(value.0), Val::Percent(value.1))),
    );
    if created.is_none() {
        return None;
    }
    do_create_change_size_ui_event(created.unwrap(), entity)
}

fn create_change_size_value(value: (f32, f32, f32, f32), starting_state: &Style, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
    create_change_size(starting_state, entity, value.0, value.1, value.2, value.3)
}

fn create_drag_x(value: (), starting_state: &Style, entity: Entity, style_context: &mut ResMut<UiContext>) -> Option<UiEventArgs> {
    if style_context.is_dragging.is_none() || style_context.delta.is_none() {
        return None;
    }
    let mut style = starting_state.clone();
    let updated_drag = match &style.left {
        Val::Px(mut px) => {
            let prev = px;
            px += style_context.delta.unwrap().x;
            style_context.delta = None;
            let pos = UiRect::new(Val::Px(px), style.right, style.top, style.bottom);
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
            entity,
        }))
    }).flatten();
    updated_drag
}

fn create_update_size(starting_state: &Style, entity: Entity, width_1: f32, height_1: f32) -> Option<UiEventArgs> {
    let created = do_create_updates(
        starting_state,
        &|style| Some(Size::new(Val::Percent(height_1), Val::Percent(width_1))),
    );
    if created.is_none() {
        return None;
    }
    do_create_change_size_ui_event(created.unwrap(), entity)
}

fn create_change_size(starting_state: &Style, entity: Entity, width_1: f32, width_2: f32, height_1: f32, height_2: f32) -> Option<UiEventArgs> {
    let created = do_create_updates(
        starting_state,
        &|style| size(height_1, height_2, width_1, width_2, style),
    );
    if created.is_none() {
        return None;
    }
    do_create_change_size_ui_event(created.unwrap(), entity)
}

pub(crate) fn size(height_1: f32, height_2: f32, width_1: f32, width_2: f32, starting_state: &Style) -> Option<Size> {
    let mut size = None;
    info!("{:?} is starting width and {:?} is starting height, \
    and height_1: {}, height_2: {}, width_1: {}, width_2: {}", &starting_state.width, &starting_state.height, height_1, height_2, width_1, width_2);
    if let Val::Percent(height) = starting_state.height {
        info!("{} is height and {} is height_1", height, height_1);
        if height == height_1 {
            if let Val::Percent(width) = starting_state.width {
                info!("{} is width and {} is width_1", width, width_1);
                if width == width_1 {
                    return Some(Size::new(Val::Percent(width_2), Val::Percent(height_2)));
                } else if height_1 == height_2 {
                    if width == width_2 {
                        return Some(Size::new(Val::Percent(width_1), Val::Percent(height_1)));
                    }
                }
            }
        } else if height == height_2 {
            if let Val::Percent(width) = starting_state.width {
                if width == width_2 {
                    return Some(Size::new(Val::Percent(width_1), Val::Percent(height_1)));
                } else if height_1 == height_2 {
                    info!("{} is width and {} is width_1", width, width_1);
                    if width == width_1 {
                        return Some(Size::new(Val::Percent(width_2), Val::Percent(height_2)));
                    }
                }
            }
        }
    }
    info!("Sizes did not match");
    size
}

/// Need to translate StyleChangeType to UiComponentFilters, and then pass the UiComponentFilters to
/// a general function or trait to execute behavior for all UiComponentFilters.
/// UiComponentType (Type of node) -> UiComponentFilter
/// UiComponentFilter -> ChangeStyleBehavior
fn do_create_change_size_ui_event(update_display: Update<Size>, entity: Entity) -> Option<UiEventArgs> {
    return Some(UiEventArgs::Event(UiClickStateChange::ChangeSize {
        update_display,
        entity,
    }));
}

fn do_create_display_ui_event(update_display: Update<Display>, entity: Entity) -> Option<UiEventArgs> {
    return Some(UiEventArgs::Event(UiClickStateChange::ChangeDisplay {
        update_display,
        entity,
    }));
}

fn do_create_updates<T: Clone + Debug + Default + Send + Sync>(
    to_update: &Style,
    get_next_state_component: &dyn Fn(&Style) -> Option<T>,
) -> Option<Update<T>> {
    let opt: Option<T> = get_next_state_component(to_update);
    opt.map(|update_to| Update { update_to: Some(update_to) })
}

