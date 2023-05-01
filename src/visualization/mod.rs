use std::marker::PhantomData;
use bevy::a11y::accesskit::NodeBuilder;
use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::prelude::Visibility::{Hidden, Visible};
use bevy::render::render_resource::Face::Back;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashMap;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickableMesh, PickingEvent, SelectionEvent};
use crate::menu::{CollapsableMenu, ConfigurationOption, ConfigurationOptionEnum, DataType, Dropdown, DropdownOption, MenuInputType, MenuItemMetadata, MenuOption, MenuOptionType};
use crate::menu::menu_event::{ChangePropagation, StateChange, HoverStateChange, StartingState, StateChangeActionType, UiComponent, UiComponentFilters};
use crate::menu::menu_event::change_style::ChangeStyleTypes;
use crate::menu::menu_event::StateChange::ChangeComponentStyle;
use crate::menu::menu_event::UiComponent::CollapsableMenuComponent;
use crate::menu::menu_resource::MenuResource;
use crate::metrics::MetricType;

#[derive(Component, Debug, Clone)]
pub struct UiIdentifiableComponent(pub f32);

pub fn create_dropdown(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut menu_resource: Res<MenuResource>
) {
    // create_dropdown_from_opts(
    //     commands,
    //     vec!["WeightVariance".to_string(), "Concavity".to_string()],
    //     meshes,
    //     materials,
    //     asset_server,
    // );
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
                commands.get_entity(root_node.1)
                    .unwrap()
                    .add_child(dropdown);
            }
            MenuInputType::Radial { .. } => {}
            MenuInputType::FormInput { .. } => {}
            MenuInputType::ContinuousMovingButton { .. } => {}
        }
    }
}

fn root_collapsable_menu(mut commands: &mut Commands) -> (Entity, Entity) {
    let mut other_entity = None;
    let root_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            UiIdentifiableComponent(0.0),
        ))
        .insert(UiComponent::Node(vec![]))
        .with_children(|child_builder| {
            other_entity = Some(child_builder.spawn((
                ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }, CollapsableMenuComponent(CollapsableMenu::default(), vec![
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeSize {
                                height_1: 100.0,
                                height_2: 0.0,
                                width_1: 10.0,
                                width_2: 0.0,
                                filters: None,
                            },
                            ChangePropagation::Children(StartingState::Child)
                        ),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeVisible(None),
                            ChangePropagation::Children(StartingState::Child),
                        ),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeSize {
                                height_1: 100.0,
                                height_2: 100.0,
                                width_1: 100.0,
                                width_2: 100.0,
                                filters: None
                            },
                            ChangePropagation::SelfChange(StartingState::SelfState)
                        ),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeSize {
                                height_1: 100.0,
                                height_2: 100.0,
                                width_1: 20.0,
                                width_2: 3.0,
                                filters: None
                            },
                            ChangePropagation::Parent(StartingState::Parent)
                        ),
                    }
                ]),
                UiIdentifiableComponent(1.0),
            )).id());
        }).id();

    (root_entity, other_entity.unwrap())
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

    let dropdown_entity = draw_dropdown_components(commands, asset_server, menu_metadata, options, config_option);

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

