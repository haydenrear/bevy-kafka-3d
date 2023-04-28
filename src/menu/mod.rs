use bevy::prelude::Entity;
use bevy::utils::petgraph::visit::Data;
use bevy_mod_picking::Selection;

mod menu_event;

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