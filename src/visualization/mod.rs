use bevy::a11y::accesskit::NodeBuilder;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::prelude::Visibility::{Visible, Hidden};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{HoverEvent, PickableBundle, PickableMesh, PickingEvent, SelectionEvent};
use crate::metrics::MetricType;

pub struct DropdownPlugin;

impl Plugin for DropdownPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(dropdown_click_system)
            .add_system(dropdown_read_event)
            // .add_system(dropdown_event)
            .add_event::<DropdownEvent>()
            .add_system(hover_event)
            .add_startup_system(create_dropdown);
    }
}

#[derive(Component, Default)]
pub struct Dropdown {
    selected_index: usize,
    options: Vec<String>
}

#[derive(Component, Default)]
pub struct DropdownOption {
    index: usize,
    option_name: String,
}


pub enum DropdownEvent {
    Click,
    Hover
}

pub fn create_dropdown(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>
) {
    create_dropdown_from_opts(commands, vec!["WeightVariance".to_string(), "Concavity".to_string()], meshes, materials, asset_server);
}

fn create_dropdown_from_opts(
    mut commands: Commands,
    options: Vec<String>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>
) -> Entity {

    let dropdown_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
        )
        .with_children(|node| {
            node.spawn(ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            }).insert(Dropdown {
                options: options.clone(),
                selected_index: 0,
            })
            .with_children(|button| {
                button.spawn((TextBundle {
                    style: Style {
                        display: Display::Flex,
                        ..default()
                    },
                    text: Text::from_section("Configuration Options".to_string(), TextStyle {
                        font_size: 16.0,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        color: Color::BLACK,
                        ..default()
                    }),
                    ..default()
                }, Label));
            });

        })
        .id();

    options
        .iter()
        .enumerate()
        .for_each(|(index, option)| {
            let _ = commands
                .spawn((ButtonBundle {
                    style: Style {
                        display: Display::None,
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                }))
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
                        }, Label))
                    ;
                })
                .insert((DropdownOption {
                    index,
                    option_name: option.clone()
                }))
                .set_parent(dropdown_entity)
                .id();
        });




    dropdown_entity
}

pub struct UiInteractionEvent;

fn dropdown_click_system(
    mut commands: Commands,
    mut event_write: EventWriter<DropdownEvent>,
    mut query: Query<(Entity, &mut Dropdown), With<Button>>,
    mut interaction_query: Query<&Interaction, (With<Button>, Changed<Interaction>, With<Dropdown>)>,
) {
    for (entity, dropdown) in query.iter() {
        for interaction in interaction_query.iter() {
            if let Interaction::Clicked = interaction {
                info!("dropdown");
                event_write.send(DropdownEvent::Click);
            }
        }
    }
}

fn hover_event(
    mut query: Query<(&mut Style, &mut BackgroundColor, &Interaction), (With<DropdownOption>, With<Button>, Changed<Interaction>)>,
) {
    for (_, mut color, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                color.0 = Color::BLUE;
            }
            Interaction::Hovered => {
                color.0 = Color::YELLOW;
            }
            Interaction::None => {
                color.0 = Color::GREEN;
            }
        }
    }
}

fn dropdown_read_event(
    mut commands: Commands,
    mut event_write: EventReader<DropdownEvent>,
    mut query: Query<(&mut Style, &mut BackgroundColor), With<DropdownOption>>,
) {
    for event in event_write.iter() {
        if let DropdownEvent::Click = event {
            for (mut q, mut button) in query.iter_mut() {
                info!("Read!");
                match &q.display {
                    Display::Flex => {
                        q.display = Display::None;
                    }
                    Display::None => {
                        info!("Read!");
                        q.display = Display::Flex;
                    }
                }
            }
        }
    }
}

// fn dropdown_event(
//     mut commands: Commands,
//     mut event_write: EventReader<DropdownClickedEvent>,
//     mut query: Query<&mut Style, With<Dropdown>>,
// ) {
//     for event in event_write.iter() {
//         if let DropdownClickedEvent::Hide = event {
//             for mut q in query.iter_mut() {
//                 info!("Read!");
//                 match &q.display {
//                     Display::Flex => {
//                         q.display = Display::None;
//                     }
//                     Display::None => {
//                         info!("Read!");
//                         q.display = Display::Flex;
//                     }
//                 }
//             }
//         }
//     }
// }

// fn handle_dropdown_selected(
//     mut event_reader: EventReader<PickingEvent>,
//     mut query: Query<(Entity, &DropdownOption, &mut VisualizationConfig, &Parent)>,
//     mut dropdown_query: Query<(Entity, &mut Dropdown)>,
//     mut commands: Commands
// ) {
//     for event in event_reader.iter() {
//         if let PickingEvent::Clicked(selected) = event {
//             query.get(selected.clone())
//                 .map(|dropdown| {
//                     let selected_option = &dropdown.1.index;
//                     dropdown_query.get_mut(dropdown.3.get())
//                         .map(|mut dropdown| {
//                             dropdown.1.selected_index = *selected_option;
//                             commands.entity(dropdown.0)
//                                 .insert(TextBundle {
//                                     text: Text::from_section(dropdown.1.options[*selected_option].to_string(), TextStyle::default()),
//                                     ..default()
//                                 });
//                         })
//                         ;
//                 });
//         }
//     }
// }

struct DropdownSelected {
    dropdown_entity: Entity,
    option_entity: Entity,
}

#[derive(Component, Default)]
pub struct VisualizationConfig {
}
