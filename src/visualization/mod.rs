use std::marker::PhantomData;
use bevy::a11y::accesskit::NodeBuilder;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::prelude::Visibility::{Hidden, Visible};
use bevy::render::render_resource::Face::Back;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::HashMap;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickableMesh, PickingEvent, SelectionEvent};
use crate::menu::{CollapsableMenu, ConfigurationOption, DataType, Dropdown, DropdownOption};
use crate::menu::menu_event::{ChangePropagation, StateChange, HoverStateChange, StartingState, StateChangeActionType, UiComponent, UiComponentFilters};
use crate::menu::menu_event::change_style::ChangeStyleTypes;
use crate::menu::menu_event::StateChange::ChangeComponentStyle;
use crate::menu::menu_event::UiComponent::CollapsableMenuComponent;
use crate::metrics::MetricType;

#[derive(Component, Debug, Clone)]
pub struct UiIdentifiableComponent(pub f32);

pub fn create_dropdown(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
) {
    create_dropdown_from_opts(
        commands,
        vec!["WeightVariance".to_string(), "Concavity".to_string()],
        meshes,
        materials,
        asset_server,
    );
}

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
                                            ChangeStyleTypes::ChangeVisible(Some(UiComponentFilters {
                                                exclude: Some(vec![1.0])
                                            })),
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

