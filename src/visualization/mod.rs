use bevy::a11y::accesskit::NodeBuilder;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::prelude::Visibility::{Hidden, Visible};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickableMesh, PickingEvent, SelectionEvent};
use crate::menu::{CollapsableMenu, Dropdown, DropdownOption};
use crate::menu::menu_event::{ChangeStyle, ClickStateChange, HoverStateChange, StateChangeActionType, StyleChangeType, UiComponent, UiComponentFilters};
use crate::menu::menu_event::UiComponent::CollapsableMenuComponent;
use crate::metrics::MetricType;

#[derive(Component, Debug, Clone)]
pub struct UpdateableComponent(pub f32);

pub fn create_dropdown(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
) {
    create_dropdown_from_opts(commands, vec!["WeightVariance".to_string(), "Concavity".to_string()], meshes, materials, asset_server);
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
                    border: UiRect::new(Val::Px(5.0), Val::Px(5.0), Val::Px(5.0), Val::Px(5.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            UpdateableComponent(0.0)
        ))
        .with_children(|node| {

            node.spawn((
                ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(5.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                }, CollapsableMenuComponent(CollapsableMenu::default(), vec![
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ClickStateChange::ChangeDisplay(
                            ChangeStyle::ChangeVisible(Some(UiComponentFilters {
                            exclude: None,
                            include: None,
                        })), vec![StyleChangeType::Child]),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ClickStateChange::ChangeDisplay(
                            ChangeStyle::ChangeSize {
                                height_1: 100.0,
                                height_2: 100.0,
                                width_1: 10.0,
                                width_2: 100.0,
                            },
                            vec![StyleChangeType::SelfChange]),
                    },
                    StateChangeActionType {
                        hover: HoverStateChange::None,
                        clicked: ClickStateChange::ChangeDisplay(
                            ChangeStyle::ChangeSize {
                                height_1: 100.0,
                                height_2: 100.0,
                                width_1: 20.0,
                                width_2: 15.0,
                            },
                            vec![StyleChangeType::Parent]),
                    }
                ]),
                UpdateableComponent(3.0)
            ));

            dropdown = Some(node
                .spawn((
                    ButtonBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLUE),
                        ..default()
                    },
                    UpdateableComponent(1.0)
                ))
                .insert(
                    UiComponent::Dropdown(
                        Dropdown {
                            options: options.clone(),
                            selected_index: 0,
                        }, vec![
                            StateChangeActionType {
                                hover: HoverStateChange::None,
                                clicked: ClickStateChange::ChangeDisplay(
                                    ChangeStyle::ChangeVisible(None),
                                    vec![StyleChangeType::SelfChange]),
                            }]
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
                        UpdateableComponent(2.0)
                    ));
                })
                .id());



        })
        .id();

    let mut component_id = 3.0;

    options
        .iter()
        .enumerate()
        .for_each(|(index, option)| {
            let _ = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            display: Display::None,
                            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                            // justify_content: JustifyContent::Center,
                            // align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::GREEN),
                        ..default()
                    },
                    UpdateableComponent(component_id + 1 as f32)
                ))
                .insert((
                    UiComponent::DropdownOption(
                        DropdownOption {
                            index,
                            option_name: option.clone(),
                        }, vec![StateChangeActionType {
                            hover: HoverStateChange::None,
                            clicked: ClickStateChange::None,
                        }]
                    )
                ))
                .with_children(|child_builder| {
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
                        UpdateableComponent(component_id + 1 as f32)
                    ));
                })
                .set_parent(dropdown.unwrap())
                .id();
        });


    dropdown_entity
}

