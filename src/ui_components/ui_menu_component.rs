use std::marker::PhantomData;
use bevy::prelude::{AlignSelf, BackgroundColor, Bundle, ButtonBundle, Color, ColorMaterial, Commands, Component, default, Display, Entity, FlexDirection, JustifyContent, Label, Mesh, NextState, NodeBundle, Query, Res, ResMut, Style, Text, TextBundle, TextStyle, UiRect, Val, Visibility, World};
use bevy::asset::{Assets, AssetServer};
use bevy::log::{error, info};
use bevy::hierarchy::BuildChildren;
use crate::event::event_state::{HoverStateChange, StyleStateChangeEventData, Update, UpdateStateInPlace};
use crate::event::event_state::StyleStateChangeEventData::ChangeComponentStyle;
use crate::menu::{CollapsableMenuComponent, ConfigurationOptionEnum, DataType, DraggableComponent, Dropdown, DropdownOption, Menu, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionInputType, MenuOptionType, MenuType, MetricsConfigurationOption, Radial, ScrollableMenuComponent, ScrollableMenuItemsBarComponent, ScrollingSidebarComponent, ScrollWheelComponent, SelectableType, Slider, SliderData, SliderKnob, UiComponent};
use crate::menu::menu_resource::{MENU, MenuResource};
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::UiComponent::CollapsableMenu;
use crate::ui_components::ui_menu_component;
use bevy::ecs::system::EntityCommands;
use bevy::ui::{AlignItems, FocusPolicy, ZIndex};
use crate::event::state_transition::state_transitions_plugin::TransitionsState;
use crate::menu::config_menu_event::interaction_config_event_writer::{GraphMenuResultBuilder, NetworkMenuResultBuilder};
use crate::menu::MenuInputType::CollapsableMenuInputType;
use crate::menu::ui_menu_event::next_action::{DisplayState, SizeState, UiComponentState};
use crate::ui_components;
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::menu_types::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::{DropdownMenuOptionBuilder, DropdownMenuOptionResult};
use crate::ui_components::menu_components::menu_options::MenuOptionBuilder;
use crate::ui_components::menu_components::menu_options::slider_menu_option::SliderMenuOptionResult;
use crate::ui_components::menu_components::menu_types::base_menu::BaseMenu;
use crate::ui_components::menu_components::menu_types::collapsable_menu::{CollapsableMenuBuilder, DrawCollapsableMenuResult};
use crate::ui_components::menu_components::menu_types::root_collapsable::RootNodeBuilder;
use crate::ui_components::menu_components::menu_types::submenu_builder::{DrawSubmenuResult, SubmenuBuilder};

#[derive(Component, Debug, Clone)]
pub struct UiIdentifiableComponent(pub f32);

pub fn create_menu(
    mut commands: Commands,
    mut build_result: ResMut<BuildMenuResult>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut menu_resource: Res<MenuResource>,
    mut menu_state: ResMut<NextState<TransitionsState>>,
) {
    let mut root_node_builder = RootNodeBuilder {};
    let root_node_result = root_node_builder.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
    let root_parent = root_node_result.root;
    build_result.root.insert(root_node_result.root, root_node_result);
    for item in menu_resource.menu_data.selectables.iter() {
        match item {
            MenuInputType::Dropdown { options, metadata, option } => {
                add_dropdown(
                    &mut commands,
                    &mut build_result,
                    &mut materials,
                    &mut meshes,
                    &mut asset_server,
                    root_parent,
                    options,
                    &metadata,
                    option
                );
            }
            MenuInputType::CollapsableMenuInputType { options, option, metadata } => {
                add_collapsable(
                    &mut commands,
                    &mut build_result,
                    &mut materials,
                    &mut meshes,
                    &mut asset_server,
                    root_parent,
                    options,
                    &option,
                    &metadata
                );
            }
            MenuInputType::ScrollableMenu { .. } => {}
            MenuInputType::Radial { .. } => {}
            MenuInputType::FormInput { .. } => {}
            MenuInputType::Slider { .. } => {}
        }
    }
    menu_state.set(TransitionsState::PopulateOptionsBuilder);
}

pub fn populate_options_builder(
    mut network_menu_result: ResMut<NetworkMenuResultBuilder>,
    mut graph_menu_result: ResMut<GraphMenuResultBuilder>,
    mut build_result: ResMut<BuildMenuResult>,
    mut menu_state: ResMut<NextState<TransitionsState>>,
) {
    build_result.dropdown_menu_option_results.iter().filter(|i| {
        matches!(i.1.configuration_option, ConfigurationOptionEnum::Menu(MetricsConfigurationOption::GraphMenu(..)))
    }).next().map(|i| {
        graph_menu_result.graph_menu_config_option = Some(*i.0)
    });
    build_result.dropdown_menu_option_results.iter().filter(|i| {
        matches!(i.1.configuration_option, ConfigurationOptionEnum::Menu(MetricsConfigurationOption::NetworkMenu(..)))
    }).next().map(|i| {
        network_menu_result.network_menu_config_option = Some(*i.0)
    });
    menu_state.set(TransitionsState::InsertStateTransitions);
}

