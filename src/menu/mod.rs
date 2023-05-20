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
use ui_menu_event::next_action::{DisplayState, SizeState, UiComponentState};
use ui_menu_event::types::{StyleStateChange, UiStateChange, UiStyleComponentStateTransitions};
use crate::event::event_propagation::{ChangePropagation, Relationship};
use crate::event::event_state::{Context, StyleStateChangeEventData, UpdateStateInPlace};
use crate::event::event_state::StyleStateChangeEventData::ChangeComponentStyle;
use crate::graph::GraphParent;
use crate::menu::config_menu_event::config_event::NextConfigurationOptionState;
use crate::menu::menu_resource::{MENU, VARIANCE};
use crate::menu::ui_menu_event::change_style::UiChangeTypes;
use crate::menu::ui_menu_event::ui_context::UiContext;
use crate::menu::ui_menu_event::ui_menu_event_plugin::{EntitiesStateTypes, EntityComponentStateTransition, StateChangeActionType, UiComponentStateTransition, UiComponentStateTransitions, UiEntityComponentStateTransitions, UiEventArgs};
use crate::metrics::network_metrics::Metric;
use crate::network::{Layer, MetricChildNodes, Network, Node};
use crate::ui_components::menu_components::{BuildMenuResult};
use crate::ui_components::menu_components::menu_types::dropdown_menu::{DrawDropdownMenuResult, DropdownMenuBuilder};
use crate::ui_components::menu_components::menu_options::dropdown_menu_option::{DropdownMenuOptionBuilder, DropdownMenuOptionResult};
use crate::ui_components::menu_components::menu_types::base_menu::BuildBaseMenuResult;
use crate::ui_components::menu_components::menu_types::collapsable_menu::DrawCollapsableMenuResult;
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

pub enum SelectableType {
    DropdownSelectableReplaceText,
    DropdownSelectableCheckmarkActivate,
    NotSelectable
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum MenuOptionInputType {
    Activated,
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
    CollapsableMenuInputType {
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
        MenuOptionInputType::Activated
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

#[derive(Component, Default, Clone, Debug)]
pub struct Dropdown {
    pub(crate) selected_index: usize,
    pub(crate) selectable: bool,
    pub(crate) options: Vec<String>
}

#[derive(Component, Clone, Debug, Default)]
pub struct CollapsableMenuComponent {
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

#[derive(Component, Default, Clone, Debug)]
pub struct DropdownSelected {
}

#[derive(Component, Default, Clone, Debug)]
pub struct DropdownName {
}

#[derive(Component, Debug, Clone, Default)]
pub struct DraggableComponent {
    pub(crate) drag_x: bool,
    pub(crate) drag_y: bool
}

#[derive(Component, Debug, Clone, Default)]
pub struct ScrollableComponent {
    /// when you enter a scrollable component, set is_attached to true, and then when you scroll,
    /// you
    pub(crate) is_attached: bool,
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
    CollapsableMenu(CollapsableMenuComponent),
    SlideComponent(Slider),
    SliderKnob(SliderKnob),
    RadialComponent(Radial),
    RadialButton(RadialButton),
    RadialSelection(RadialButtonSelection),
    ScrollableMenuComponent(ScrollableMenuComponent),
    ScrollWheel(ScrollWheelComponent),
    ScrollingSidebar(ScrollingSidebarComponent),
    ScrollableMenuItemsBar(ScrollableMenuItemsBarComponent),
    DropdownSelectable,
    NamedDropdownMenu,
    Node,
    MenuOptionCheckmark,
}

impl UiComponent {
    pub(crate) fn starting_display(&self) -> Display {
        match self {
            UiComponent::RadialComponent(_) => Display::Flex,
            _ => Display::None
        }
    }
}
