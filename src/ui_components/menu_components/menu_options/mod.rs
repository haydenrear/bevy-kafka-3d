use bevy::prelude::Commands;
use crate::menu::{ConfigurationOptionEnum, Dropdown, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionInputType, UiComponent};
use crate::ui_components::menu_components::menu_types::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::{DropdownMenuOptionResult, DropdownMenuOptionBuilder};
use crate::ui_components::menu_components::menu_options::slider_menu_option::SliderMenuOptionBuilder;
use crate::ui_components::menu_components::menu_types::submenu_builder::SubmenuBuilder;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub(crate) mod dropdown_menu_option;
pub(crate) mod slider_menu_option;

pub enum MenuOptionBuilder<'a> {
    DropdownMenuOptionBuilder(DropdownMenuOptionBuilder<'a>),
    SliderMenuOptionBuilder(SliderMenuOptionBuilder<'a>),
    SubmenuBuilder(SubmenuBuilder<'a>)
}