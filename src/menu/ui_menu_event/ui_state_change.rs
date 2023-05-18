use bevy::utils::HashMap;
use bevy::prelude::{BackgroundColor, Button, Changed, Color, Component, Display, Entity, Interaction, Query, ResMut, Size, Style, With};
use bevy::log::info;
use bevy::ui::UiRect;
use bevy_transform::prelude::Transform;
use crate::event::event_descriptor::EventArgs;
use crate::event::event_state::{ClickContext, Context, StyleStateChangeEventData, Update};
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::menu::{Position, UiComponent};
use crate::menu::ui_menu_event::change_style::{ChangeStyleTypes};
use crate::menu::ui_menu_event::next_action::{DisplayState, UiComponentState};
use crate::menu::ui_menu_event::style_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{UiComponentStateFilter, UiEventArgs};

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

pub trait StateChangeMachine<S, Ctx: Context, Args: EventArgs>: Send + Sync + 'static {
    fn state_machine_event(
        &self,
        starting: &S,
        style_context: &mut ResMut<Ctx>,
        entity: Entity
    ) -> Option<Args>;
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
