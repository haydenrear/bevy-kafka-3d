use std::default::default;
use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::log::info;
use bevy::prelude::{AlignSelf, BackgroundColor, Bundle, ButtonBundle, Color, Commands, Component, Display, Entity, FromReflect, Reflect, ResMut, Style, UiRect};
use bevy::ui::{FlexDirection, Size, Val};
use bevy::utils::petgraph::visit::Data;
use bevy_mod_picking::Selection;
use rdkafka::metadata;
use serde::Deserialize;
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::{Context, UpdateStateInPlace};
use crate::event::event_state::StateChange::ChangeComponentStyle;
use crate::graph::GraphParent;
use crate::menu::config_menu_event::config_event::NextConfigurationOptionState;
use crate::menu::menu_resource::{MENU, VARIANCE};
use crate::menu::ui_menu_event::change_style::ChangeStyleTypes;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{DisplayState, EntitiesStateTypes, EntityComponentStateTransition, SizeState, StateChangeActionType, UiComponentState, UiComponentStateTransition, UiComponentStateTransitions, UiEntityComponentStateTransitions};
use crate::metrics::network_metrics::Metric;
use crate::network::{Layer, MetricChildNodes, Network, Node};
use crate::ui_components::ui_components::{BuilderResult, BuildMenuResult};
use crate::ui_components::ui_components::base_menu::BuildBaseMenuResult;
use crate::ui_components::ui_components::collapsable_menu::{CollapsableMenuBuilder, DrawCollapsableMenuResult};
use crate::ui_components::ui_components::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::ui_components::menu_options::dropdown_menu_option::SelectionMenuOptionBuilderResult;
use crate::ui_components::ui_menu_component::UiIdentifiableComponent;

pub(crate) mod ui_menu_event;
pub(crate) mod config_menu_event;
pub(crate) mod menu_resource;

pub struct MenuData {
    pub(crate) sub_menus: Vec<SubMenu>,
    pub(crate) selectables: Vec<MenuInputType>
}

pub struct SubMenu {
    pub(crate) selectables: Vec<MenuInputType>
}

#[derive(Default, Clone, Debug)]
pub struct MenuItemMetadata {
    pub(crate) icon: String,
    pub(crate) name: String,
    pub(crate) initial_value: String,
    pub(crate) icon_pos: Position,
    pub(crate) font: Option<MenuItemFont>,
    pub(crate) size: Option<Size>,
    pub(crate) color: Option<Color>,
    pub(crate) description: String,
    pub(crate) id: f32,
    pub(crate) swing_out: bool
}


#[derive(Clone, Debug)]
pub struct MenuItemFont {
    font: String
}

