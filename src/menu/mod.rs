use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Color, Component, Entity};
use bevy::ui::{Size, Val};
use bevy::utils::petgraph::visit::Data;
use bevy_mod_picking::Selection;
use crate::metrics::{Metric, MetricChildNodes};
use crate::network::{Layer, Network, Node};

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
    pub(crate) font: MenuItemFont,
    pub(crate) name: String,
    pub(crate) icon_pos: Position,
    pub(crate) size: Option<Size>,
    pub(crate) color: Color,
    pub(crate) description: String,
    pub(crate) id: f32
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


#[derive(Clone, Debug)]
pub enum MenuInputType {
    Dropdown {
        /// Maybe want to make this a tuple to add type information, because may not be able
        /// to know which Component type the Interaction will be with.
        options: Vec<MenuOption>,
        metadata: MenuItemMetadata,
        option: ConfigurationOptionEnum
    },
    CollapsableMenu {
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
pub enum ConfigurationOption<T: Component + Send + Sync + Clone + Debug + Default + 'static> {
    Variance(PhantomData<T>, DataType),
    Concavity(PhantomData<T>, DataType),
    Metrics(PhantomData<T>, DataType),
    Menu(PhantomData<T>, DataType)
}

/// When you select an option, there are a few things to keep in mind:
/// 1. When you select an option it may deselect other options. Or it may not.
/// This means that when an option is selected, there needs to be a way to query all of the other options
/// that exist in order to modify or delete them.
#[derive(Component, Clone, Debug)]
pub struct ConfigurationOptionComponent<T: Component + Send + Sync + Clone + Debug + Default + 'static> {
    phantom: PhantomData<T>,
    configuration_option: ConfigurationOption<T>,
    value: DataType,
}

impl <T: Component + Send + Sync + Clone + Debug + Default + 'static> Default for ConfigurationOption<T> {
    fn default() -> Self {
        ConfigurationOption::Variance(PhantomData::default(), DataType::Number(Some(0.0)))
    }
}

pub trait AcceptConfigurationOption<T> where Self: Component + Clone + Default + Debug {
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

#[derive(Clone, Debug)]
pub struct MenuOption {
    pub(crate) data_type: MenuOptionType,
    pub(crate) index: usize,
    pub(crate) metadata: MenuItemMetadata
}

#[derive(Clone, Debug, Component, Default)]
pub struct Menu;

#[derive(Clone, Debug, Component)]
pub enum ConfigurationOptionEnum {
    Menu(ConfigurationOption<Menu>),
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

impl Default for ConfigurationOptionEnum {
    fn default() -> Self {
        ConfigurationOptionEnum::Menu(ConfigurationOption::Menu(PhantomData::default(), DataType::Selected))
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
