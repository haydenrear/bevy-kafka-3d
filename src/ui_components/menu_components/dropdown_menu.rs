use std::default::default;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::menu::{ConfigurationOptionEnum, MenuItemMetadata, MenuOption, MenuOptionType, UiComponent};
use crate::ui_components::menu_components::base_menu::{BaseMenu, BuildBaseMenuResult};
use crate::ui_components::menu_components::{add_config_opt, BuilderResult, do_submenu_menu_building};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::{SelectionMenuOptionBuilder, DropdownMenuOptionResult};
use crate::ui_components::menu_components::menu_options::MenuOptionBuilder;
use crate::ui_components::menu_components::menu_options::slider_menu_option::SliderMenuOptionResult;
use crate::ui_components::menu_components::submenu_builder::{DrawSubmenuResult, SubmenuBuilder};
use crate::ui_components::ui_menu_component::insert_config_option;

pub struct DropdownMenuBuilder<'a> {
    pub(crate) menu_metadata: &'a MenuItemMetadata,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parent_menus: Vec<MenuItemMetadata>,
    pub(crate) base_menu: BaseMenu<'a>,
    pub(crate) menu_option_builders: Vec<(MenuOption, MenuOptionBuilder<'a>)>,
}

#[derive(Default, Clone, Debug)]
pub struct DrawDropdownMenuResult {
    pub(crate) dropdown_menu_option_results: Vec<DropdownMenuOptionResult>,
    pub(crate) submenu_results: Vec<DrawSubmenuResult>,
    pub(crate) slider: Vec<SliderMenuOptionResult>,
    pub(crate) base_menu_result: BuildBaseMenuResult
}

impl DrawDropdownMenuResult {
    pub(crate) fn get_direct_children(&self) -> Vec<Entity> {
        let mut menu_options = self.dropdown_menu_option_results.iter()
            .flat_map(|e| e.menu_option_entity.into_iter())
            .collect::<Vec<Entity>>();
        self.submenu_results.iter()
            .flat_map(|submenu| submenu.dropdown_menu_result.base_menu_result.base_menu_parent.iter())
            .for_each(|submenu| menu_options.push(*submenu));
        self.slider.iter()
            .for_each(|slider| menu_options.push(slider.slider_entity));
        menu_options
    }

}

impl BuilderResult for DrawDropdownMenuResult {}

impl DrawDropdownMenuResult {
    fn new(base_menu_result: BuildBaseMenuResult) -> Self {
        Self {
            base_menu_result,
            ..default()
        }
    }
}

pub(crate) fn set_parent(mut commands: &mut Commands, entity: Option<Entity>, parent: Option<Entity>) {
    parent.map(|parent| {
        commands.get_entity(parent).as_mut()
            .map(|parent| entity.map(|entity| {
                parent.add_child(entity);
            }));
    });
}

impl <'a> DropdownMenuBuilder<'a>{
    pub(crate) fn build(&'a mut self, mut commands: &mut Commands,
                        mut materials: &mut ResMut<Assets<ColorMaterial>>,
                        mut meshes: &mut ResMut<Assets<Mesh>>,
                        mut asset_server: &mut Res<AssetServer>,
    ) -> DrawDropdownMenuResult {
        let base_menu_result = self.base_menu.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
        let mut draw_dropdown = DrawDropdownMenuResult::new(base_menu_result.clone());

        let (submenu, menu_option, slider)
            = do_submenu_menu_building(
            &mut commands,
            &mut self.menu_option_builders,
            base_menu_result.base_menu_parent.clone(),
            materials,
            meshes,
            asset_server
        );

        draw_dropdown.submenu_results = submenu;
        draw_dropdown.dropdown_menu_option_results = menu_option;
        draw_dropdown.slider = slider;

        add_config_opt(commands, base_menu_result.base_menu_parent.clone(), self.config_option);

        draw_dropdown
    }
}

