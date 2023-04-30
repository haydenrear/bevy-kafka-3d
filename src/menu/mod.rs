use std::marker::PhantomData;
use bevy::prelude::{Component, Entity};
use bevy::ui::Val;
use bevy::utils::petgraph::visit::Data;
use bevy_mod_picking::Selection;
use crate::metrics::{Metric, MetricChildNodes};
use crate::network::{Layer, Network, Node};

pub(crate) mod menu_event;

pub struct MenuData {
    sub_menus: Vec<SubMenu>,
    selectables: Vec<MenuInputs>,
    associated: Entity
}

pub struct SubMenu {
    selectables: Vec<MenuInputs>
}

pub struct MenuItemMetadata {
    icon: String,
    font: String,
    name: String,
    icon_pos: Position,
    height: u32,
    width: u32
}

pub enum Position {
    Left, Middle, Right
}

pub enum MenuInputs {
    Dropdown{
        options: Vec<MenuInputOptions>,
        metadata: MenuItemMetadata
    },
    Radial{
       options: Vec<MenuInputOptions>,
        metadata: MenuItemMetadata
    },
    FormInput {
        name: String,
        metadata: MenuItemMetadata
    },
    ContinuousMovingButton {
        metadata: MenuItemMetadata,
        start: u32,
        end: u32
    }
}

/// Query by the T in ConfigurationOption, and then query by the T component in order to apply
/// the configuration option
#[derive(Component)]
pub enum ConfigurationOption<T: Component> {
    Variance(PhantomData<T>)
}

impl <T: Component> Default for ConfigurationOption<T> {
    fn default() -> Self {
        ConfigurationOption::Variance(PhantomData::default())
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

pub struct MenuInputOptions {
    data_type: MenuOptionType
}

pub enum MenuOptionType {
    Primitive(DataType)
}


/// Contains the default value.
#[derive(Clone)]
pub enum DataType {
    Number(Option<u32>),
    String(Option<String>)
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
