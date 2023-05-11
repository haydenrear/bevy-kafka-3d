use std::marker::PhantomData;
use bevy::prelude::{AlignSelf, BackgroundColor, ButtonBundle, Color, ColorMaterial, Commands, Component, default, Display, Entity, FlexDirection, JustifyContent, Label, Mesh, NodeBundle, Res, ResMut, Size, Style, Text, TextBundle, TextStyle, UiRect, Val, Visibility};
use bevy::asset::{Assets, AssetServer};
use bevy::log::info;
use bevy::hierarchy::BuildChildren;
use crate::event::event_propagation::{ChangePropagation, StartingState};
use crate::event::event_state::{HoverStateChange, StateChange};
use crate::event::event_state::StateChange::ChangeComponentStyle;
use crate::menu::{CollapsableMenu, ConfigurationOptionEnum, DataType, Dropdown, DropdownOption, Menu, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionType, MenuType, MetricsConfigurationOption};
use crate::menu::menu_resource::{MENU, MenuResource};
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{DisplayState, SizeState, StateChangeActionType, UiComponent, UiComponentState, UiComponentStateTransition, UiComponentStateTransitions};
use crate::menu::ui_menu_event::ui_menu_event_plugin::UiComponent::CollapsableMenuComponent;
use crate::ui_components::ui_menu_component;
use bevy::ecs::system::EntityCommands;

#[derive(Component, Debug, Clone)]
pub struct UiIdentifiableComponent(pub f32);

pub fn create_dropdown(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut menu_resource: Res<MenuResource>
) {
    create_dropdown_from_resource(commands, meshes, materials, asset_server, menu_resource);
}

fn create_dropdown_from_resource(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
    menu_resource: Res<MenuResource>
) {

    let root_node = root_collapsable_menu(&mut commands);

    for selectable in menu_resource.menu_data.selectables.iter() {
        match selectable {
            MenuInputType::Dropdown { options, metadata, option } => {
                let dropdown = draw_dropdown(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut asset_server,
                    options,
                    metadata,
                    option,
                    vec![],
                );
                commands.get_entity(root_node)
                    .unwrap()
                    .add_child(dropdown);
            }
            MenuInputType::Radial { .. } => {}
            MenuInputType::FormInput { .. } => {}
            MenuInputType::ContinuousMovingButton { .. } => {}
            MenuInputType::CollapsableMenu { options, metadata, option } => {
                let collapsable = collapsable_menu(&mut commands, &mut meshes, &mut materials, &mut asset_server, option, metadata, options);
                commands.get_entity(root_node)
                    .unwrap()
                    .add_child(collapsable);
            }
        }
    }
}

fn collapsable_menu(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &mut Res<AssetServer>,
    option: &ConfigurationOptionEnum,
    metadata: &MenuItemMetadata,
    options: &Vec<MenuOption>
) -> Entity {

    let mut button = commands.spawn((
        ButtonBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(4.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        UiComponentStateTransitions {
            transitions: vec![
                UiComponentStateTransition{
                    filter_state: UiComponentState::StateSize(SizeState::Expanded{
                        height: 100,
                        width: 20
                    }),
                    state_change: vec![
                        StateChangeActionType::Clicked(ChangeComponentStyle(
                            ChangeStyleTypes::RemoveVisible(None)
                        ))
                    ],
                    propagation: ChangePropagation::Children(
                        StartingState::EachSelfState
                    ),
                    current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                },
                UiComponentStateTransition{
                    filter_state: UiComponentState::StateSize(SizeState::Minimized {
                        height: 100,
                        width: 4
                    }),
                    state_change: vec![
                        StateChangeActionType::Clicked(ChangeComponentStyle(
                            ChangeStyleTypes::AddVisible(None)
                        ))
                    ],
                    propagation: ChangePropagation::Children(
                        StartingState::EachSelfState
                    ),
                    current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayNone),
                },
                UiComponentStateTransition{
                    filter_state: UiComponentState::StateSize(SizeState::Minimized{
                        height: 100,
                        width: 4
                    }),
                    state_change: vec![
                        StateChangeActionType::Clicked(
                            ChangeComponentStyle(
                                ChangeStyleTypes::UpdateSize {
                                    height_1: 100.0,
                                    width_1: 20.0,
                                    filters: None,
                                }
                            ))
                    ],
                    propagation: ChangePropagation::SelfChange(
                        StartingState::SelfState
                    ),
                    current_state_filter: UiComponentState::StateSize(SizeState::Minimized {
                        height: 100,
                        width: 4
                    }),
                },
                UiComponentStateTransition{
                    filter_state: UiComponentState::StateSize(SizeState::Expanded {
                        height: 100,
                        width: 20
                    }),
                    state_change: vec![
                        StateChangeActionType::Clicked(
                            ChangeComponentStyle(
                                ChangeStyleTypes::UpdateSize {
                                    height_1: 100.0,
                                    width_1: 4.0,
                                    filters: None,
                                }
                            ))
                    ],
                    propagation: ChangePropagation::SelfChange(
                        StartingState::SelfState
                    ),
                    current_state_filter: UiComponentState::StateSize(SizeState::Expanded {
                        height: 100,
                        width: 20
                    }),
                },
            ],
        },
        CollapsableMenuComponent(CollapsableMenu::default()),
        UiIdentifiableComponent(metadata.id),
    ));

    let entity_commands = button.with_children(|child_builder| {
            child_builder.spawn(
                TextBundle {
                    style: Style {
                        size: Size::new(Val::Percent(95.0), Val::Px(30.0)),
                        ..default()
                    },
                    text: Text::from_section(metadata.name.to_string(), TextStyle {
                        font_size: 16.0,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        color: Color::BLACK,
                        ..default()
                    }),
                    ..default()
                }
            );
        });

    info!("Inserting {:?} into collapsable menu component.",option);
    insert_config_option(option, entity_commands);

    let entity = entity_commands.id().clone();

    for menu_option in options.iter() {
        match &menu_option.data_type {
            MenuOptionType::Primitive(primitive) => {
                draw_menu_option(commands, asset_server, menu_option, primitive, &vec![], entity);
            }
            MenuOptionType::SubMenu { sub_menu, parent, config_option } => {
                draw_menu_recurs(
                    commands,
                    meshes,
                    materials,
                    asset_server,
                    sub_menu,
                    metadata,
                    vec![],
                    entity
                );
            }
        }
    }

    entity.clone()

}

