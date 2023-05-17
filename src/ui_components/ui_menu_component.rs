use std::marker::PhantomData;
use bevy::prelude::{AlignSelf, BackgroundColor, Bundle, ButtonBundle, Color, ColorMaterial, Commands, Component, default, Display, Entity, FlexDirection, JustifyContent, Label, Mesh, NextState, NodeBundle, Query, Res, ResMut, Size, Style, Text, TextBundle, TextStyle, UiRect, Val, Visibility, World};
use bevy::asset::{Assets, AssetServer};
use bevy::log::{error, info};
use bevy::hierarchy::BuildChildren;
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::{HoverStateChange, StateChange, Update, UpdateStateInPlace};
use crate::event::event_state::StateChange::ChangeComponentStyle;
use crate::menu::{CollapsableMenu, ConfigurationOptionEnum, DataType, DraggableComponent, Dropdown, DropdownOption, Menu, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionInputType, MenuOptionType, MenuType, MetricsConfigurationOption, Radial, ScrollableMenuComponent, ScrollableMenuItemsBarComponent, ScrollingSidebarComponent, ScrollWheelComponent, Slider, SliderData, SliderKnob, UiBundled, UiComponent};
use crate::menu::menu_resource::{MENU, MenuResource};
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{CreateMenu, StateChangeActionType, UiComponentStateTransition, UiComponentStateTransitions};
use crate::menu::UiComponent::CollapsableMenuComponent;
use crate::ui_components::ui_menu_component;
use bevy::ecs::system::EntityCommands;
use bevy::ui::{AlignItems, FocusPolicy, ZIndex};
use crate::menu::ui_menu_event::next_action::{DisplayState, SizeState, UiComponentState};
use crate::ui_components::menu_components::base_menu::BaseMenu;
use crate::ui_components::menu_components::BuildMenuResult;
use crate::ui_components::menu_components::collapsable_menu::{CollapsableMenuBuilder, DrawCollapsableMenuResult};
use crate::ui_components::menu_components::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::{SelectionMenuOptionBuilder, DropdownMenuOptionResult};
use crate::ui_components::menu_components::menu_options::MenuOptionBuilder;
use crate::ui_components::menu_components::menu_options::slider_menu_option::SliderMenuOptionResult;
use crate::ui_components::menu_components::root_collapsable::RootNodeBuilder;
use crate::ui_components::menu_components::submenu_builder::{DrawSubmenuResult, SubmenuBuilder};

#[derive(Component, Debug, Clone)]
pub struct UiIdentifiableComponent(pub f32);

pub fn create_menu(
    mut commands: Commands,
    mut build_result: ResMut<BuildMenuResult>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut menu_resource: Res<MenuResource>,
    mut menu_state: ResMut<NextState<CreateMenu>>
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
            MenuInputType::CollapsableMenu { options, option, metadata } => {
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
    menu_state.set(CreateMenu::InsertStateTransitions);
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
        menu_option_builders: menu_options(options, parents),
        menu_component: CollapsableMenuComponent(CollapsableMenu::default()),
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
            component: UiComponent::Dropdown(
                Dropdown {
                    selected_index: 0,
                    selectable: false,
                    options: get_menu_option_names(options),
                }
            ),
        },
        menu_option_builders: menu_options(options, parents),
    };
    let dropdown = dropdown_builder.build(&mut commands, &mut materials, &mut meshes, &mut asset_server);
    add_results_dropdown(&mut build_result, dropdown);
}

fn get_menu_option_names(options: &Vec<MenuOption>) -> Vec<String> {
    options.iter()
        .map(|opt| opt.metadata.name.to_string())
        .collect()
}

