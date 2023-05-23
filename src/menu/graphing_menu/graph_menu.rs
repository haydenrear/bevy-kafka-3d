use bevy::prelude::{Component, Entity, Resource};
use crate::graph::GraphDim;
use crate::menu::MenuData;
use crate::menu::ui_menu_event::ui_state_change::StateAdviser;

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
    pub(crate) realized: bool
}

impl StateAdviser<ChangeGraphingMenu> for GraphMenuPotential {
    fn advise(&self, in_state: &ChangeGraphingMenu) -> ChangeGraphingMenu {
        if !self.realized {
            ChangeGraphingMenu::AddGraphingMenu
        } else {
            ChangeGraphingMenu::RemoveGraphingMenu
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub enum ChangeGraphingMenu {
    AddGraphingMenu,
    RemoveGraphingMenu
}