fn add_collapsable(
    mut commands: &mut Commands,
    mut build_result: &mut ResMut<BuildMenuResult>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut asset_server: &mut Res<AssetServer>,
    root_parent: Entity,
    options: &Vec<MenuOption>,
    option: &&ConfigurationOptionEnum,
    metadata: &MenuItemMetadata
) {
    let mut parents = vec![];
    parents.push(metadata.clone());
    let mut collapsable = CollapsableMenuBuilder {
        parent: Some(root_parent),
        menu_metadata: &metadata,
        config_option: &option,
        parent_menus: vec![],
        menu_option_builders: menu_options(options, &parents),
        menu_component: CollapsableMenu(CollapsableMenuComponent::default()),
    };
    let collapsable = collapsable.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
    add_results_collapsable(&mut build_result, collapsable);
}

fn add_dropdown(
    mut commands: &mut Commands,
    mut build_result: &mut ResMut<BuildMenuResult>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut asset_server: &mut Res<AssetServer>,
    root_parent: Entity,
    options: &Vec<MenuOption>,
    metadata: &MenuItemMetadata,
    option: &ConfigurationOptionEnum,
) {
    let mut parents = vec![];
    parents.push(metadata.clone());
    let mut dropdown_builder = DropdownMenuBuilder {
        menu_metadata: &metadata,
        config_option: option,
        parent_menus: vec![],
        base_menu: BaseMenu {
            parent: root_parent,
            menu_metadata: &metadata,
            config_option: option,
            parent_menus: vec![],
            component: dropdown_component(options),
        },
        menu_option_builders: menu_options(options, &parents),
    };
    let dropdown = dropdown_builder.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
    add_results_dropdown(&mut build_result, dropdown);
}

pub(crate) fn dropdown_component(options: &Vec<MenuOption>) -> UiComponent {
    UiComponent::Dropdown(
        Dropdown {
            selected_index: 0,
            selectable: false,
            options: ui_components::get_menu_option_names(options),
        }
    )
}

pub(crate) fn menu_options<'a>(options: &'a Vec<MenuOption>, parents: &'a Vec<MenuItemMetadata>) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    options.iter().flat_map(|opt| {
        match opt.ui_option_type {
            MenuOptionInputType::DropdownMenu => submenu_builder(parents, &opt),
            MenuOptionInputType::CollapsableMenu => submenu_builder(parents, &opt),
            MenuOptionInputType::SubMenu => submenu_builder(parents, &opt),
            MenuOptionInputType::Activated => selected_option_builder(&opt, parents),
            MenuOptionInputType::Radial => vec![],
            MenuOptionInputType::FormInput => vec![],
            MenuOptionInputType::Slider => vec![],
        }
    }).collect()
}

fn submenu_builder<'a>(parents: &'a Vec<MenuItemMetadata>, opt: &'a MenuOption) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    match &opt.data_type {
        MenuOptionType::Primitive(_) => panic!("Dropdown MenuOptionInputType has primitive data type!"),
        MenuOptionType::SubMenu {
            sub_menu,
            parent,
            config_option
        } => build_submenu(opt, sub_menu, parents.clone())
    }
}

fn build_submenu<'a>(
    opt: &MenuOption,
    sub_menu: &'a MenuInputType,
    parents: Vec<MenuItemMetadata>
) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    match sub_menu {
        MenuInputType::Dropdown { options, option, metadata } => {
            vec![(
               opt.clone(),
               get_submenu_builder(sub_menu, &parents, option, metadata, dropdown_component(options))
            )]
        }
        MenuInputType::ScrollableMenu { option, options, metadata } => {
            vec![(
                opt.clone(),
                get_submenu_builder(sub_menu, &parents, option, metadata, UiComponent::ScrollableMenuComponent(ScrollableMenuComponent {}))
            )]
        }
        MenuInputType::CollapsableMenuInputType { option, options, metadata } => {
            vec![(
                opt.clone(),
                get_submenu_builder(sub_menu, &parents, option, metadata, UiComponent::CollapsableMenu(CollapsableMenuComponent {}))
            )]
        }
        _ => panic!("Submenu has incompatible menu input type")
    }
}

