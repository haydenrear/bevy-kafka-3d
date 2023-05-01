use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Color, Component, Entity};
use bevy::ui::Val;
use bevy::utils::petgraph::visit::Data;
use bevy_mod_picking::Selection;
use crate::metrics::{Metric, MetricChildNodes};
use crate::network::{Layer, Network, Node};

pub(crate) mod menu_event;
pub(crate) mod menu_resource;
pub(crate) mod interaction_ui_event_writer_system;
pub(crate) mod interaction_ui_event_reader;
pub(crate) mod interaction_config_event_writer_system;
pub(crate) mod interaction_config_event_reader_system;

pub struct MenuData {
    sub_menus: Vec<SubMenu>,
    selectables: Vec<MenuInputs>
}

macro_rules! menu_data {
    ($($menu_input_ty:ty, $menu_input_ident:ident),*, $($submenu_ty:ty, $submenu_ident:ident),*) => {
        pub struct MenuData {
            $(
                $menu_input_ident: $menu_input_ty,
                $submenu_ident: $submenu_ty
            )*
        }
    }
}

pub struct SubMenu {
    selectables: Vec<MenuInputs>
}

#[derive(Default, Clone, Debug)]
pub struct MenuItemMetadata {
    icon: String,
    font: MenuItemFont,
    name: String,
    icon_pos: Position,
    height: u32,
    width: u32,
    color: Color,
    description: String
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

pub enum MenuInputs {
    Dropdown {
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
    ContinuousMovingButton {
        metadata: MenuItemMetadata,
        start: u32,
        end: u32,
        option: ConfigurationOptionEnum
    }
}

/// Query by the T in ConfigurationOption, and then query by the T component in order to apply
/// the configuration option
#[derive(Component, Debug, Clone)]
pub enum ConfigurationOption<T: Component + Send + Sync + 'static> {
    Variance(PhantomData<T>, DataType),
    Concavity(PhantomData<T>, DataType),
    Metrics(PhantomData<T>, DataType)
}


/// When you select an option, there are a few things to keep in mind:
/// 1. When you select an option it may deselect other options. Or it may not.
/// This means that when an option is selected, there needs to be a way to query all of the other options
/// that exist in order to modify or delete them.
#[derive(Component, Clone, Debug)]
pub struct ConfigurationOptionComponent<T: Component + Send + Sync + 'static> {
    phantom: PhantomData<T>,
    configuration_option: ConfigurationOption<T>,
    value: DataType,
}

impl <T: Component + Send + Sync + 'static> Default for ConfigurationOption<T> {
    fn default() -> Self {
        ConfigurationOption::Variance(PhantomData::default(), DataType::Number(Some(0.0)))
    }
}

pub trait AcceptConfigurationOption<T> where Self: Component {
    fn accept_configuration_option(value: ConfigurationOption<Self>, args: T)
    where Self: Sized;
}

impl AcceptConfigurationOption<()> for Node {
    fn accept_configuration_option(value: ConfigurationOption<Node>, args: ()) {
        todo!()
    }
}

impl AcceptConfigurationOption<Vec<Node>> for MetricChildNodes {
    fn accept_configuration_option(value: ConfigurationOption<MetricChildNodes>, nodes: Vec<Node>) {
        todo!()
    }
}

pub struct MenuOption {
    data_type: MenuOptionType,
    metadata: MenuItemMetadata
}

pub enum ConfigurationOptionEnum {
    Metrics(ConfigurationOption<Metric>),
    NetworkMetrics(ConfigurationOption<Network>),
    NetworkVariance(ConfigurationOption<Network>),
    NetworkConcavity(ConfigurationOption<Network>),
    LayerMetrics(ConfigurationOption<Layer>),
    LayerVariance(ConfigurationOption<Layer>),
    LayerConcavity(ConfigurationOption<Layer>),
    NodeMetrics(ConfigurationOption<Node>),
    NodeVariance(ConfigurationOption<Node>),
    NodeConcavity(ConfigurationOption<Node>)
}

pub enum MenuOptionType {
    Primitive(ConfigurationOptionEnum, DataType),
    SubMenu {
        sub_menu: MenuInputs,
        parent: MenuItemMetadata,
        config_option: ConfigurationOptionEnum
    },
    MenuRef {
        sub_menu: MenuInputs,
        parent: MenuItemMetadata,
        config_option: ConfigurationOptionEnum
    }
}

/// Contains the default value.
#[derive(Clone, Debug)]
pub enum DataType {
    Number(Option<f32>),
    String(Option<String>),
    Selected
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String(None)
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Dropdown {
    pub(crate) selected_index: usize,
    pub(crate) options: Vec<String>
}

#[derive(Component, Clone, Debug, Default)]
pub struct CollapsableMenu {
}

#[derive(Component, Default, Clone, Debug)]
pub struct DropdownOption {
    pub(crate) index: usize,
    pub(crate) option_name: String
}