fn root_collapsable_menu(mut commands: &mut Commands) -> Entity {
    let mut node_bundle = commands
        .spawn((
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
        ));
    let root_entity = node_bundle
        .insert(UiComponent::Node);

    root_entity.id()
}

fn draw_menu_recurs(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut asset_server: &Res<AssetServer>,
    sub_menu: &MenuInputType,
    menu_metadata: &MenuItemMetadata,
    parent_menus: Vec<MenuItemMetadata>,
    parent_entity: Entity,
) -> Option<Entity> {
    match sub_menu {
        MenuInputType::Dropdown { options, metadata, option } => {
            let mut parent_menu_this = parent_menus.clone();
            parent_menu_this.push(menu_metadata.clone());
            info!("Drawing dropdown menu with metadata: {:?} and options: {:?}.", menu_metadata, options);
            draw_dropdown_menu_recurs(
                commands,
                meshes,
                materials,
                asset_server,
                options,
                metadata,
                option,
                parent_menu_this,
                parent_entity
            )
        }
        _ => {
            None
        }
    }
}

fn draw_dropdown_menu_recurs(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut asset_server: &Res<AssetServer>,
    menu_options: &Vec<MenuOption>,
    menu_metadata: &MenuItemMetadata,
    config_option: &ConfigurationOptionEnum,
    parent_menus: Vec<MenuItemMetadata>,
    parent_entity: Entity,
) -> Option<Entity> {
    Some(draw_dropdown(commands, meshes, materials, asset_server, menu_options,  menu_metadata, config_option, parent_menus.clone()))
        .map(|dropdown| {
            commands.get_entity(dropdown)
                .unwrap()
                .set_parent(parent_entity.clone());
            dropdown
        })
}

fn draw_dropdown(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut asset_server: &Res<AssetServer>,
    menu_options: &Vec<MenuOption>,
    menu_metadata: &MenuItemMetadata,
    config_option: &ConfigurationOptionEnum,
    parent_menus: Vec<MenuItemMetadata>
) -> Entity {

    let options = menu_options
        .iter()
        .map(|menu| {
            menu.metadata.name.clone()
        })
        .collect::<Vec<String>>();

    let dropdown_entity = draw_dropdown_components(commands, asset_server, menu_metadata, options, config_option, parent_menus.clone());

    menu_options
        .iter()
        .for_each(|menu_option| {
            match &menu_option.data_type {
                MenuOptionType::Primitive(config_option) => {
                    vec![draw_menu_option(commands, asset_server, menu_option, config_option, &parent_menus, dropdown_entity)]
                }
                MenuOptionType::SubMenu { sub_menu, parent, config_option } => {
                    draw_menu_recurs(commands, meshes, materials, asset_server, sub_menu, menu_metadata, parent_menus.clone(), dropdown_entity)
                        .into_iter()
                        .collect()
                }
            };
        });

    dropdown_entity

}


