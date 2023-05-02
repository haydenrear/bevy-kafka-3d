use bevy::prelude::*;
use crate::menu::config_menu_event::network_component_interaction_system::update_components_selected;


pub struct ConfigMenuEventPlugin;

impl Plugin for ConfigMenuEventPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(update_components_selected::<crate::network::Node>);
    }
}
