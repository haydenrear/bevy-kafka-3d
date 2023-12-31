use bevy::prelude::{Entity, ResMut, Resource};
use bevy::math::Vec2;
use bevy::input::mouse::MouseScrollUnit;
use crate::event::event_state::{ClickContext, Context};
use crate::graph::Graph;
use crate::menu::graphing_menu::graph_menu::GraphMenuPotential;
use crate::menu::ui_menu_event::type_alias::event_reader_writer_filter::{DraggableUiComponentFilter, DraggableUiComponentIxnFilter, PickableFilter, PickableIxnFilter, ScrollableIxnFilterQuery, ScrollableUiComponentFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter, VisibleFilter, VisibleIxnFilter};
use crate::menu::ui_menu_event::ui_state_change::{ChangeVisible, GlobalState};

#[derive(Resource, Default, Clone, Debug)]
pub struct UiContext {
    pub(crate) is_dragging: Option<Entity>,
    pub(crate) scroll_context: Option<Entity>,
    pub(crate) delta: Option<Vec2>,
    pub(crate) scroll_wheel: Option<Vec2>,
    pub(crate) scroll_wheel_units: Option<MouseScrollUnit>,
}

impl Context for UiContext {}

impl ClickContext<ScrollableUiComponentFilter, ScrollableIxnFilterQuery> for UiContext {
    fn clicked(&mut self, entity: Entity) {
        self.is_dragging = Some(entity);
    }

    fn un_clicked(&mut self) {
        self.is_dragging = None;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}

impl ClickContext<UiComponentStyleFilter, UiComponentStyleIxnFilter> for UiContext {
    fn clicked(&mut self, entity: Entity) {
        self.is_dragging = Some(entity);
    }

    fn un_clicked(&mut self) {
        self.is_dragging = None;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}

impl UiContext {
    fn set_values(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
    fn clicked_inner(&mut self, entity: Entity) {
        self.is_dragging = Some(entity);
    }

    fn un_clicked_inner(&mut self) {
        self.is_dragging = None;
    }

    fn cursor_inner(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}


impl ClickContext<DraggableUiComponentFilter, DraggableUiComponentIxnFilter> for UiContext {
    fn clicked(&mut self, entity: Entity) {
        self.clicked_inner(entity);
    }

    fn un_clicked(&mut self) {
        self.un_clicked_inner();
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }
}

impl <T: ChangeVisible> ClickContext<VisibleFilter<T>, VisibleIxnFilter<T>> for UiContext {

    fn clicked(&mut self, entity: Entity) {
        self.is_dragging = Some(entity);
    }

    fn un_clicked(&mut self) {
        self.is_dragging = None;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }

}

impl ClickContext<PickableFilter<GraphMenuPotential>, PickableIxnFilter<GraphMenuPotential>> for UiContext {

    fn clicked(&mut self, entity: Entity) {
        self.is_dragging = Some(entity);
    }

    fn un_clicked(&mut self) {
        self.is_dragging = None;
    }

    fn cursor(&mut self, cursor_moved: &mut ResMut<GlobalState>) {
        self.set_values(cursor_moved);
    }

}