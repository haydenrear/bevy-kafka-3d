use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::menu::{ConfigurationOptionEnum, DropdownName, DropdownSelected, MenuItemMetadata, ScrollableComponent, SelectableType, UiComponent};
use crate::menu::ui_menu_event::transition_groups::PropagateDisplay;
use crate::ui_components::menu_components::BuilderResult;
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct BaseMenu<'a> {
    pub(crate) menu_metadata: &'a MenuItemMetadata,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parent_menus: Vec<MenuItemMetadata>,
    pub(crate) component: UiComponent,
    pub(crate) parent: Entity,
}

#[derive(Clone, Debug)]
pub struct BuildBaseMenuResult {
    pub(crate) base_menu_parent: Entity,
    pub(crate) base_menu_child_text: Entity
}

#[derive(Default)]
struct BuildBaseMenuResultBuilder {
    self_entity: Option<Entity>,
    text_entity: Option<Entity>
}

impl BuildBaseMenuResultBuilder {
    fn build(&self) -> BuildBaseMenuResult {
        BuildBaseMenuResult {
            base_menu_parent: self.self_entity.unwrap(),
            base_menu_child_text: self.text_entity.unwrap()
        }
    }
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
        let mut build_base_menu = BuildBaseMenuResultBuilder::default();

        let button = draw_button
            .with_children(|button| {
                let text_bundle = &mut button.spawn(self.text_bundle(&mut asset_server));
                self.spawn_text_bundle(text_bundle);
                let mut child_text_id = text_bundle.id();
                build_base_menu.text_entity = Some(child_text_id);
            });

        build_base_menu.self_entity = Some(button.id());

        insert_config_option(self.config_option, button);

        commands.get_entity(self.parent)
            .as_mut()
            .map(|parent| {
                parent.add_child(build_base_menu.self_entity.unwrap());
            });

        info!("{:?} is base menu parent.", &build_base_menu.self_entity.unwrap());

        build_base_menu.build()
    }

    /// if you have a filter component for the state transition to determine if it's activated,
    /// then you can filter, and add the state transition, and filter according to that component.
    /// When you add the state transitions to the components, you can check to see which transition
    /// group it is a part of.
    fn spawn_text_bundle<'b>(&'b self, text_bundle: &'b mut EntityCommands)  {
        // TODO: if want to add selectable from dropdown instead of checkmark selectable
        // if self.selectable {
        //     text_bundle.insert(
        //         self.selectable_bundle()
        //     );
        // } else {
        //     text_bundle.insert(
        //         self.non_selectable_bundle()
        //     );
        // }
    }

    pub(crate) fn text_bundle(&self, mut asset_server: &mut Res<AssetServer>) -> impl Bundle {
        (
            TextBundle {
                style: Style {
                    height: Val::Percent(95.0),
                    width: Val::Percent(100.0),
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

    pub(crate) fn selectable_bundle<'b>(&'b self) -> impl Bundle {
        (
            UiComponent::DropdownSelectable,
            DropdownSelected::default()
        )
    }

    pub(crate) fn non_selectable_bundle<'b>(&'b self) -> impl Bundle {
        (
            UiComponent::NamedDropdownMenu,
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
                    height: Val::Percent(100.0),
                    width: Val::Px(40.0),
                    left: pos.left,
                    right: pos.right,
                    top: pos.top,
                    bottom: pos.bottom,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..default()
            },
            ScrollableComponent::default(),
            PropagateDisplay::default(),
            UiIdentifiableComponent(self.menu_metadata.id),
            self.component.clone(),
        )
    }
}