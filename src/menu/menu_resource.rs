use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Component, Resource};
use bevy::utils::default;
use crate::menu::{ConfigurationOption, DataType, MenuData, MenuOption, MenuInputs, MenuItemMetadata, MenuOptionType, Position, ConfigurationOptionEnum};
use crate::metrics::Metric;
use crate::network::{Layer, Network, Node};

#[derive(Resource)]
pub struct MenuResource {
    menu_data: MenuData,
}



impl Default for MenuResource {
    fn default() -> Self {
        Self {
            //  metrics -- dropdown
            //  - layer metrics -- submenu
            //       - concavity
            //       - variance
            //  - node metrics -- submenu
            //       - concavity
            //       - variance
            //  - whole network metrics -- submenu
            //       - concavity
            //       - variance
            menu_data: MenuData {
                sub_menus: vec![],
                selectables: vec![
                    MenuInputs::Dropdown {
                        options: vec![

                            // network
                            MenuOption {
                                data_type: MenuOptionType::SubMenu {
                                    sub_menu: MenuInputs::Dropdown {
                                        options: vec![
                                            // concavity
                                            MenuOption {
                                                data_type: MenuOptionType::Primitive(
                                                    ConfigurationOptionEnum::NetworkConcavity(ConfigurationOption::Concavity(
                                                        PhantomData::<Network>::default(),
                                                        DataType::Selected
                                                    )), DataType::Selected),
                                                metadata: Default::default(),
                                            },
                                            // variance
                                            MenuOption {
                                                data_type: MenuOptionType::Primitive(
                                                    ConfigurationOptionEnum::NetworkVariance(ConfigurationOption::Variance(
                                                        PhantomData::<Network>::default(),
                                                        DataType::Selected
                                                    )), DataType::Selected),
                                                metadata: Default::default(),
                                            },
                                        ],
                                        metadata: MenuItemMetadata {
                                            name: "Network Metric Options".to_string(),
                                            icon_pos: Position::Left,
                                            height: 0,
                                            width: 0,
                                            color: Default::default(),
                                            description: "Show metrics for whole network ".to_string(),
                                            ..default()
                                        },
                                        option: ConfigurationOptionEnum::NetworkMetrics(ConfigurationOption::Metrics(
                                            PhantomData::<Network>::default(),
                                            DataType::Selected
                                        )),
                                    },
                                    parent: MenuItemMetadata {
                                        icon: "".to_string(),
                                        name: "".to_string(),
                                        icon_pos: Position::Left,
                                        height: 0,
                                        width: 0,
                                        color: Default::default(),
                                        ..default()
                                    },
                                    config_option: ConfigurationOptionEnum::NetworkMetrics(ConfigurationOption::Metrics(
                                        PhantomData::<Network>::default(),
                                        DataType::Selected
                                    )),
                                },
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    name: "Layer Metrics".to_string(),
                                    icon_pos: Position::Left,
                                    height: 100,
                                    width: 100,
                                    color: Default::default(),
                                    description: "Options for metrics for layers.".to_string(),
                                    ..default()
                                },
                            },

                            // layer
                            MenuOption {
                                data_type: MenuOptionType::SubMenu {
                                    sub_menu: MenuInputs::Dropdown {
                                        options: vec![
                                            // concavity
                                            MenuOption {
                                                data_type: MenuOptionType::Primitive(
                                                    ConfigurationOptionEnum::LayerConcavity(ConfigurationOption::Concavity(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Selected
                                                    )), DataType::Selected),
                                                metadata: Default::default(),
                                            },
                                            // variance
                                            MenuOption {
                                                data_type: MenuOptionType::Primitive(
                                                    ConfigurationOptionEnum::LayerVariance(ConfigurationOption::Variance(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Selected
                                                    )), DataType::Selected),
                                                metadata: Default::default(),
                                            },
                                        ],
                                        metadata: MenuItemMetadata {
                                            name: "".to_string(),
                                            icon_pos: Position::Left,
                                            height: 0,
                                            width: 0,
                                            color: Default::default(),
                                            description: "Show metrics for whole network ".to_string(),
                                            ..default()
                                        },
                                        option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                            PhantomData::<Layer>::default(),
                                            DataType::Selected
                                        )),
                                    },
                                    parent: MenuItemMetadata {
                                        icon: "".to_string(),
                                        name: "".to_string(),
                                        icon_pos: Position::Left,
                                        height: 0,
                                        width: 0,
                                        color: Default::default(),
                                        ..default()
                                    },
                                    config_option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                        PhantomData::<Layer>::default(),
                                        DataType::Selected
                                    )),
                                },
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    name: "Layer Metrics".to_string(),
                                    icon_pos: Position::Left,
                                    height: 100,
                                    width: 100,
                                    color: Default::default(),
                                    description: "Options for metrics for layers.".to_string(),
                                    ..default()
                                },
                            },

                            // node
                            MenuOption {
                                data_type: MenuOptionType::SubMenu {
                                    sub_menu: MenuInputs::Dropdown {
                                        options: vec![
                                            // concavity
                                            MenuOption {
                                                data_type: MenuOptionType::Primitive(
                                                    ConfigurationOptionEnum::NodeConcavity(ConfigurationOption::Concavity(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Selected
                                                    )), DataType::Selected),
                                                metadata: Default::default(),
                                            },
                                            // variance
                                            MenuOption {
                                                data_type: MenuOptionType::Primitive(
                                                    ConfigurationOptionEnum::NodeVariance(ConfigurationOption::Variance(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Selected
                                                    )), DataType::Selected),
                                                metadata: Default::default(),
                                            },
                                        ],
                                        metadata: MenuItemMetadata {
                                            name: "".to_string(),
                                            icon_pos: Position::Left,
                                            height: 0,
                                            width: 0,
                                            color: Default::default(),
                                            description: "Show metrics for whole network ".to_string(),
                                            ..default()
                                        },
                                        option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                            PhantomData::<Node>::default(),
                                            DataType::Selected
                                        )),
                                    },
                                    parent: MenuItemMetadata {
                                        icon: "".to_string(),
                                        name: "".to_string(),
                                        icon_pos: Position::Left,
                                        height: 0,
                                        width: 0,
                                        color: Default::default(),
                                        ..default()
                                    },
                                    config_option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                        PhantomData::<Node>::default(),
                                        DataType::Selected
                                    )),
                                },
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    name: "Layer Metrics".to_string(),
                                    icon_pos: Position::Left,
                                    height: 100,
                                    width: 100,
                                    color: Default::default(),
                                    description: "Options for metrics for layers.".to_string(),
                                    ..default()
                                },
                            },

                        ],
                        metadata: MenuItemMetadata {
                            icon: "".to_string(),
                            name: "Network Metrics".to_string(),
                            icon_pos: Position::Left,
                            height: 0,
                            width: 0,
                            color: Default::default(),
                            description: "Menu options for metrics.".to_string(),
                            ..default()
                        },
                        option: ConfigurationOptionEnum::Metrics(ConfigurationOption::Metrics(
                            PhantomData::<Metric>::default(),
                            DataType::Selected
                        )),
                    },
                ],
            }
        }
    }
}

