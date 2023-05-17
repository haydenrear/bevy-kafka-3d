use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::menu::{ConfigurationOptionEnum, MenuOption, MenuOptionInputType, MenuOptionType};
use crate::ui_components::menu_components::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::collapsable_menu::{CollapsableMenuBuilder, DrawCollapsableMenuResult};
use crate::ui_components::menu_components::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::{SelectionMenuOptionBuilder, DropdownMenuOptionResult};
use crate::ui_components::menu_components::menu_options::MenuOptionBuilder;
use crate::ui_components::menu_components::menu_options::slider_menu_option::{SliderMenuOptionBuilder, SliderMenuOptionResult};
use crate::ui_components::menu_components::root_collapsable::{DrawRootNodeResult, RootNodeBuilder};
use crate::ui_components::menu_components::submenu_builder::{DrawSubmenuResult, SubmenuBuilder};
use crate::ui_components::ui_menu_component::insert_config_option;

pub(crate) mod dropdown_menu;
pub(crate) mod base_menu;
pub(crate) mod submenu_builder;
pub(crate) mod collapsable_menu;
pub(crate) mod root_collapsable;
pub(crate) mod menu_options;

pub trait BuilderResult {}

#[derive(Resource, Default, Debug)]
pub struct BuildMenuResult {
    pub(crate) root: HashMap<Entity, DrawRootNodeResult>,
    pub(crate) collapsable: HashMap<Entity, DrawCollapsableMenuResult>,
    pub(crate) dropdown: HashMap<Entity, DrawDropdownMenuResult>,
    pub(crate) dropdown_menu_option_results: HashMap<Entity, DropdownMenuOptionResult>,
    pub(crate) submenu_results:  Vec<DrawSubmenuResult>,
    pub(crate) base_menu_results: HashMap<Entity, BuildBaseMenuResult>,
    pub(crate) slider: HashMap<Entity, SliderMenuOptionResult>,
}

pub struct MenuBuilder<'a> {
    pub(crate) root: Vec<RootNodeBuilder>,
    pub(crate) collapsable: Vec<CollapsableMenuBuilder<'a>>,
    pub(crate) dropdown: Vec<DropdownMenuBuilder<'a>>,
    pub(crate) dropdown_menu_option_results: Vec<DropdownMenuBuilder<'a>>,
    pub(crate) submenu_results: Vec<SubmenuBuilder<'a>>,
    pub(crate) slider: Vec<SliderMenuOptionBuilder<'a>>,
}

fn add_config_opt(mut commands: &mut Commands, base_menu_result: Option<Entity>, config_option: &ConfigurationOptionEnum) {
    base_menu_result.map(|parent|
        commands.get_entity(parent)
            .as_mut()
            .map(|mut entity| insert_config_option(config_option, &mut entity))
    );
}

pub(crate) fn get_swing_out(menu_option: &MenuOption) -> f32 {
    let swing_out = match menu_option.ui_option_type  {
        MenuOptionInputType::Selected => false,
        MenuOptionInputType::Radial => false,
        MenuOptionInputType::FormInput => false,
        MenuOptionInputType::Slider => false,
        MenuOptionInputType::DropdownMenu => true,
        MenuOptionInputType::CollapsableMenu => true,
        MenuOptionInputType::SubMenu => true
    };
    let mut swing_out_percentage = 0.0;
    if swing_out {
        swing_out_percentage = 100.0;
    }
    swing_out_percentage
}

fn do_submenu_menu_building<'a>(
    mut commands: &mut Commands,
    mut builders: &'a mut Vec<(MenuOption, MenuOptionBuilder<'a>)>,
    parent: Option<Entity>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut asset_server: &mut Res<AssetServer>,
) -> (Vec<DrawSubmenuResult>, Vec<DropdownMenuOptionResult>, Vec<SliderMenuOptionResult>) {
    let mut draw_submenu = vec![];
    let mut draw_menu_option = vec![];
    let mut slider_menu = vec![];
    for (option, builder) in builders.iter_mut() {
        match &option.data_type {
            MenuOptionType::Primitive(config_type) => {
                if let MenuOptionBuilder::SelectionOptionBuilder(builder) = builder {
                    builder.parent = parent;
                    let menu_option = builder.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
                    draw_menu_option.push(menu_option);
                } else if let MenuOptionBuilder::SliderMenuOptionBuilder(slider) = builder {
                    slider.parent = parent;
                    let menu_option = slider.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
                    slider_menu.push(menu_option);
                }
            }
            MenuOptionType::SubMenu { .. } => {
                if let MenuOptionBuilder::SubmenuBuilder(builder) = builder {
                    builder.parent = parent;
                    builder.build(&mut commands, &mut materials, &mut meshes, &mut asset_server)
                        .map(|submenu| draw_submenu.push(submenu));
                }
            }
        };
    }
    (draw_submenu, draw_menu_option, slider_menu)
}