fn draw_dropdown_components(
    commands: &mut Commands,
    mut asset_server: &Res<AssetServer>,
    menu_metadata: &MenuItemMetadata,
    options: Vec<String>,
    config_option: &ConfigurationOptionEnum
) -> Entity {

    let mut draw_button = commands.spawn((
        ButtonBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::End,
                size: Size::new(Val::Percent(100.0), Val::Percent(98.0)),
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
                }, vec![
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeVisible(None),
                            ChangePropagation::Children(StartingState::Child),
                        ),
                    }],
            )
        ));

    let commands = insert_dropdown
        .with_children(|button| {
            button.spawn((
                TextBundle {
                    style: Style {
                        size: Size::new(Val::Percent(95.0), Val::Percent(10.0)),
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
                UiIdentifiableComponent(menu_metadata.id)
            ));
        });

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
    parent_entity: Entity
) -> Entity {

    let component_id = menu_option.metadata.id;
    let option = menu_option.metadata.name.clone();

    let mut menu_option_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::None,
                    size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::GREEN),
                ..default()
            },
            UiIdentifiableComponent(component_id)
        ));

    let mut add_dropdown_option_component = menu_option_button
        .insert((
            UiComponent::DropdownOption(
                DropdownOption {
                    index: menu_option.index,
                    option_name: option.clone(),
                }, vec![StateChangeActionType {
                    hover: HoverStateChange::None,
                    clicked: StateChange::None,
                }]
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

    insert_config_option(config_option, &mut add_text_style_dropdown_option);

    add_text_style_dropdown_option.id()

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
        NodeConcavity
);

fn create_dropdown_from_opts(
    mut commands: Commands,
    options: Vec<String>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) -> Entity {
    let mut dropdown: Option<Entity> = None;

    let dropdown_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            UiIdentifiableComponent(0.0),
        ))
        .insert(UiComponent::Node(vec![]))
        .with_children(|node| {

            node.spawn((
                ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                }, CollapsableMenuComponent(CollapsableMenu::default(), vec![
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeSize {
                                height_1: 100.0,
                                height_2: 0.0,
                                width_1: 10.0,
                                width_2: 0.0,
                                filters: None,
                            },
                            ChangePropagation::Children(StartingState::Child)
                        ),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeVisible(None),
                            ChangePropagation::Children(StartingState::Child) ,
                        ),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeSize {
                                height_1: 100.0,
                                height_2: 100.0,
                                width_1: 100.0,
                                width_2: 100.0,
                                filters: None
                            },
                            ChangePropagation::SelfChange(StartingState::SelfState)
                        ),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ChangeComponentStyle(
                            ChangeStyleTypes::ChangeSize {
                                height_1: 100.0,
                                height_2: 100.0,
                                width_1: 20.0,
                                width_2: 3.0,
                                filters: None
                            },
                            ChangePropagation::Parent(StartingState::Parent)
                        ),
                    }
                ]),
                UiIdentifiableComponent(1.0),
            ))
                .with_children(|submenu| {
                    dropdown = Some(submenu
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    align_self: AlignSelf::End,
                                    size: Size::new(Val::Percent(100.0), Val::Percent(98.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::BLUE),
                                ..default()
                            },
                            UiIdentifiableComponent(2.0)
                        ))
                        .insert((
                            UiComponent::Dropdown(
                                Dropdown {
                                    options: options.clone(),
                                    selected_index: 0,
                                }, vec![
                                    StateChangeActionType {
                                        hover: HoverStateChange::None,
                                        clicked: ChangeComponentStyle(
                                            ChangeStyleTypes::ChangeVisible(None),
                                            ChangePropagation::Children(StartingState::Child)
                                        )
                                    }]
                            )
                        ))
                        .with_children(|button| {
                            button.spawn((
                                TextBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(95.0), Val::Percent(10.0)),
                                        ..default()
                                    },
                                    text: Text::from_section("Configuration Options".to_string(), TextStyle {
                                        font_size: 16.0,
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        color: Color::BLACK,
                                        ..default()
                                    }),
                                    ..default()
                                },
                                Label,
                                UiIdentifiableComponent(3.0)
                            ));
                        })
                        .id());
                });




        })
        .id();

    let mut component_id = 4.0;

    options
        .iter()
        .enumerate()
        .for_each(|(index, option)| {
            let _ = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            display: Display::None,
                            size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::GREEN),
                        ..default()
                    },
                    UiIdentifiableComponent(component_id as f32)
                ))
                .insert((
                    UiComponent::DropdownOption(
                        DropdownOption {
                            index,
                            option_name: option.clone(),
                        }, vec![StateChangeActionType {
                            hover: HoverStateChange::None,
                            clicked: StateChange::None,
                        }]
                    )
                ))
                .with_children(|child_builder| {
                    child_builder.spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                size: Size::all(Val::Percent(10.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::BLUE),
                            ..default()
                        },
                        Label,
                        UiIdentifiableComponent(component_id + 1.0)
                    ));
                    child_builder.spawn((
                        TextBundle {
                            style: Style {
                                display: Display::Flex,
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
                        UiIdentifiableComponent(component_id + 1.0)
                    ));
                })
                .set_parent(dropdown.unwrap())
                .id();
            component_id = component_id + 2.0 as f32;
        });


    dropdown_entity
}

