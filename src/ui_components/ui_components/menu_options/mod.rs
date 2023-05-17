use bevy::prelude::Commands;
use crate::menu::{ConfigurationOptionEnum, Dropdown, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionInputType, UiComponent};
use crate::ui_components::ui_components::base_menu::BaseMenu;
use crate::ui_components::ui_components::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::ui_components::menu_options::dropdown_menu_option::{SelectionMenuOptionBuilderResult, SelectionMenuOptionBuilder};
use crate::ui_components::ui_components::menu_options::slider_menu_option::SliderMenuOptionBuilder;
use crate::ui_components::ui_components::submenu_builder::SubmenuBuilder;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub(crate) mod dropdown_menu_option;
pub(crate) mod slider_menu_option;

pub enum MenuOptionBuilder<'a> {
    SelectionOptionBuilder(SelectionMenuOptionBuilder<'a>),
    SliderMenuOptionBuilder(SliderMenuOptionBuilder<'a>),
    SubmenuBuilder(SubmenuBuilder<'a>)
}