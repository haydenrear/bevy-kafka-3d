use std::default::default;
use bevy::prelude::*;
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::StateChange::ChangeComponentStyle;
use crate::menu::{ConfigurationOptionEnum, DraggableComponent, MenuInputType, MenuItemMetadata, MenuOption, Slider, SliderKnob, UiComponent};
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{StateChangeActionType, UiComponentState, UiComponentStateTransition, UiComponentStateTransitions};
use crate::ui_components::ui_components::base_menu::BaseMenu;
use crate::ui_components::ui_components::BuilderResult;
use crate::ui_components::ui_menu_component::{insert_config_option, UiIdentifiableComponent};

pub struct SliderMenuOptionBuilder<'a> {
    pub(crate) parent: Option<Entity>,
    menu_option: &'a MenuOption,
    config_option: &'a ConfigurationOptionEnum,
    parents: &'a Vec<MenuItemMetadata>,
    menu_input_type: Option<&'a MenuInputType>,
    menu_option_component: UiComponent,
    metadata: MenuItemMetadata,
    id_component: UiIdentifiableComponent,
}

#[derive(Clone, Debug)]
pub struct SliderMenuOptionResult {
    pub(crate) slider_knob_entity: Entity,
    pub(crate) text_entity: Entity,
    pub(crate) slider_entity: Entity
}

impl SliderMenuOptionResult {
    fn new(
        slider_knob_entity: Entity,
        text_entity: Entity,
        slider_entity: Entity
    ) -> Self {
        Self {
            slider_knob_entity, text_entity, slider_entity
        }
    }
}

impl BuilderResult for SliderMenuOptionResult {}

impl <'a> SliderMenuOptionBuilder<'a> {

    pub(crate) fn build(
        &self,
        mut commands: &mut Commands,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut asset_server: &mut Res<AssetServer>,
    ) -> SliderMenuOptionResult {
        let text_entity = commands.spawn(self.spawn_text_value(&mut asset_server)).id();
        let slider_knob = commands.spawn(self.spawn_slider_knob()).id();

        let mut slider_entity = commands.spawn(self.spawn_base());

        slider_entity.push_children(vec![text_entity, slider_knob].as_slice());

        let slider_entity = slider_entity.id();

        SliderMenuOptionResult::new(slider_knob, text_entity, slider_entity)
    }

    pub(crate) fn spawn_base(&self) -> impl Bundle {
        (
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    // size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::GREEN),
                ..default()
            },
            UiIdentifiableComponent(20.0),
            self.menu_option_component.clone(),
            Label,
        )
    }

    pub(crate) fn spawn_text_value(&self, mut asset_server: &mut Res<AssetServer>) -> impl Bundle {
        (
            TextBundle {
                style: Style {
                    display: Display::Flex,
                    size: Size::new(Val::Percent(100.0), Val::Px(50.0)),
                    ..default()
                },
                text: Text::from_section(self.metadata.initial_value.clone(), TextStyle {
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

    pub(crate) fn spawn_slider_knob(&self) -> impl Bundle {
        (
            ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    position: UiRect::left(Val::Px(30.0)),
                    size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::ORANGE),
                ..default()
            },
            UiComponent::SliderKnob(SliderKnob::default()),
            DraggableComponent::default(),
            UiComponentStateTransitions {
                transitions: vec![
                    UiComponentStateTransition {
                        filter_state: UiComponentState::Any,
                        state_change: vec![StateChangeActionType::Dragged(
                            ChangeComponentStyle(ChangeStyleTypes::DragX)
                        )],
                        propagation: ChangePropagation::SelfChange(
                            Relationship::SelfState
                        ),
                        current_state_filter: UiComponentState::Any,
                    }
                ],
            },
            UiIdentifiableComponent(20.0)
        )
    }

}
