use crate::menu::MenuOption;

pub(crate) mod ui_menu_component;
pub(crate) mod menu_components;

pub(crate) fn get_menu_option_names(options: &Vec<MenuOption>) -> Vec<String> {
    options.iter()
        .map(|opt| opt.metadata.name.to_string())
        .collect()
}