pub(crate) fn menu_options<'a>(options: &'a Vec<MenuOption>, parents: Vec<MenuItemMetadata>) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    options.iter().flat_map(|opt| {
        match opt.ui_option_type {
            MenuOptionInputType::Selected => selected_option_builder(&opt, parents.clone()),
            MenuOptionInputType::Radial => vec![],
            MenuOptionInputType::FormInput => vec![],
            MenuOptionInputType::Slider => vec![],
            MenuOptionInputType::DropdownMenu => match &opt.data_type {
                    MenuOptionType::Primitive(_) => panic!("Dropdown MenuOptionInputType has primitive data type!"),
                    MenuOptionType::SubMenu {
                        sub_menu,
                        parent,
                        config_option
                    } => build_submenu(opt, sub_menu, config_option, parents.clone())
            },
            MenuOptionInputType::CollapsableMenu => match &opt.data_type {
                    MenuOptionType::Primitive(_) => panic!("Dropdown MenuOptionInputType has primitive data type!"),
                    MenuOptionType::SubMenu {
                        sub_menu,
                        parent,
                        config_option
                    } => build_submenu(opt, sub_menu, config_option, parents.clone())
                }
            ,
            MenuOptionInputType::SubMenu => match &opt.data_type {
                    MenuOptionType::Primitive(_) => panic!("Dropdown MenuOptionInputType has primitive data type!"),
                    MenuOptionType::SubMenu {
                        sub_menu,
                        parent,
                        config_option
                    } => build_submenu(opt, sub_menu, config_option, parents.clone())
                }
        }
    }).collect()
}

fn build_submenu<'a>(
    opt: &MenuOption,
    sub_menu: &'a MenuInputType,
    config_option: &'a ConfigurationOptionEnum,
    parents: Vec<MenuItemMetadata>
) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    match sub_menu {
        MenuInputType::Dropdown { options, option, metadata } => {
            vec![(
               opt.clone(),
                MenuOptionBuilder::SubmenuBuilder(
                    SubmenuBuilder {
                        parent: None,
                        menu_metadata: metadata.clone(),
                        config_option: option,
                        parent_menus: parents.clone(),
                        menu_component: UiComponent::Dropdown(Dropdown {
                            selected_index: 0,
                            selectable: false,
                            options: get_menu_option_names(options),
                        }),
                        sub_menu,
                    }
                )
            )]
        }
        MenuInputType::ScrollableMenu { option, options, metadata } => {
            vec![(
                opt.clone(),
                MenuOptionBuilder::SubmenuBuilder(
                    SubmenuBuilder {
                        parent: None,
                        menu_metadata: opt.metadata.clone(),
                        config_option,
                        parent_menus: parents.clone(),
                        menu_component: UiComponent::ScrollableMenuComponent(ScrollableMenuComponent {}),
                        sub_menu,
                    }
                )
            )]
        }
        MenuInputType::CollapsableMenu { option, options, metadata } => {
            vec![(
                opt.clone(),
                MenuOptionBuilder::SubmenuBuilder(
                    SubmenuBuilder {
                        parent: None,
                        menu_metadata: opt.metadata.clone(),
                        config_option,
                        parent_menus: parents.clone(),
                        menu_component: CollapsableMenuComponent(CollapsableMenu {}),
                        sub_menu,
                    }
                )
            )]
        }
        _ => panic!("Submenu has incompatible menu input type")
    }
}

