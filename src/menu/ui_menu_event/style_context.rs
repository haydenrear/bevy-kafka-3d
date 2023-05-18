use bevy::prelude::{Entity, ResMut, Resource, Style};
use bevy::utils::HashMap;
use bevy::math::Vec2;
use bevy::input::mouse::MouseScrollUnit;
use crate::event::event_state::{ClickContext, Context};
use crate::menu::ui_menu_event::interaction_ui_event_writer::GlobalState;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter};

#[derive(Resource, Default, Clone, Debug)]
pub struct UiContext {
    pub(crate) visible: HashMap<Entity, Style>,
    pub(crate) is_dragging: bool,
    pub(crate) delta: Option<Vec2>,
    pub(crate) scroll_wheel: Option<Vec2>,
    pub(crate) scroll_wheel_units: Option<MouseScrollUnit>,
}

impl Context for UiContext {}

impl ClickContext<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter> for UiContext {
    fn clicked(&mut self) {
        self.is_dragging = true;
    }

    fn un_clicked(&mut self) {
        self.is_dragging = false;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}

impl ClickContext<UiComponentStyleFilter, UiComponentStyleIxnFilter> for UiContext {
    fn clicked(&mut self) {
        self.is_dragging = true;
    }

    fn un_clicked(&mut self) {
        self.is_dragging = false;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}

impl UiContext {
    fn set_values(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}

impl ClickContext<DraggableUiComponentFilter, DraggableUiComponentIxnFilter> for UiContext {
    fn clicked(&mut self) {
        self.is_dragging = true;
    }

    fn un_clicked(&mut self) {
        self.is_dragging = false;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}