impl Default for MenuItemFont {
    fn default() -> Self {
        Self {
            font: "fonts/FiraSans-Bold.ttf".to_string()
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum Position {
    Left,
    #[default]
    Middle,
    Right
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum MenuOptionInputType {
    Selected,
    Radial,
    FormInput,
    Slider,
    DropdownMenu,
    CollapsableMenu,
    SubMenu
}

#[derive(Clone, Debug)]
pub enum MenuInputType {
    Dropdown {
        /// Maybe want to make this a tuple to add type information, because may not be able
        /// to know which Component type the Interaction will be with.
        options: Vec<MenuOption>,
        metadata: MenuItemMetadata,
        option: ConfigurationOptionEnum,
    },
    CollapsableMenu {
        options: Vec<MenuOption>,
        metadata: MenuItemMetadata,
        option: ConfigurationOptionEnum
    },
    ScrollableMenu {
        options: Vec<MenuOption>,
        metadata: MenuItemMetadata,
        option: ConfigurationOptionEnum
    },
    Radial{
       options: Vec<MenuOption>,
        metadata: MenuItemMetadata,
        option: ConfigurationOptionEnum
    },
    FormInput {
        name: String,
        metadata: MenuItemMetadata,
        option: ConfigurationOptionEnum
    },
    Slider {
        metadata: MenuItemMetadata,
        slider_data: SliderData,
        option: ConfigurationOptionEnum
    }
}

pub(crate) fn update_style(style: &mut Style, metadata: MenuItemMetadata) {
    metadata.size.map(|size| {
        style.size = size;
    });
}



impl MenuInputType {

    // pub(crate) fn node_bundle(&self, component: UiComponent) -> impl Bundle {
    //     match self {
    //         MenuInputType::Dropdown { metadata, option, options } => {
    //             (
    //                 ButtonBundle {
    //                     style: Style {
    //                         display: component.starting_display(),
    //                         flex_direction: FlexDirection::Column,
    //                         align_self: AlignSelf::Start,
    //                         size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
    //                         position: if metadata.swing_out {
    //                             UiRect::new(Val::Percent(100.0), Val::Percent(0.0), Val::Percent(0.0), Val::Percent(0.0))
    //                         } else {
    //                             UiRect::default()
    //                         },
    //                         ..default()
    //                     },
    //                     background_color: BackgroundColor(Color::BLUE),
    //                     ..default()
    //                 },
    //                 UiIdentifiableComponent(metadata.id)
    //             )
    //         }
    //         MenuInputType::CollapsableMenu { metadata, .. } => {
    //             ()
    //         }
    //         MenuInputType::Radial { metadata, .. } => {
    //             ()
    //         }
    //         MenuInputType::FormInput { metadata, .. } => {
    //             ()
    //         }
    //         MenuInputType::Slider { metadata, .. } => {
    //             ()
    //         }
    //     }
    // }
    //
    // pub(crate) fn children(&self) -> impl Bundle {
    //     ()
    // }

    pub(crate) fn consumer(&self, mut commands: &mut Commands) {

    }

}

#[derive(Clone, Debug)]
pub struct SliderData {
    start: u32,
    end: u32
}

/// Query by the T in ConfigurationOption, and then query by the T component in order to apply
/// the configuration option
#[derive(Component, Debug, Clone)]
pub enum MetricsConfigurationOption<T: Component + Send + Sync + Clone + Debug + Default + 'static> {
    Variance(PhantomData<T>, DataType, &'static str),
    Concavity(PhantomData<T>, DataType, &'static str),
    Metrics(PhantomData<T>, DataType, &'static str),
    GraphMenu(PhantomData<T>, DataType, &'static str, MenuType),
    NetworkMenu(PhantomData<T>, DataType, &'static str, MenuType),
}

#[derive(Debug, Clone)]
pub enum MenuType {
    Graph, Network, Metrics, Menu
}

impl <T, Ctx> UpdateStateInPlace<MetricsConfigurationOption<T>, Ctx>
for MetricsConfigurationOption<T>
where T: Component + Send + Sync + Clone + Debug + Default + 'static,
    Ctx: Context
{
    fn update_state(&self,commands: &mut Commands, value: &mut MetricsConfigurationOption<T>, ctx: &mut ResMut<Ctx>) {
        info!("Updating state from {:?} to {:?}.", value, &self);
        *value = self.clone()
    }
}

impl <T: Component + Send + Sync + Clone + Debug + Default + 'static> MetricsConfigurationOption<T> {
    pub(crate) fn get_data(&self) -> &DataType {
        match self {
            MetricsConfigurationOption::Variance(_, data, _) => { data }
            MetricsConfigurationOption::Concavity(_, data, _) => { data }
            MetricsConfigurationOption::Metrics(_, data, _) => { data }
            MetricsConfigurationOption::GraphMenu(_, data, _, _) => { data }
            MetricsConfigurationOption::NetworkMenu(_, data, _, _) =>  data
        }
    }
}

impl <T: Component + Send + Sync + Clone + Debug + Default + 'static> MetricsConfigurationOption<T> {
    pub(crate) fn get_id(&self) -> &'static str{
        match self {
            MetricsConfigurationOption::Variance(_, _, id) => {
                id
            }
            MetricsConfigurationOption::Concavity(_, _, id) => {
                id
            }
            MetricsConfigurationOption::Metrics(_, _, id) => {
                id
            }
            MetricsConfigurationOption::GraphMenu(_, _, id, _) => {
                id
            }
            MetricsConfigurationOption::NetworkMenu(_, _, id, _) => id
        }
    }
}


/// When you select an option, there are a few things to keep in mind:
/// 1. When you select an option it may deselect other options. Or it may not.
/// This means that when an option is selected, there needs to be a way to query all of the other options
/// that exist in order to modify or delete them.
#[derive(Component, Clone, Debug)]
pub struct ConfigurationOptionComponent<T: Component + Send + Sync + Clone + Debug + Default + 'static> {
    phantom: PhantomData<T>,
    configuration_option: MetricsConfigurationOption<T>,
    value: DataType,
}

impl <T: Component + Send + Sync + Clone + Debug + Default + 'static> Default
for MetricsConfigurationOption<T> {
    fn default() -> Self {
        MetricsConfigurationOption::Variance(PhantomData::default(), DataType::Number(Some(0.0)), VARIANCE)
    }
}

pub trait AcceptConfigurationOption<T> where Self: Component + Clone + Default + Debug {
    fn accept_configuration_option(value: MetricsConfigurationOption<Self>, args: T)
    where Self: Sized;
}

impl AcceptConfigurationOption<()> for Node {
    fn accept_configuration_option(value: MetricsConfigurationOption<Node>, args: ()) {
        todo!()
    }
}

impl AcceptConfigurationOption<Vec<Node>> for MetricChildNodes {
    fn accept_configuration_option(value: MetricsConfigurationOption<MetricChildNodes>, nodes: Vec<Node>) {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct MenuOption {
    pub(crate) data_type: MenuOptionType,
    pub(crate) index: usize,
    pub(crate) metadata: MenuItemMetadata,
    pub(crate) swing_out: bool,
    pub(crate) ui_option_type: MenuOptionInputType
}

#[derive(Clone, Debug, Component, Default, Deserialize, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Menu;

#[derive(Clone, Debug, Component)]
pub enum ConfigurationOptionEnum {
    Menu(MetricsConfigurationOption<Menu>),
    Metrics(MetricsConfigurationOption<Metric<Network>>),
    NetworkMetrics(MetricsConfigurationOption<Network>),
    NetworkVariance(MetricsConfigurationOption<Network>),
    NetworkConcavity(MetricsConfigurationOption<Network>),
    LayerMetrics(MetricsConfigurationOption<Layer>),
    LayerVariance(MetricsConfigurationOption<Layer>),
    LayerConcavity(MetricsConfigurationOption<Layer>),
    NodeMetrics(MetricsConfigurationOption<Node>),
    NodeVariance(MetricsConfigurationOption<Node>),
    NodeConcavity(MetricsConfigurationOption<Node>),
}

impl ConfigurationOptionEnum {
    pub(crate) fn get_option_input_enum(&self) -> MenuOptionInputType {
        MenuOptionInputType::Selected
    }
}

impl Default for ConfigurationOptionEnum {
    fn default() -> Self {
        ConfigurationOptionEnum::Menu(
            MetricsConfigurationOption::GraphMenu(PhantomData::default(), DataType::Selected, MENU, MenuType::Menu),
        )
    }
}


#[derive(Clone, Debug)]
pub enum MenuOptionType {
    Primitive(ConfigurationOptionEnum),
    SubMenu {
        sub_menu: MenuInputType,
        parent: MenuItemMetadata,
        config_option: ConfigurationOptionEnum
    },
}

/// Contains the default value.
#[derive(Clone, Debug)]
pub enum DataType {
    Number(Option<f32>),
    String(Option<String>),
    Selected,
    Deselected,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String(None)
    }
}

pub trait UiBundled {
    fn get_bundle(&self, commands: &mut Commands, menu_metadata: &MenuItemMetadata) -> Entity;
}

#[derive(Component, Default, Clone, Debug)]
pub struct Dropdown {
    pub(crate) selected_index: usize,
    pub(crate) options: Vec<String>
}

#[derive(Component, Clone, Debug, Default)]
pub struct CollapsableMenu {
}

#[derive(Component, Clone, Debug, Default)]
pub struct Slider {
}

#[derive(Component, Clone, Debug, Default)]
pub struct SliderKnob {
}

#[derive(Component, Clone, Debug, Default)]
pub struct Radial {
}

#[derive(Component, Clone, Debug, Default)]
pub struct RadialButton {
}

#[derive(Component, Clone, Debug, Default)]
pub struct RadialButtonSelection {
}

#[derive(Component, Default, Clone, Debug)]
pub struct DropdownOption {
    pub(crate) index: usize,
    pub(crate) option_name: String
}

#[derive(Component, Debug, Clone, Default)]
pub struct DraggableComponent {
    pub(crate) drag_x: bool,
    pub(crate) drag_y: bool
}

#[derive(Component, Debug, Clone, Default)]
pub struct ScrollableComponent {
}

#[derive(Component, Debug, Clone, Default)]
pub struct ScrollWheelComponent {
}

#[derive(Component, Debug, Clone, Default)]
pub struct ScrollingSidebarComponent {
}

#[derive(Component, Debug, Clone, Default)]
pub struct ScrollableMenuComponent {
}

#[derive(Component, Debug, Clone, Default)]
pub struct ScrollableMenuItemsBarComponent {
}

/// The event writer will take the component and it will create the event descriptor, and then
/// pass it on to the event reader, which will read the event. There will be a generic event reader
/// function, and the generic event reader function will be generic over the UiComponentStateFactory
/// and the EventData type. The UiEvents will then be read in that generic function and the state
/// will be updated by the UiComponentStateFactory.
#[derive(Component, Debug, Clone)]
pub enum UiComponent {
    Dropdown(Dropdown),
    MenuOption(DropdownOption),
    CollapsableMenuComponent(CollapsableMenu),
    SlideComponent(Slider),
    SliderKnob(SliderKnob),
    RadialComponent(Radial),
    RadialButton(RadialButton),
    RadialSelection(RadialButtonSelection),
    ScrollableMenuComponent(ScrollableMenuComponent),
    ScrollWheel(ScrollWheelComponent),
    ScrollingSidebar(ScrollingSidebarComponent),
    ScrollableMenuItemsBar(ScrollableMenuItemsBarComponent),
    Node,
}

pub trait GetStateTransitions<T: BuilderResult> {

    fn get_state_transitions(builder_result: &T, entities: &Entities) -> Option<UiEntityComponentStateTransitions>;

    fn change_child(style_type: ChangeStyleTypes, entities: &Vec<Entity>) -> Vec<(Entity, Relationship, StateChangeActionType)> {
        let mut change_visisble: Vec<(Entity, Relationship, StateChangeActionType)> = entities
            .iter()
            .map(|e| {
                info!("Adding child for change visible: {:?}, {:?}.", e, &style_type);
                e
            })
            .map(|entity| (
                *entity,
                Relationship::Child,
                StateChangeActionType::Clicked(ChangeComponentStyle(style_type.clone()))
            ))
            .collect();
        change_visisble
    }

    fn change_visible_self(build_menu_result: &Entities, style_type: ChangeStyleTypes) -> Vec<(Entity, Relationship, StateChangeActionType)> {
        vec![(
                build_menu_result.self_state,
                Relationship::SelfState,
                StateChangeActionType::Clicked(ChangeComponentStyle(style_type.clone()))
        )]
    }

}

pub struct Entities {
    pub(crate) siblings: Vec<Entity>,
    pub(crate) children: Vec<Entity>,
    pub(crate) siblings_children: Vec<Entity>,
    pub(crate) siblings_children_recursive: Vec<Entity>,
    pub(crate) parent: Vec<Entity>,
    pub(crate) self_state: Entity,
    pub(crate) children_recursive: Vec<Entity>
}

impl GetStateTransitions<BuildBaseMenuResult> for DrawDropdownMenuResult {
    fn get_state_transitions(
        builder_result: &BuildBaseMenuResult,
        build_menu_result: &Entities,
    ) -> Option<UiEntityComponentStateTransitions> {

        let remove_visible = Self::change_child(ChangeStyleTypes::RemoveVisible, &build_menu_result.children_recursive);
        let add_visible = Self::change_child(ChangeStyleTypes::AddVisible, &build_menu_result.children);
        let change_visible = Self::change_child(ChangeStyleTypes::ChangeVisible, &build_menu_result.children);

        let mut siblings: Vec<(Entity, Relationship, StateChangeActionType)> = build_menu_result.siblings_children_recursive
            .iter()
            .map(|entity| (
                *entity,
                Relationship::SiblingChild,
                StateChangeActionType::Clicked(ChangeComponentStyle(ChangeStyleTypes::RemoveVisible))
            ))
            .collect();

        info!("{:?} are the sibling recursive.", &siblings);

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: change_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                    },
                    EntityComponentStateTransition {
                        entity_to_change: EntitiesStateTypes {
                            states: siblings
                        },
                        filter_state: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayAny),
                    },
                ]
            }
        )
    }
}


