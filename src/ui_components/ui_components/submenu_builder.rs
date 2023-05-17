use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::menu::{ConfigurationOptionEnum, Dropdown, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionType, UiComponent};
use crate::ui_components::ui_components::base_menu::BaseMenu;
use crate::ui_components::ui_components::BuilderResult;
use crate::ui_components::ui_components::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::ui_menu_component::menu_options;

pub struct SubmenuBuilder<'a> {
    pub(crate) parent: Option<Entity>,
    pub(crate) menu_metadata: MenuItemMetadata,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parent_menus: Vec<MenuItemMetadata>,
    pub(crate) menu_component: UiComponent,
    pub(crate) sub_menu: &'a MenuInputType,
}

#[derive(Default, Clone, Debug)]
pub struct DrawSubmenuResult {
    pub(crate) dropdown_menu_result: DrawDropdownMenuResult,
    with_submenu_added: Vec<MenuItemMetadata>
}

impl BuilderResult for DrawSubmenuResult {}

impl <'a> SubmenuBuilder<'a> {
    pub(crate) fn build(
        &mut self,
        mut commands: &mut Commands,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut asset_server: &mut Res<AssetServer>,
    ) -> Option<DrawSubmenuResult> {
        match self.sub_menu {
            MenuInputType::Dropdown { options, metadata, option } => {
                self.parent_menus.push(metadata.clone());
                let base_menu = BaseMenu {
                    menu_metadata: metadata,
                    config_option: option,
                    parent_menus: self.parent_menus.clone(),
                    component: UiComponent::Dropdown(Dropdown {
                        selected_index: 0,
                        options: options.iter()
                            .map(|opt| opt.metadata.name.clone())
                            .collect(),
                    }),
                    parent: self.parent.unwrap(),
                };
                let mut dropdown_menu_builder = DropdownMenuBuilder {
                    menu_metadata: metadata,
                    config_option: option,
                    parent_menus: self.parent_menus.clone(),
                    base_menu,
                    menu_option_builders: menu_options(options, self.parent_menus.clone()),
                };
                let result = dropdown_menu_builder
                    .build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
                self.parent_menus.push(metadata.clone());
                Some(DrawSubmenuResult {
                    dropdown_menu_result: result,
                    with_submenu_added: self.parent_menus.clone(),
                })
            },
            _ => {
                None
            }
        }
    }
}

