use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::menu::{ConfigurationOptionEnum, DropdownName, DropdownSelected, MenuItemMetadata, UiComponent};
use crate::ui_components::menu_components::BuilderResult;
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct BaseMenu<'a> {
    pub(crate) menu_metadata: &'a MenuItemMetadata,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parent_menus: Vec<MenuItemMetadata>,
    pub(crate) component: UiComponent,
    pub(crate) parent: Entity,
    pub(crate) selectable: bool,
}

#[derive(Default, Clone, Debug  )]
pub struct BuildBaseMenuResult {
    pub(crate) base_menu_parent: Option<Entity>,
    pub(crate) base_menu_child_text: Option<Entity>
}

impl BuilderResult for BuildBaseMenuResult{}

impl<'a> BaseMenu<'a> {
    pub(crate) fn build(
        &self,
        mut commands: &mut Commands,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut asset_server: &mut Res<AssetServer>,
    ) -> BuildBaseMenuResult {

        let mut draw_button = commands.spawn(self.button_bundle());
        let mut build_base_menu = BuildBaseMenuResult::default() ;

        let button = draw_button
            .with_children(|button| {
                let mut child_text_id = self.spawn_text_bundle(&mut asset_server, button).id();
                build_base_menu.base_menu_child_text = Some(child_text_id);
            });

        build_base_menu.base_menu_parent = Some(button.id());

        insert_config_option(self.config_option, button);

        commands.get_entity(self.parent)
            .as_mut()
            .map(|parent| {
                parent.add_child(build_base_menu.base_menu_parent.unwrap());
            });

        info!("{:?} is base menu parent.", &build_base_menu.base_menu_parent.unwrap());

        build_base_menu
    }

    fn spawn_text_bundle(&self, mut asset_server: &mut &mut Res<AssetServer>, button: &mut ChildBuilder) -> EntityCommands {
        let mut text_bundle = button.spawn(self.text_bundle(&mut asset_server));
        if self.selectable {
            text_bundle.insert(
                self.selectable_bundle()
            );
        } else {
            text_bundle.insert(
                self.non_selectable_bundle()
            );
        }
        text_bundle
    }

    pub(crate) fn text_bundle(&self, mut asset_server: &mut Res<AssetServer>) -> impl Bundle {
        (
            TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(95.0), Val::Percent(100.0)),
                    padding: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                text: Text::from_section(self.menu_metadata.name.clone(), TextStyle {
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    color: Color::BLACK,
                    ..default()
                }),
                ..default()
            },
            Label,
            UiIdentifiableComponent(self.menu_metadata.id),
        )
    }

    pub(crate) fn selectable_bundle(&self) -> impl Bundle {
        (
            UiComponent::DropdownSelected,
            DropdownSelected::default()
        )
    }

    pub(crate) fn non_selectable_bundle(&self) -> impl Bundle {
        (
            UiComponent::DropdownName,
            DropdownName::default(),
        )
    }

    pub(crate) fn button_bundle(&self) -> impl Bundle {
        let mut pos;

        if self.parent_menus.len() > 2 {
            pos = UiRect::new(Val::Percent(100.0), Val::Percent(0.0), Val::Percent(0.0), Val::Percent(0.0));
        } else {
            pos = UiRect::default()
        }
        (
            ButtonBundle {
                style: Style {
                    display: self.component.starting_display(),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Start,
                    size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                    position: pos,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            },
            UiIdentifiableComponent(self.menu_metadata.id),
            self.component.clone(),
        )
    }
}