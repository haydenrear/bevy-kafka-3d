use bevy::prelude::{Commands, Component, Display, Entity, ResMut, Size, Style, Text, UiRect, Val, Visibility};
use bevy::log::info;
use crate::cursor_adapter::PickableComponent;
use crate::event::event_state::{Update, UpdateStateInPlace};
use crate::menu::{Menu, MetricsConfigurationOption};
use crate::menu::graphing_menu::graph_menu::{ChangeGraphingMenu, GraphMenuPotential};
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::pickable_events::PickableComponentState;

#[derive(Debug)]
pub enum NextUiState {
    ReplaceSize(Update<Size>),
    ReplaceDisplay(Update<Display>),
    UpdatePosition(Update<UiRect>),
    UpdateSelection(Update<Text>)
}

impl UpdateStateInPlace<Style, UiContext> for NextUiState {
    fn update_state(&self, commands: &mut Commands,  value: &mut Style, style_context: &mut ResMut<UiContext>) {
        match &self {
            NextUiState::ReplaceSize(update) => update.update_state(commands, &mut value.size, style_context),
            NextUiState::ReplaceDisplay(display) => display.update_state(commands, &mut value.display, style_context),
            NextUiState::UpdatePosition(update) => update.update_state(commands, &mut value.position ,style_context),
            _ => {}
        }
    }
}

impl UpdateStateInPlace<Text, UiContext> for NextUiState {
    fn update_state(&self, commands: &mut Commands, mut value: &mut Text, style_context: &mut ResMut<UiContext>) {
        match &self {
            Self::UpdateSelection(update) => update.update_state(commands, &mut value, style_context),
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum UiComponentState {
    StateDisplay(DisplayState),
    StateSize(SizeState),
    StateVisible(VisibilityIdentifier),
    Selected,
    Deselected,
    Any
}

impl Matches<Style> for UiComponentState {
    fn matches(&self, style: &Style) -> bool {
        match self {
            UiComponentState::StateDisplay(display) => display.matches(&style.display),
            UiComponentState::StateSize(state) => state.matches(&style.size),
            UiComponentState::Any => true,
            other => {
                info!("Did not match: {:?}", other);
                false
            }
        }
    }
}

impl Matches<Visibility> for UiComponentState {
    fn matches(&self, style: &Visibility) -> bool {
        match self {
            UiComponentState::StateVisible(visible) => {
                if style == Visibility::Visible && matches!(visible, VisibilityIdentifier::Visible) {
                    true
                } else if style == Visibility::Hidden && matches!(visible, VisibilityIdentifier::Hidden)  {
                    true
                } else if matches!(visible, VisibilityIdentifier::Any) {
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum DisplayState {
    DisplayFlex,
    DisplayNone,
    DisplayAny,
}

#[derive(Debug, Eq, PartialEq)]
pub enum VisibilityIdentifier {
    Visible,
    Hidden,
    Any
}

#[derive(Debug, Eq, PartialEq)]
pub enum SizeState {
    Expanded{
        height: u32,
        width: u32,
    }, Minimized {
        height: u32,
        width: u32,
    }
}

impl DisplayState {
    fn get_display(&self) -> Display {
        match self {
            DisplayState::DisplayFlex => {
                Display::Flex
            }
            DisplayState::DisplayNone => {
                Display::None
            }
            DisplayState::DisplayAny => {
                Display::Flex
            }
        }
    }
}

pub trait Matches<T> : Send + Sync + 'static{
    fn matches(&self, other: &T) -> bool;
}

impl Matches<Display> for DisplayState {
    fn matches(&self, other: &Display) -> bool {
        if let DisplayState::DisplayAny = self {
            return true;
        }
        if self.get_display() == *other  {
            return true;
        } else {
            return false;
        }
    }
}

impl Matches<GraphMenuPotential> for PickableComponentState {
    fn matches(&self, other: &GraphMenuPotential) -> bool {
        true
    }
}

impl Matches<ChangeGraphingMenu> for PickableComponentState {
    fn matches(&self, other: &ChangeGraphingMenu) -> bool {
        true
    }
}

impl SizeState {

    fn get_width_height(&self) -> (u32, u32) {
        match self  {
            SizeState::Expanded { height, width } => {
                (*height, *width)
            }
            SizeState::Minimized { height, width } => {
                (*height, *width)
            }
        }
    }

}

impl Matches<Size> for SizeState {
    fn matches(&self, starting_state: &Size) -> bool {
        let (height_state, width_state) = self.get_width_height();
        info!("{} is height and {} is width, and {:?} is starting_state.", height_state, width_state, starting_state);
        if let Val::Percent(height) = starting_state.height {
            if let Val::Percent(width) = starting_state.width {
                info!("{} is match height and {} is match width.", height, width);
                if height as u32 == height_state && width as u32 == width_state {
                    info!("matched");
                    return true;
                }
                return false;
            }
        }
        if let Val::Px(height) = starting_state.height {
            if let Val::Px(width) = starting_state.width {
                info!("{} is match height and {} is match width.", height, width);
                info!("{} is match height and {} is match width.", height, width);
                if height as u32 == height_state && width as u32 == width_state {
                    info!("matched");
                    return true;
                }
                return false;
            }
        }
        false
    }
}
