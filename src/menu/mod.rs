use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Color, Commands, Component, Entity, ResMut};
use bevy::ui::{Size, Val};
use bevy::utils::petgraph::visit::Data;
use bevy_mod_picking::Selection;
use crate::event::event_state::{Context, UpdateStateInPlace};
use crate::graph::Graph;
use crate::menu::config_menu_event::config_event::NextConfigurationOptionState;
use crate::menu::menu_resource::{MENU, VARIANCE};
use crate::metrics::network_metrics::{Metric, MetricChildNodes};
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
pub enum MetricsConfigurationOption<T: Component + Send + Sync + Clone + Debug + Default + 'static> {
    Variance(PhantomData<T>, DataType, &'static str),
    Concavity(PhantomData<T>, DataType, &'static str),
    Metrics(PhantomData<T>, DataType, &'static str),
    Menu(PhantomData<T>, DataType, &'static str)
}

impl <T, Ctx> UpdateStateInPlace<MetricsConfigurationOption<T>, Ctx>
for MetricsConfigurationOption<T>
where T: Component + Send + Sync + Clone + Debug + Default + 'static,
    Ctx: Context
{
    fn update_state(&self,commands: &mut Commands, value: &mut MetricsConfigurationOption<T>, ctx: &mut Option<ResMut<Ctx>>) {
        *value = self.clone()
    }
}

impl <T: Component + Send + Sync + Clone + Debug + Default + 'static> MetricsConfigurationOption<T> {
    pub(crate) fn get_data(&self) -> &DataType {
        match self {
            MetricsConfigurationOption::Variance(_, data, _) => { data }
            MetricsConfigurationOption::Concavity(_, data, _) => { data }
            MetricsConfigurationOption::Metrics(_, data, _) => { data }
            MetricsConfigurationOption::Menu(_, data, _) => { data }
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
            MetricsConfigurationOption::Menu(_, _, id) => {
                id
            }
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
    pub(crate) metadata: MenuItemMetadata
}

#[derive(Clone, Debug, Component, Default)]
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
    DisplayGraph(MetricsConfigurationOption<Menu>),
    DisplayNetwork(MetricsConfigurationOption<Graph>),
}

impl ConfigurationOptionEnum {
    pub(crate) fn update_data(&mut self, data: DataType) {
        match self {
            ConfigurationOptionEnum::Menu(MetricsConfigurationOption::Menu(_, a, _)) => {
                *a = data;
            }
            ConfigurationOptionEnum::Metrics(_) => {}
            ConfigurationOptionEnum::NetworkMetrics(_) => {}
            ConfigurationOptionEnum::NetworkVariance(_) => {}
            ConfigurationOptionEnum::NetworkConcavity(_) => {}
            ConfigurationOptionEnum::LayerMetrics(_) => {}
            ConfigurationOptionEnum::LayerVariance(_) => {}
            ConfigurationOptionEnum::LayerConcavity(_) => {}
            ConfigurationOptionEnum::NodeMetrics(_) => {}
            ConfigurationOptionEnum::NodeVariance(_) => {}
            ConfigurationOptionEnum::NodeConcavity(_) => {}
            _ => {}
        };
    }
}

impl Default for ConfigurationOptionEnum {
    fn default() -> Self {
        ConfigurationOptionEnum::Menu(MetricsConfigurationOption::Menu(PhantomData::default(), DataType::Selected, MENU))
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
    Deselect
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
