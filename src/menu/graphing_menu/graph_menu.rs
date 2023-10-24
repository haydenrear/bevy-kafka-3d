use bevy::log::info;
use bevy::prelude::{Bundle, Commands, Component, Entity, Resource};
use crate::graph::GraphDim;
use crate::menu::{ConfigurationOptionEnum, MenuData, MetricsConfigurationOption};
use crate::menu::ui_menu_event::ui_state_change::StateAdviser;
use crate::network::Node;

#[derive(Resource)]
pub struct GraphBuilder {
    graph_name: String,
    metric: Entity,
    dimensions: Vec<GraphDim>
}

#[derive(Resource)]
pub struct GraphBuilders {
    graph_builders: Vec<GraphBuilder>
}

#[derive(Resource)]
pub struct GraphMenuResource {
    pub(crate) data: MenuData
}

#[derive(Component, Default, Debug, Clone)]
pub struct GraphMenuPotential {
    pub(crate) realized: bool,
    pub(crate) menu_options: Vec<ConfigurationOptionEnum>
}

impl StateAdviser<ChangeGraphingMenu> for GraphMenuPotential {
    fn advise(&self, mut commands: &mut Commands, in_state: &ChangeGraphingMenu) -> ChangeGraphingMenu {
        if !self.realized {
            info!("Advising to remove graphing menu.");
            let menu = commands.spawn(()).id();
            return ChangeGraphingMenu::RemoveGraphingMenu(menu);
        } else if let ChangeGraphingMenu::RemoveGraphingMenu(entity) = in_state {
            info!("Removing graphing menu.");
            commands.entity(*entity)
                .remove::<Node>();
        }
        info!("Returning to add graphing menu.");
        return ChangeGraphingMenu::AddGraphingMenu;
    }
}

#[derive(Component, Clone, Debug, Copy)]
pub enum ChangeGraphingMenu {
    AddGraphingMenu,
    RemoveGraphingMenu(Entity)
}