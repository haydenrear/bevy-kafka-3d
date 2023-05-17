use std::default::default;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::menu::{ConfigurationOptionEnum, MenuItemMetadata, MenuOption, MenuOptionType, UiComponent};
use crate::ui_components::ui_components::base_menu::{BaseMenu, BuildBaseMenuResult};
use crate::ui_components::ui_components::{add_config_opt, BuilderResult, do_submenu_menu_building};
use crate::ui_components::ui_components::submenu_builder::{DrawSubmenuResult, SubmenuBuilder};
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct RootNodeBuilder {
}

#[derive(Clone, Debug)]
pub struct DrawRootNodeResult {
    pub(crate) root: Entity
}

impl BuilderResult for DrawRootNodeResult {}

impl  RootNodeBuilder {
    pub(crate) fn build(&mut self, mut commands: &mut Commands,
                        mut materials: &mut ResMut<Assets<ColorMaterial>>,
                        mut meshes: &mut ResMut<Assets<Mesh>>,
                        mut asset_server: &mut Res<AssetServer>,) -> DrawRootNodeResult {
        let root =commands.spawn(self.root_node())
            .id();
        info!("Root node: {:?}", root);
        DrawRootNodeResult {
            root
        }
    }

    pub(crate) fn root_node(&self) -> impl Bundle {
        (
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            UiIdentifiableComponent(0.0),
            UiComponent::Node
        )
    }

}

