use std::default::default;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::menu::{ConfigurationOptionEnum, MenuItemMetadata, MenuOption, MenuOptionType, UiComponent};
use crate::menu::ui_menu_event::transition_groups::PropagateDisplay;
use crate::menu::UiComponent::CollapsableMenu;
use crate::ui_components::menu_components::{add_config_opt, BuilderResult, do_submenu_menu_building};
use crate::ui_components::menu_components::menu_types::dropdown_menu::set_parent;
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::DropdownMenuOptionResult;
use crate::ui_components::menu_components::menu_options::MenuOptionBuilder;
use crate::ui_components::menu_components::menu_options::slider_menu_option::SliderMenuOptionResult;
use crate::ui_components::menu_components::menu_types::submenu_builder::DrawSubmenuResult;
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct CollapsableMenuBuilder<'a> {
    pub(crate) parent: Option<Entity>,
    pub(crate) menu_metadata: &'a MenuItemMetadata,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parent_menus: Vec<MenuItemMetadata>,
    pub(crate) menu_option_builders: Vec<(MenuOption, MenuOptionBuilder<'a>)>,
    pub(crate) menu_component: UiComponent,
}

#[derive(Clone, Debug)]
pub struct DrawCollapsableMenuResult {
    pub(crate) dropdown_menu_option_results: Vec<DropdownMenuOptionResult>,
    pub(crate) submenu_results: Vec<DrawSubmenuResult>,
    pub(crate) slider: Vec<SliderMenuOptionResult>,
    pub(crate) collapsable_menu_button: Entity,
    pub(crate) menu_text_entity: Entity
}

impl BuilderResult for DrawCollapsableMenuResult {}

impl DrawCollapsableMenuResult {
    fn new(
        collapsable_menu_button: Entity,
        menu_text_entity: Entity
    )  -> Self {
        Self {
            dropdown_menu_option_results: vec![],
            submenu_results: vec![],
            slider: vec![],
            collapsable_menu_button, menu_text_entity,
        }
    }
}

impl <'a> CollapsableMenuBuilder<'a> {
    pub(crate) fn build(&'a mut self, mut commands: &mut Commands,
                        mut materials: &mut ResMut<Assets<ColorMaterial>>,
                        mut meshes: &mut ResMut<Assets<Mesh>>,
                        mut asset_server: &mut Res<AssetServer>,) -> DrawCollapsableMenuResult {

        let collapsable_menu_parent = commands.spawn(self.collapsable_button()).id();
        info!("Collapsable menu: {:?}", collapsable_menu_parent);
        let text_child = commands.spawn(self.text_child(&mut asset_server)).id();

        commands.get_entity(collapsable_menu_parent)
            .as_mut()
            .map(|e| e.add_child(text_child));

        commands.get_entity(self.parent.unwrap())
            .as_mut()
            .map(|root_parent| root_parent.add_child(collapsable_menu_parent));

        let mut collapsable = DrawCollapsableMenuResult::new(collapsable_menu_parent, text_child);

        let (submenu, menu_option, slider)
            = do_submenu_menu_building(
            &mut commands,
            &mut self.menu_option_builders,
            &None,
            &Some(&collapsable),
            materials,
            meshes,
            asset_server
        );

        collapsable.submenu_results = submenu;
        collapsable.dropdown_menu_option_results = menu_option;
        collapsable.slider = slider;

        add_config_opt(commands, Some(collapsable_menu_parent), self.config_option);

        collapsable
    }

    pub(crate) fn collapsable_button(&self) -> impl Bundle {
        (
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
            PropagateDisplay::default(),
            self.menu_component.clone(),
            UiIdentifiableComponent(self.menu_metadata.id),
        )
    }

    pub(crate) fn text_child(&self, mut asset_server: &mut Res<AssetServer>) -> impl Bundle {
        (
            TextBundle {
                style: Style {
                    size: Size::new(Val::Percent(95.0), Val::Px(30.0)),
                    ..default()
                },
                text: Text::from_section(self.menu_metadata.name.to_string(), TextStyle {
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    color: Color::BLACK,
                    ..default()
                }),
                ..default()
            },
            PropagateDisplay::default()
        )
    }
}