/// For the change, it depends on some previous state, and obtaining this state is difficult across
/// the entire tree.
fn draw_dropdown_components(
    commands: &mut Commands,
    mut asset_server: &Res<AssetServer>,
    menu_metadata: &MenuItemMetadata,
    options: Vec<String>,
    config_option: &ConfigurationOptionEnum,
    parent_menus: Vec<MenuItemMetadata>
) -> Entity {

    let mut pos;

    if parent_menus.len() > 1 {
        pos = UiRect::new(Val::Percent(100.0), Val::Percent(0.0), Val::Percent(0.0), Val::Percent(0.0))
    } else {
        pos = UiRect::default()
    }

    let mut draw_button = commands.spawn((
        ButtonBundle {
            style: Style {
                display: Display::None,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Start,
                size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                position: pos,
                ..default()
            },
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        },
        UiIdentifiableComponent(menu_metadata.id)
    ));

    let mut insert_dropdown = draw_button
        .insert((
            UiComponent::Dropdown(
                Dropdown {
                    options: options.clone(),
                    selected_index: 0,
                }
            ),
            UiComponentStateTransitions {
                transitions: vec![
                    UiComponentStateTransition {
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        state_change: vec![
                            StateChangeActionType::Clicked(
                                ChangeComponentStyle(
                                    ChangeStyleTypes::ChangeVisible(None)
                                ))
                        ],
                        propagation: ChangePropagation::Children(
                            StartingState::EachSelfState
                        ),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                    },
                    UiComponentStateTransition {
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        state_change: vec![
                            StateChangeActionType::Clicked(
                                ChangeComponentStyle(
                                    ChangeStyleTypes::RemoveVisible(None)
                                ))
                        ],
                        propagation: ChangePropagation::SiblingsChildrenRecursive(
                            StartingState::EachSelfState
                        ),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                    }
                ]
            }
        ));

    let commands = insert_dropdown
        .with_children(|button| {
            button.spawn((
                TextBundle {
                    style: Style {
                        size: Size::new(Val::Percent(95.0), Val::Px(40.0)),
                        padding: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    text: Text::from_section(menu_metadata.name.clone(), TextStyle {
                        font_size: 16.0,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        color: Color::BLACK,
                        ..default()
                    }),
                    ..default()
                },
                Label,
                UiIdentifiableComponent(menu_metadata.id),
            ));
        });

    info!("Inserting {:?} to dropdown entity.", &config_option);
    insert_config_option(config_option, commands);

    let dropdown_entity = commands
        .id();

    dropdown_entity
}

fn draw_menu_option(
    mut commands: &mut Commands,
    mut asset_server: &Res<AssetServer>,
    menu_option: &MenuOption,
    config_option: &ConfigurationOptionEnum,
    parents: &Vec<MenuItemMetadata>,
    parent_entity: Entity,
) -> Entity {

    let component_id = menu_option.metadata.id;
    let option = menu_option.metadata.name.clone();

    let mut menu_option_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::None,
                    size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                    position: UiRect::new(Val::Percent(get_swing_out(menu_option)), Val::Percent(0.0), Val::Percent(0.0), Val::Percent(0.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::GREEN),
                ..default()
            },
            UiIdentifiableComponent(component_id),
            UiComponentStateTransitions {
                transitions: vec![],
            }
        ));

    let mut add_dropdown_option_component = menu_option_button
        .insert((
            UiIdentifiableComponent(component_id),
            UiComponent::MenuOption(
                DropdownOption {
                    index: menu_option.index,
                    option_name: option.clone(),
                }
            )
        ));

    let mut add_text_style_dropdown_option = add_dropdown_option_component
        .with_children(|child_builder| {
            child_builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        size: Size::all(Val::Percent(parents.len() as f32 * 10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                },
                Label,
                UiIdentifiableComponent(component_id)
            ));
            child_builder.spawn((
                TextBundle {
                    style: Style {
                        display: Display::Flex,
                        size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                        ..default()
                    },
                    text: Text::from_section(option.to_string(), TextStyle {
                        font_size: 16.0,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        color: Color::BLACK,
                        ..default()
                    }),
                    ..default()
                },
                Label,
                UiIdentifiableComponent(component_id)
            ));
        })
        .set_parent(parent_entity);

    info!("Inserting {:?} to menu option.", &config_option);
    insert_config_option(config_option, &mut add_text_style_dropdown_option);

    add_text_style_dropdown_option.id()

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
        fn insert_config_option(config_option: &ConfigurationOptionEnum, menu_option: &mut EntityCommands) {
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

