use bevy_ui::Val;
use crate::menu::MenuOption;

pub(crate) mod ui_menu_component;
pub(crate) mod menu_components;

#[derive(Debug, Clone, Default)]
pub struct Size {
    pub(crate) height: Val,
    pub(crate) width: Val
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    pub(crate) right: Val,
    pub(crate) left: Val,
    pub(crate) top: Val,
    pub(crate) bottom: Val
}

impl Position {
    pub(crate) fn new(right: Val, left: Val, top: Val, bottom: Val) -> Self {
        Self {
            right, left, top, bottom
        }
    }
}

impl Size {
    pub(crate) fn new(height: Val, width: Val) -> Self {
        Self {
            height, width
        }
    }
}

pub(crate) fn get_menu_option_names(options: &Vec<MenuOption>) -> Vec<String> {
    options.iter()
        .map(|opt| opt.metadata.name.to_string())
        .collect()
}
