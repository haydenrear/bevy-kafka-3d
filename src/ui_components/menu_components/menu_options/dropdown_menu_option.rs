use std::default::default;
use bevy::prelude::*;
use crate::menu::{ConfigurationOptionEnum, MenuInputType, MenuItemMetadata, MenuOption, UiComponent};
use crate::ui_components::menu_components::base_menu::BaseMenu;
use crate::ui_components::menu_components::{add_config_opt, BuilderResult, get_swing_out};
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct SelectionMenuOptionBuilder<'a> {
    pub(crate) parent: Option<Entity>,
    pub(crate) menu_option: &'a MenuOption,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parents: Vec<MenuItemMetadata>,
    pub(crate) menu_option_component: UiComponent,
    pub(crate) id_component: UiIdentifiableComponent,
}

#[derive(Default, Clone, Debug)]
pub struct DropdownMenuOptionResult {
    pub(crate) menu_option_entity: Option<Entity>,
    pub(crate) breadcrumbs_entity: Option<Entity>,
    pub(crate) selected_entity: Option<Entity>,
    pub(crate) text_entity: Option<Entity>
}

impl BuilderResult for DropdownMenuOptionResult {}

impl <'a> SelectionMenuOptionBuilder<'a> {

    pub(crate) fn build(
        &self,
        mut commands: &mut Commands,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut asset_server: &mut Res<AssetServer>,
    ) -> DropdownMenuOptionResult {

        let mut result = DropdownMenuOptionResult::default();

        let mut menu_option_button = commands
            .spawn(self.menu_option_button())
            .id();

        result.menu_option_entity = Some(menu_option_button);

        commands.get_entity(menu_option_button)
            .as_mut()
            .map(|menu_option_button| menu_option_button
                .with_children(|child_builder| {
                    // creates a blue mark to show inside of submenu
                    result.breadcrumbs_entity = Some(child_builder.spawn(self.breadcrumbs_entity()).id());
                    result.text_entity = Some(child_builder.spawn(self.text_entity(&mut asset_server)).id());
                })
            );

        commands.get_entity(self.parent.unwrap())
            .as_mut()
            .map(|parent| {
                info!("Adding child to {:?}", self.parent.unwrap());
                parent.add_child(menu_option_button)
            });

        info!("Inserting {:?} to menu option.", &self.config_option);
        add_config_opt(&mut commands, result.menu_option_entity, self.config_option);

        result

    }

    pub(crate) fn menu_option_button(&self) -> impl Bundle {
        (
            ButtonBundle {
                style: Style {
                    display: Display::None,
                    size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                    position: UiRect::new(
                        Val::Percent(get_swing_out(self.menu_option)),
                        Val::Percent(0.0),
                        Val::Percent(0.0),
                        Val::Percent(0.0)
                    ),

                    ..default()
                },
                z_index: ZIndex::Global(100),
                background_color: BackgroundColor(Color::BLACK),
                ..default()
            },
            self.id_component.clone(),
            self.menu_option_component.clone()
        )
    }

    pub(crate) fn selected_entity(&self) -> impl Bundle {
        ()
    }

    pub(crate) fn text_entity(&self, mut asset_server: &mut Res<AssetServer>) -> impl Bundle {
        (
            TextBundle {
                style: Style {
                    display: Display::Flex,
                    size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                    ..default()
                },
                text: Text::from_section(self.menu_option.metadata.name.to_string(), TextStyle {
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    color: Color::BLACK,
                    ..default()
                }),
                ..default()
            },
            Label,
            self.id_component.clone()
        )
    }

    pub(crate) fn breadcrumbs_entity(&self) -> impl Bundle {
        (
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    size: Size::all(Val::Percent(self.parents.len() as f32 * 10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            },
            Label,
            self.id_component.clone()
        )
    }
}