fn selected_option_builder<'a>(opt: &'a MenuOption, vec1: Vec<MenuItemMetadata>) -> Vec<(MenuOption, MenuOptionBuilder<'a>)> {
    match &opt.data_type {
        MenuOptionType::Primitive(option) => {
            vec![(
                opt.clone(),
                MenuOptionBuilder::SelectionOptionBuilder(
                    SelectionMenuOptionBuilder {
                        parent: None,
                        menu_option: opt,
                        config_option: option,
                        parents: vec1,
                        menu_option_component: UiComponent::MenuOption(DropdownOption {
                            index: opt.index,
                            option_name: opt.metadata.name.clone(),
                        }),
                        id_component: UiIdentifiableComponent(opt.metadata.id),
                    }
                )
            )]
        }
        _ => panic!("Selected menu option have MenuOptionType of Primitive.")
    }
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
    build_result.dropdown.insert(dropdown.base_menu_result.base_menu_parent.unwrap(), dropdown.clone());
    build_result.base_menu_results.insert(dropdown.base_menu_result.base_menu_parent.unwrap(),
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


// fn create_menu_from_resource(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut asset_server: Res<AssetServer>,
//     menu_resource: Res<MenuResource>
// ) {
//
//     let root_node = root_collapsable_menu(&mut commands);
//
//     for selectable in menu_resource.menu_data.selectables.iter() {
//         match selectable {
//             MenuInputType::Dropdown { options, metadata, option } => {
//                 let dropdown = draw_dropdown(
//                     &mut commands,
//                     &mut meshes,
//                     &mut materials,
//                     &mut asset_server,
//                     options,
//                     metadata,
//                     option,
//                     vec![],
//                 );
//                 commands.get_entity(root_node)
//                     .unwrap()
//                     .add_child(dropdown);
//             }
//             radial_type @ MenuInputType::Radial { option, metadata, options } => {
//                 let radial = radial(&mut commands, &mut meshes, &mut materials, &mut asset_server, option, metadata, options, radial_type);
//                 commands.get_entity(root_node)
//                     .unwrap()
//                     .add_child(radial);
//             }
//             MenuInputType::FormInput { .. } => {}
//             MenuInputType::Slider { slider_data, option, metadata } => {
//                 let slider = slider(&mut commands, &mut meshes, &mut materials, &mut asset_server, option, metadata, slider_data);
//                 commands.get_entity(root_node)
//                     .unwrap()
//                     .add_child(slider);
//             }
//             MenuInputType::CollapsableMenu { options, metadata, option } => {
//                 let collapsable = collapsable_menu(
//                     &mut commands, &mut meshes,
//                     &mut materials, &mut asset_server, option, metadata, options,
//                     &CollapsableMenuComponent(CollapsableMenu::default())
//                 );
//                 commands.get_entity(root_node)
//                     .unwrap()
//                     .add_child(collapsable);
//             }
//             MenuInputType::ScrollableMenu { options, option, metadata } => {
//
//             }
//         }
//     }
// }
//
// fn slider(
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     asset_server: &mut Res<AssetServer>,
//     option: &ConfigurationOptionEnum,
//     metadata: &MenuItemMetadata,
//     slider_data: &SliderData
// ) -> Entity {
//     commands.spawn((
//             NodeBundle {
//                 style: Style {
//                     display: Display::Flex,
//                     // size: Size::new(Val::Px(200.0), Val::Px(50.0)),
//                     ..default()
//                 },
//                 background_color: BackgroundColor(Color::GREEN),
//                 ..default()
//             },
//             UiIdentifiableComponent(20.0),
//             UiComponent::SlideComponent(Slider::default()),
//             UiComponentStateTransitions {
//                 transitions: vec![],
//             },
//             Label,
//     ))
//         .with_children(|child| {
//             child.spawn((
//                 ButtonBundle {
//                     style: Style {
//                         display: Display::Flex,
//                         position: UiRect::left(Val::Px(30.0)),
//                         size: Size::new(Val::Px(30.0), Val::Px(30.0)),
//                         ..default()
//                     },
//                     background_color: BackgroundColor(Color::ORANGE),
//                     ..default()
//                 },
//                 UiComponent::SliderKnob(SliderKnob::default()),
//                 DraggableComponent::default(),
//                 UiComponentStateTransitions {
//                     transitions: vec![
//                         UiComponentStateTransition {
//                             filter_state: UiComponentState::Any,
//                             state_change: vec![StateChangeActionType::Dragged(
//                                 ChangeComponentStyle(ChangeStyleTypes::DragX)
//                             )],
//                             propagation: ChangePropagation::SelfChange(
//                                 Relationship::SelfState
//                             ),
//                             current_state_filter: UiComponentState::Any,
//                         }
//                     ],
//                 },
//                 UiIdentifiableComponent(20.0)
//             ));
//         })
//         .id()
// }
//
// fn radial(
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     asset_server: &mut Res<AssetServer>,
//     option: &ConfigurationOptionEnum,
//     metadata: &MenuItemMetadata,
//     menu_options: &Vec<MenuOption>,
//     radial_type: &MenuInputType
// ) -> Entity {
//
//     let parent = draw_menu_submenu_parent(
//         commands, asset_server, metadata, option,
//         vec![],
//         UiComponent::RadialComponent(Radial::default())
//     );
//
//     menu_options
//         .iter()
//         .for_each(|menu_option| {
//             match &menu_option.data_type {
//                 MenuOptionType::Primitive(config_option) => {
//                     vec![draw_menu_option(
//                         commands, asset_server, menu_option,
//                         config_option, &vec![],
//                         Some(radial_type),
//                         parent
//                     )]
//                 }
//                 MenuOptionType::SubMenu { sub_menu, parent, config_option } => {
//                     error!("Found submenu option for radial!");
//                     vec![]
//                 }
//             };
//         });
//
//     parent
// }
//
//
//
// fn scrollable_menu(
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     asset_server: &mut Res<AssetServer>,
//     option: &ConfigurationOptionEnum,
//     metadata: &MenuItemMetadata,
//     options: &Vec<MenuOption>,
//     ui_menu_component: &UiComponent
// ) -> Entity {
//
//     // need two sections,
//     let scrollbar = commands.spawn((
//             NodeBundle {
//                 style: Style {
//                     display: Display::None,
//                     size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                     ..default()
//                 },
//                 ..default()
//             },
//             UiComponentStateTransitions {
//                 transitions: vec![]
//             },
//             UiComponent::ScrollableMenuComponent(ScrollableMenuComponent::default())
//         )).id();
//
//
//     // the scrolling sidebar bar
//     let mut sidebar_created = commands.spawn((
//         NodeBundle {
//             style: Style {
//                 display: Display::None,
//                 flex_direction: FlexDirection::Column,
//                 size: Size::new(Val::Percent(5.0), Val::Percent(100.0)),
//                 justify_content: JustifyContent::FlexStart,
//                 ..default()
//             },
//             background_color: BackgroundColor(Color::YELLOW),
//             ..default()
//         },
//         UiComponent::ScrollingSidebar(ScrollingSidebarComponent::default())
//     ));
//
//     let scrolling_sidebar = sidebar_created
//         .with_children(|child| {
//             child.spawn((
//                 ButtonBundle {
//                         style: Style {
//                             display: Display::Flex,
//                             size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                 DraggableComponent::default(),
//                 UiComponent::ScrollWheel(ScrollWheelComponent::default()),
//                 UiComponentStateTransitions {
//                         transitions: vec![
//                             UiComponentStateTransition {
//                                 filter_state: UiComponentState::Any,
//                                 state_change: vec![
//                                     StateChangeActionType::Dragged(ChangeComponentStyle(
//                                         ChangeStyleTypes::DragY
//                                     ))
//                                 ],
//                                 propagation: ChangePropagation::SelfChange(Relationship::SelfState),
//                                 current_state_filter: UiComponentState::Any,
//                             },
//                             UiComponentStateTransition {
//                                 filter_state: UiComponentState::Any,
//                                 state_change: vec![
//                                     StateChangeActionType::Dragged(ChangeComponentStyle(
//                                         ChangeStyleTypes::ScrollX
//                                     ))
//                                 ],
//                                 propagation: ChangePropagation::Siblings(Relationship::Sibling),
//                                 current_state_filter: UiComponentState::Any,
//                             }
//                         ],
//                 },
//                 ));
//         });
//
//     scrolling_sidebar.set_parent(scrollbar);
//
//     // menu options
//     let mut scroll_options = commands.spawn((
//             NodeBundle {
//                 style: Style {
//                     size: Size::new(Val::Px(100.0), Val::Px(100.0)),
//                     ..default()
//                 },
//                 ..default()
//             },
//             UiComponentStateTransitions {
//                 transitions: vec![],
//             },
//             UiComponent::ScrollableMenuItemsBar(ScrollableMenuItemsBarComponent::default())
//         ));
//
//     scroll_options.set_parent(scrollbar);
//
//     // info!("Inserting {:?} into collapsable menu component.",option);
//     // insert_config_option(option, entity_commands);
//
//
//     // for menu_option in options.iter() {
//     //     match &menu_option.data_type {
//     //         MenuOptionType::Primitive(primitive) => {
//     //             draw_menu_option(commands, asset_server, menu_option, primitive, &vec![], None, entity);
//     //         }
//     //         MenuOptionType::SubMenu { sub_menu, parent, config_option } => {
//     //             draw_menu_recurs(
//     //                 commands,
//     //                 meshes,
//     //                 materials,
//     //                 asset_server,
//     //                 sub_menu,
//     //                 metadata,
//     //                 vec![],
//     //                 entity
//     //             );
//     //         }
//     //     }
//     // }
//
//     scrollbar
//
// }
//
//
// fn collapsable_menu(
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     asset_server: &mut Res<AssetServer>,
//     option: &ConfigurationOptionEnum,
//     metadata: &MenuItemMetadata,
//     options: &Vec<MenuOption>,
//     ui_menu_component: &UiComponent
// ) -> Entity {
//
//     let mut button = commands.spawn((
//         ButtonBundle {
//             style: Style {
//                 display: Display::Flex,
//                 flex_direction: FlexDirection::Column,
//                 size: Size::new(Val::Percent(4.0), Val::Percent(100.0)),
//                 justify_content: JustifyContent::Start,
//                 ..default()
//             },
//             background_color: BackgroundColor(Color::BLACK),
//             ..default()
//         },
//         CollapsableMenuComponent(CollapsableMenu::default()),
//         UiIdentifiableComponent(metadata.id),
//     ));
//
//     let entity_commands = button.with_children(|child_builder| {
//             child_builder.spawn(
//                 TextBundle {
//                     style: Style {
//                         size: Size::new(Val::Percent(95.0), Val::Px(30.0)),
//                         ..default()
//                     },
//                     text: Text::from_section(metadata.name.to_string(), TextStyle {
//                         font_size: 16.0,
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         color: Color::BLACK,
//                         ..default()
//                     }),
//                     ..default()
//                 }
//             );
//         });
//
//     info!("Inserting {:?} into collapsable menu component.",option);
//     insert_config_option(option, entity_commands);
//
//     let entity = entity_commands.id().clone();
//
//     for menu_option in options.iter() {
//         match &menu_option.data_type {
//             MenuOptionType::Primitive(primitive) => {
//                 draw_menu_option(commands, asset_server, menu_option, primitive, &vec![], None, entity);
//             }
//             MenuOptionType::SubMenu { sub_menu, parent, config_option } => {
//                 draw_menu_recurs(
//                     commands,
//                     meshes,
//                     materials,
//                     asset_server,
//                     sub_menu,
//                     metadata,
//                     vec![],
//                     entity
//                 );
//             }
//         }
//     }
//
//     entity.clone()
//
// }
//
// fn root_collapsable_menu(mut commands: &mut Commands) -> Entity {
//     let mut node_bundle = commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                     display: Display::Flex,
//                     flex_direction: FlexDirection::Column,
//                     ..default()
//                 },
//                 ..default()
//             },
//             UiIdentifiableComponent(0.0),
//         ));
//     let root_entity = node_bundle
//         .insert(UiComponent::Node);
//
//     root_entity.id()
// }
//
// fn draw_menu_recurs(
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     mut asset_server: &Res<AssetServer>,
//     sub_menu: &MenuInputType,
//     menu_metadata: &MenuItemMetadata,
//     parent_menus: Vec<MenuItemMetadata>,
//     parent_entity: Entity,
// ) -> Option<Entity> {
//     match sub_menu {
//         MenuInputType::Dropdown { options, metadata, option } => {
//             let mut parent_menu_this = parent_menus.clone();
//             parent_menu_this.push(menu_metadata.clone());
//             info!("Drawing dropdown menu with metadata: {:?} and options: {:?}.", menu_metadata, options);
//             draw_dropdown_menu_recurs(
//                 commands,
//                 meshes,
//                 materials,
//                 asset_server,
//                 options,
//                 metadata,
//                 option,
//                 parent_menu_this,
//                 parent_entity
//             )
//         },
//         _ => {
//             None
//         }
//     }
// }
//
// fn draw_dropdown_menu_recurs(
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     mut asset_server: &Res<AssetServer>,
//     menu_options: &Vec<MenuOption>,
//     menu_metadata: &MenuItemMetadata,
//     config_option: &ConfigurationOptionEnum,
//     parent_menus: Vec<MenuItemMetadata>,
//     parent_entity: Entity,
// ) -> Option<Entity> {
//     Some(draw_dropdown(commands, meshes, materials, asset_server, menu_options,  menu_metadata, config_option, parent_menus.clone()))
//         .map(|dropdown| {
//             commands.get_entity(dropdown)
//                 .unwrap()
//                 .set_parent(parent_entity.clone());
//             dropdown
//         })
// }
//
// fn draw_dropdown(
//     commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     mut asset_server: &Res<AssetServer>,
//     menu_options: &Vec<MenuOption>,
//     menu_metadata: &MenuItemMetadata,
//     config_option: &ConfigurationOptionEnum,
//     parent_menus: Vec<MenuItemMetadata>
// ) -> Entity {
//
//     let options = menu_options
//         .iter()
//         .map(|menu| {
//             menu.metadata.name.clone()
//         })
//         .collect::<Vec<String>>();
//
//     let dropdown_entity = draw_menu_submenu_parent(
//         commands, asset_server, menu_metadata, config_option,
//         parent_menus.clone(),
//         UiComponent::Dropdown(
//             Dropdown {
//                 options: options.clone(),
//                 selected_index: 0,
//             }
//         )
//     );
//
//     menu_options
//         .iter()
//         .for_each(|menu_option| {
//             match &menu_option.data_type {
//                 MenuOptionType::Primitive(config_option) => {
//                     vec![draw_menu_option(commands, asset_server, menu_option, config_option, &parent_menus, None, dropdown_entity)]
//                 }
//                 MenuOptionType::SubMenu { sub_menu, parent, config_option } => {
//                     draw_menu_recurs(commands, meshes, materials, asset_server, sub_menu, menu_metadata, parent_menus.clone(), dropdown_entity)
//                         .into_iter()
//                         .collect()
//                 }
//             };
//         });
//
//     dropdown_entity
//
// }
//
//
// /// For the change, it depends on some previous state, and obtaining this state is difficult across
// /// the entire tree.
// fn draw_menu_submenu_parent(
//     commands: &mut Commands,
//     mut asset_server: &Res<AssetServer>,
//     menu_metadata: &MenuItemMetadata,
//     config_option: &ConfigurationOptionEnum,
//     parent_menus: Vec<MenuItemMetadata>,
//     component: UiComponent,
// ) -> Entity {
//
//     let mut pos;
//
//     if parent_menus.len() > 1 {
//         pos = UiRect::new(Val::Percent(100.0), Val::Percent(0.0), Val::Percent(0.0), Val::Percent(0.0))
//     } else {
//         pos = UiRect::default()
//     }
//
//     let mut draw_button = commands.spawn((
//         ButtonBundle {
//             style: Style {
//                 display: component.starting_display(),
//                 flex_direction: FlexDirection::Column,
//                 align_self: AlignSelf::Start,
//                 size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
//                 position: pos,
//                 ..default()
//             },
//             background_color: BackgroundColor(Color::BLUE),
//             ..default()
//         },
//         UiIdentifiableComponent(menu_metadata.id)
//     ));
//
//     let state_transitions = component.get_state_transitions();
//     let mut insert_dropdown = draw_button
//         .insert((
//             component,
//             state_transitions
//         ));
//
//     let commands = insert_dropdown
//         .with_children(|button| {
//             button.spawn((
//                 TextBundle {
//                     style: Style {
//                         size: Size::new(Val::Percent(95.0), Val::Percent(100.0)),
//                         padding: UiRect::top(Val::Px(10.0)),
//                         ..default()
//                     },
//                     text: Text::from_section(menu_metadata.name.clone(), TextStyle {
//                         font_size: 16.0,
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         color: Color::BLACK,
//                         ..default()
//                     }),
//                     ..default()
//                 },
//                 Label,
//                 UiIdentifiableComponent(menu_metadata.id),
//             ));
//         });
//
//     info!("Inserting {:?} to dropdown entity.", &config_option);
//     insert_config_option(config_option, commands);
//
//     let dropdown_entity = commands
//         .id();
//
//     dropdown_entity
// }
//
// fn draw_menu_option(
//     mut commands: &mut Commands,
//     mut asset_server: &Res<AssetServer>,
//     menu_option: &MenuOption,
//     config_option: &ConfigurationOptionEnum,
//     parents: &Vec<MenuItemMetadata>,
//     menu_input_type: Option<&MenuInputType>,
//     parent_entity: Entity,
// ) -> Entity {
//
//     let component_id = menu_option.metadata.id;
//     let option = menu_option.metadata.name.clone();
//
//     let menu_option_component = UiComponent::MenuOption(
//         DropdownOption {
//             index: menu_option.index,
//             option_name: option.clone(),
//         }
//     );
//     let mut menu_option_button = commands
//         .spawn((
//             ButtonBundle {
//                 style: Style {
//                     display: menu_input_type
//                         .filter(|f| matches!(f, MenuInputType::Radial {..}))
//                         .map(|_| Display::Flex)
//                         .or(Some(Display::None)).unwrap(),
//                     size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
//                     position: UiRect::new(Val::Percent(get_swing_out(menu_option)), Val::Percent(0.0), Val::Percent(0.0), Val::Percent(0.0)),
//                     ..default()
//                 },
//                 background_color: BackgroundColor(Color::GREEN),
//                 ..default()
//             },
//             UiIdentifiableComponent(component_id),
//             menu_option_component.get_state_transitions(),
//             menu_option_component
//         ));
//
//     let mut add_text_style_dropdown_option = menu_option_button
//         .with_children(|child_builder| {
//             // creates a blue mark to show inside of submenu
//             child_builder.spawn((
//                 NodeBundle {
//                     style: Style {
//                         display: Display::Flex,
//                         size: Size::all(Val::Percent(parents.len() as f32 * 10.0)),
//                         ..default()
//                     },
//                     background_color: BackgroundColor(Color::BLUE),
//                     ..default()
//                 },
//                 Label,
//                 UiIdentifiableComponent(component_id)
//             ));
//             child_builder.spawn((
//                 TextBundle {
//                     style: Style {
//                         display: Display::Flex,
//                         size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
//                         ..default()
//                     },
//                     text: Text::from_section(option.to_string(), TextStyle {
//                         font_size: 16.0,
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         color: Color::BLACK,
//                         ..default()
//                     }),
//                     ..default()
//                 },
//                 Label,
//                 UiIdentifiableComponent(component_id)
//             ));
//
//             if let Some(MenuInputType::Radial {..} ) = menu_input_type {
//                 info!("Built radial menu option.");
//             }
//
//         })
//         .set_parent(parent_entity);
//
//     info!("Inserting {:?} to menu option.", &config_option);
//     insert_config_option(config_option, &mut add_text_style_dropdown_option);
//
//     add_text_style_dropdown_option.id()
//
// }
//
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