fn get_submenu_builder<'a>(
    sub_menu: &'a MenuInputType,
    parents: &Vec<MenuItemMetadata>,
    option: &'a ConfigurationOptionEnum,
    metadata: &'a MenuItemMetadata,
    ui_component: UiComponent
) -> MenuOptionBuilder<'a> {
    MenuOptionBuilder::SubmenuBuilder(
        SubmenuBuilder {
            parent: None,
            menu_metadata: metadata.clone(),
            config_option: option,
            parent_menus: parents.clone(),
            menu_component: ui_component,
            sub_menu,
        }
    )
}

fn selected_option_builder<'a>(opt: &'a MenuOption, parent_metadata: &'a Vec<MenuItemMetadata>) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    match &opt.data_type {
        MenuOptionType::Primitive(option) => {
            let propagate_visible = option.is_propagate_visible();
            vec![(
                opt.clone(),
                MenuOptionBuilder::DropdownMenuOptionBuilder(
                    DropdownMenuOptionBuilder {
                        base_menu_parent: None,
                        collapsable_menu_parent: None,
                        menu_option: opt,
                        config_option: option,
                        parents: parent_metadata.clone(),
                        menu_option_component: menu_option(opt),
                        id_component: UiIdentifiableComponent(opt.metadata.id),
                        selectable: if propagate_visible.is_some() {
                            SelectableType::DropdownSelectableChangeVisible
                        } else {
                            SelectableType::DropdownSelectableCheckmarkActivate
                        }
                    }
                )
            )]
        }
        _ => panic!("Selected menu option have MenuOptionType of Primitive.")
    }
}

fn menu_option(opt: &MenuOption) -> UiComponent {
    UiComponent::MenuOption(DropdownOption {
        index: opt.index,
        option_name: opt.metadata.name.clone(),
    })
}

fn add_results_collapsable(mut build_result: &mut ResMut<BuildMenuResult>, dropdown: DrawCollapsableMenuResult) {
    build_result.collapsable.insert(dropdown.collapsable_menu_button, dropdown.clone());
    add_results(&mut build_result,
                &dropdown.dropdown_menu_option_results,
                &dropdown.slider,
                &dropdown.submenu_results
    );
}

fn add_results_dropdown(mut build_result: &mut ResMut<BuildMenuResult>, dropdown: DrawDropdownMenuResult) {
    build_result.dropdown.insert(dropdown.base_menu_result.base_menu_parent, dropdown.clone());
    build_result.base_menu_results.insert(dropdown.base_menu_result.base_menu_parent,
                                          dropdown.base_menu_result);
    add_results(&mut build_result,
                &dropdown.dropdown_menu_option_results,
                &dropdown.slider,
                &dropdown.submenu_results
    );
}

fn add_results(
    mut build_result: &mut ResMut<BuildMenuResult>,
    dropdown_opt: &Vec<DropdownMenuOptionResult>,
    slider: &Vec<SliderMenuOptionResult>,
    submenu: &Vec<DrawSubmenuResult>
) {
    add_dropdown_menu_option_results(build_result, dropdown_opt);
    add_slider_entities(build_result, slider);
    add_submenu_results(&mut build_result, submenu);
}

fn add_submenu_results(mut build_result: &mut &mut ResMut<BuildMenuResult>, submenu: &Vec<DrawSubmenuResult>) {
    submenu
        .iter()
        .for_each(|s| {
            add_results_dropdown(&mut build_result, s.dropdown_menu_result.clone());
            build_result.submenu_results.push(s.clone());
        });
}

fn add_dropdown_menu_option_results(mut build_result: &mut ResMut<BuildMenuResult>, dropdown_menu: &Vec<DropdownMenuOptionResult>) {
    dropdown_menu
        .iter()
        .for_each(|d| {
            build_result.dropdown_menu_option_results.insert(d.menu_option_entity.unwrap(), d.clone());
        });
}

fn add_slider_entities(mut build_result: &mut ResMut<BuildMenuResult>,
                       slider_entities: &Vec<SliderMenuOptionResult>) {
    slider_entities.iter()
        .for_each(|d| {
            build_result.slider.insert(d.slider_entity, d.clone());
        });
}


fn get_swing_out(menu_option: &MenuOption) -> f32 {
    let mut swing_out_percentage = 0.0;
    if menu_option.swing_out {
        swing_out_percentage = 100.0;
    }
    swing_out_percentage
}

macro_rules! insert_config_option {
    ($($name:ident),*) => {
        pub fn insert_config_option(config_option: &ConfigurationOptionEnum, menu_option: &mut EntityCommands) {
            match config_option {
                $(
                    ConfigurationOptionEnum::$name(metrics) => {
                        menu_option.insert(metrics.clone());
                    }
                )*
            }
        }
    }
}

insert_config_option!(
    Metrics,
    NetworkMetrics,
    NetworkVariance,
    NetworkConcavity,
    LayerMetrics,
    LayerVariance,
    LayerConcavity,
    NodeMetrics,
    NodeVariance,
    NodeConcavity,
    Menu
);

