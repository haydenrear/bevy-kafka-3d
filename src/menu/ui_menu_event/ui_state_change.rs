use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Button, Changed, Color, Component, Display, Entity, Interaction, Query, ResMut, Resource, Size, Style, With};
use bevy::log::info;
use bevy::ui::UiRect;
use bevy_transform::prelude::Transform;
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::math::Vec2;
use bevy::input::mouse::MouseScrollUnit;
use crate::event::event_descriptor::{EventArgs, EventData};
use crate::event::event_state::{ClickContext, Context, StyleStateChangeEventData, Update};
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::menu::{Position, UiComponent};
use crate::menu::ui_menu_event::change_style::{DoChange, UiChangeTypes};
use crate::menu::ui_menu_event::interaction_ui_event_writer::ClickSelectOptions;
use crate::menu::ui_menu_event::next_action::{DisplayState, UiComponentState};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::types::{ClickEvents, ClickSelectionEventRetriever, DraggableStateChangeRetriever, DraggableUiComponentFilter, DraggableUiComponentIxnFilter, ScrollableStateChangeRetriever, ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter, UiComponentStyleFilter, UiComponentStyleIxnFilter};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiEventArgs};

/// Contains the state data needed in order to generate the UIEvents from the state change required.
#[derive(Clone, Debug)]
pub enum UiClickStateChange {
    ChangeColor {
        entity: Entity,
        update_display: Update<BackgroundColor>,
    },
    ChangeDisplay {
        entity: Entity,
        update_display: Update<Display>,
    },
    ChangeSize {
        entity: Entity,
        update_display: Update<Size>,
    },
    Slider {
        entity: Entity,
        update_scroll: Update<UiRect>
    },
    None,
}

pub trait StateChangeMachine<ComponentT, Ctx: Context, EventArgsT: EventArgs>: Send + Sync + 'static + EventData {
    fn state_machine_event(
        &self,
        starting: &ComponentT,
        style_context: &mut ResMut<Ctx>,
        entity: Entity
    ) -> Option<EventArgsT>;
}

impl StateChangeMachine<Style, UiContext, UiEventArgs> for StyleStateChangeEventData {
    fn state_machine_event(&self, starting: &Style, style_context: &mut ResMut<UiContext>, entity: Entity) -> Option<UiEventArgs> {
        if let StyleStateChangeEventData::ChangeComponentStyle(change_style) = self {
            return change_style.do_change(starting, entity, style_context);
        }
        None
    }
}

impl StyleStateChangeEventData {

    pub fn get_ui_event(
        &self,
        starting: &Style,
        style_context: &mut ResMut<UiContext>,
        entity: Entity
    ) -> Option<UiEventArgs> {
        if let StyleStateChangeEventData::ChangeComponentStyle(change_style) = self {
            return change_style.do_change(starting, entity, style_context);
        }
        None
    }

}

pub fn hover_event(
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
                // color.0 = Color::GREEN;
            }
        }
    }
}

#[derive(Resource, Debug)]
pub struct GlobalState
{
    pub(crate) cursor_pos: Vec2,
    pub(crate) cursor_delta: Vec2,
    pub(crate) click_hover_ui: bool,
    pub(crate) hover_ui: bool,
    pub(crate) scroll_wheel_delta: Vec2,
    pub(crate) wheel_units: Option<MouseScrollUnit>
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            cursor_pos: Default::default(),
            cursor_delta: Default::default(),
            click_hover_ui: false,
            hover_ui: false,
            scroll_wheel_delta: Default::default(),
            wheel_units: None,
        }
    }
}

pub trait UpdateGlobalState<SELF, IXN>: Resource + Send + Sync
where SELF: ReadOnlyWorldQuery,
      IXN: ReadOnlyWorldQuery
{
    fn update_wheel(resource: &mut GlobalState, event: Vec2, wheel_units: Option<MouseScrollUnit>) {
        let mut prev: Vec2 = Vec2::new(event.x, event.y);
        std::mem::swap(&mut prev, &mut resource.scroll_wheel_delta);
        info!("{:?} is scroll wheel delta", &resource.scroll_wheel_delta);
        resource.wheel_units = wheel_units;
        if prev != Vec2::ZERO {
            let delta = resource.cursor_pos - prev;
            resource.scroll_wheel_delta = delta;
        }
    }

    fn update_cursor(resource: &mut GlobalState, cursor_pos: Vec2) {
        let mut prev: Vec2 = cursor_pos;
        std::mem::swap(&mut prev, &mut resource.cursor_pos);
        if prev != Vec2::ZERO {
            let delta = resource.cursor_pos - prev;
            resource.cursor_delta = delta;
        }
    }

    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
        resource.hover_ui = hover_ui;
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
        resource.hover_ui = hover_ui;
    }
    fn click_hover_ui(resources: &mut GlobalState) -> bool {
        resources.click_hover_ui
    }

}

impl UpdateGlobalState<UiComponentStyleFilter, UiComponentStyleIxnFilter>
for ClickEvents {}

impl UpdateGlobalState<UiComponentStyleFilter, UiComponentStyleIxnFilter>
for ClickSelectionEventRetriever {}

impl UpdateGlobalState<ScrollableUiComponentFilter, ScrollableUiComponentIxnFilter>
for ScrollableStateChangeRetriever  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }
}

impl UpdateGlobalState<DraggableUiComponentFilter, DraggableUiComponentIxnFilter>
for DraggableStateChangeRetriever  {
    fn update_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }

    fn update_click_hover_ui(resource: &mut GlobalState, hover_ui: bool) {
    }
}