impl GetStateTransitions<DrawCollapsableMenuResult> for DrawCollapsableMenuResult {
    fn get_state_transitions(
        builder_result: &DrawCollapsableMenuResult,
        build_menu_result: &Entities,
    ) -> Option<UiEntityComponentStateTransitions> {

        let mut add_visible = Self::change_child( ChangeStyleTypes::AddVisible, &build_menu_result.children);

        let mut remove_visible_recurs = Self::change_child(ChangeStyleTypes::RemoveVisible, &build_menu_result.children_recursive);

        let mut self_change_minimize = Self::change_visible_self(build_menu_result, ChangeStyleTypes::UpdateSize {
            height_1: 100.0,
            width_1: 20.0,
        });

        let mut self_change_maximize = Self::change_visible_self(build_menu_result, ChangeStyleTypes::UpdateSize {
            height_1: 100.0,
            width_1: 4.0,
        });

        Some(
            UiEntityComponentStateTransitions {
                transitions: vec![
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Expanded{
                            height: 100,
                            width: 20
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: remove_visible_recurs,
                        },
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayFlex),
                    },
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: add_visible,
                        },
                        current_state_filter: UiComponentState::StateDisplay(DisplayState::DisplayNone),
                    },
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Minimized{
                            height: 100,
                            width: 4
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: self_change_minimize
                        },
                        current_state_filter: UiComponentState::StateSize(SizeState::Minimized {
                            height: 100,
                            width: 4
                        }),
                    },
                    EntityComponentStateTransition{
                        filter_state: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20
                        }),
                        entity_to_change: EntitiesStateTypes {
                            states: self_change_maximize,
                        },
                        current_state_filter: UiComponentState::StateSize(SizeState::Expanded {
                            height: 100,
                            width: 20
                        }),
                    },
                ],
            }
        )
    }
}

impl UiComponent {
    pub(crate) fn starting_display(&self) -> Display {
        match self {
            UiComponent::RadialComponent(_) => Display::Flex,
            _ => Display::None
        }
    }
}
