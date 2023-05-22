use std::default::default;
use bevy::prelude::*;
use crate::menu::{ConfigurationOptionEnum, MenuInputType, MenuItemMetadata, MenuOption, SelectableType, UiComponent};
use crate::menu::ui_menu_event::ui_menu_event_plugin::{PropagateDisplay, PropagateSelect, PropagateVisible};
use crate::ui_components::menu_components::{add_config_opt, BuilderResult, get_parent_entity, get_swing_out};
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct DropdownMenuOptionBuilder<'a> {
    pub(crate) base_menu_parent: Option<BuildBaseMenuResult>,
    pub(crate) collapsable_menu_parent: Option<DrawCollapsableMenuResult>,
    pub(crate) menu_option: &'a MenuOption,
    pub(crate) config_option: &'a ConfigurationOptionEnum,
    pub(crate) parents: Vec<MenuItemMetadata>,
    pub(crate) menu_option_component: UiComponent,
    pub(crate) id_component: UiIdentifiableComponent,
    pub(crate) selectable: SelectableType,
}

#[derive(Default, Clone, Debug)]
pub struct DropdownMenuOptionResult {
    pub(crate) menu_option_entity: Option<Entity>,
    pub(crate) breadcrumbs_entity: Option<Entity>,
    pub(crate) selected_checkmark_entity: Option<Entity>,
    pub(crate) text_entity: Option<Entity>,
    pub(crate) parent_entity: Option<Entity>,
    pub(crate) parent_text_entity: Option<Entity>,
    pub(crate) configuration_option: ConfigurationOptionEnum,
    pub(crate) dropdown_selectable_menu_option: bool
}

impl DropdownMenuOptionResult {
    pub(crate) fn new(
        configuration_option: ConfigurationOptionEnum
    ) -> Self {
        Self {
            configuration_option,
            ..default()
        }
    }
}

impl BuilderResult for DropdownMenuOptionResult {}

impl <'a> DropdownMenuOptionBuilder<'a> {

    pub(crate) fn build(
        &self,
        mut commands: &mut Commands,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut asset_server: &mut Res<AssetServer>,
    ) -> DropdownMenuOptionResult {

        let mut result = DropdownMenuOptionResult::new(self.config_option.clone());

        let mut menu_option_button = commands
            .spawn(self.menu_option_button());

        if matches!(self.selectable, SelectableType::DropdownSelectableChangeVisible) {
            menu_option_button.insert(PropagateVisible::default());
        }

        let mut menu_option_button = menu_option_button
            .id();

        result.menu_option_entity = Some(menu_option_button);

        commands.get_entity(menu_option_button)
            .as_mut()
            .map(|menu_option_button| menu_option_button
                .with_children(|child_builder| {
                    // creates a blue mark to show inside of submenu
                    result.breadcrumbs_entity = Some(child_builder.spawn(self.breadcrumbs_entity()).id());
                    result.text_entity = Some(child_builder.spawn(self.text_entity(&mut asset_server)).id());
                    if matches!(self.selectable, SelectableType::DropdownSelectableChangeVisible)
                        || matches!(self.selectable, SelectableType::DropdownSelectableCheckmarkActivate) {
                        result.selected_checkmark_entity = Some(child_builder.spawn(self.selected_entity()).id());
                        info!("{:?} is selected checkmark entity.", &result.selected_checkmark_entity);
                    }
                })
            );

        let parent_entity = get_parent_entity(
            &self.base_menu_parent.as_ref(),
            &self.collapsable_menu_parent.as_ref()
        ).unwrap();

        commands.get_entity(parent_entity)
            .as_mut()
            .map(|parent| {
                info!("Adding child to {:?}", parent_entity);
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
            PropagateSelect::default(),
            PropagateDisplay::default(),
            self.id_component.clone(),
            self.menu_option_component.clone()
        )
    }

    pub(crate) fn selected_entity(&self) -> impl Bundle {
        (
            NodeBundle {
                style: Style {
                    display: Display::None,
                    position: UiRect::right(Val::Percent(10.0)),
                    size: Size::all(Val::Percent(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::GREEN),
                ..default()
            },
            UiComponent::MenuOptionCheckmark,
            PropagateSelect::default(),
            self.id_component.clone()
        )
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
            PropagateDisplay::default(),
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
            PropagateDisplay::default(),
            self.id_component.clone()
        )
    }
}
