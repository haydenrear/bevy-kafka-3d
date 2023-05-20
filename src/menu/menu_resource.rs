use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Component, Resource};
use bevy::ui::{Size, Val};
use bevy::utils::default;
use crate::graph::GraphParent;
use crate::graph::graph_plugin::GraphPlugin;
use crate::menu::{MetricsConfigurationOption, DataType, MenuData, MenuOption, MenuInputType, MenuItemMetadata, MenuOptionType, Position, ConfigurationOptionEnum, Menu, MenuType, SliderData, MenuOptionInputType, SelectableType};
use crate::menu::config_menu_event::config_menu_event_plugin::ConfigMenuEventPlugin;
use crate::metrics::network_metrics::Metric;
use crate::network::{Layer, Network, Node};

#[derive(Resource)]
pub struct MenuResource {
    pub(crate) menu_data: MenuData,
}

pub const METRICS: &'static str = "Metrics";
pub const MENU: &'static str = "Menu";
pub const VARIANCE: &'static str = "Variance";
pub const CONCAVITY: &'static str = "Concavity";

impl Default for MenuResource {
    fn default() -> Self {
        Self {
            menu_data: MenuData {
                sub_menus: vec![],
                selectables: vec![

                    MenuInputType::CollapsableMenuInputType {
                        options: vec![
                            MenuOption {
                                data_type: MenuOptionType::SubMenu {
                                    sub_menu: MenuInputType::Dropdown {
                                        options: vec![

                                            // network
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkConcavity(
                                                                        MetricsConfigurationOption::Concavity(
                                                                            PhantomData::<Network>::default(),
                                                                            DataType::Deselected,
                                                                            CONCAVITY
                                                                        ))),
                                                                index: 0,
                                                                metadata: MenuItemMetadata {
                                                                    icon: "".to_string(),
                                                                    font: Default::default(),
                                                                    name: "Network Concavity".to_string(),
                                                                    icon_pos: Default::default(),
                                                                    color: Default::default(),
                                                                    description: "".to_string(),
                                                                    id: 9.0,
                                                                    swing_out: true,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                                ui_option_type: MenuOptionInputType::Activated,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(
                                                                        MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Deselected,
                                                                            VARIANCE
                                                                    ))),
                                                                index: 1,
                                                                metadata: MenuItemMetadata {
                                                                    icon: "".to_string(),
                                                                    font: Default::default(),
                                                                    name: "Network Variance".to_string(),
                                                                    icon_pos: Default::default(),
                                                                    color: Default::default(),
                                                                    description: "".to_string(),
                                                                    id: 10.0,
                                                                    swing_out: true,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                                ui_option_type: MenuOptionInputType::Activated,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Network Metric Options".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 6.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            swing_out: true,
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::NetworkMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Deselected,
                                                            METRICS
                                                        )),
                                                    },
                                                    parent: MenuItemMetadata {
                                                        icon: "".to_string(),
                                                        name: "".to_string(),
                                                        id: 3.0,
                                                        icon_pos: Position::Left,
                                                        color: Default::default(),
                                                        ..default()
                                                    },
                                                    config_option: ConfigurationOptionEnum::NetworkMetrics(
                                                        MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Deselected,
                                                            METRICS
                                                        )),
                                                },
                                                index: 0,
                                                metadata: MenuItemMetadata {
                                                    id: 2.0,
                                                    icon: "".to_string(),
                                                    name: "Layer Metrics".to_string(),
                                                    size: Some(Size::new(Val::Percent(100.0), Val::Px(30.0))),
                                                    icon_pos: Position::Left,
                                                    color: Default::default(),
                                                    description: "Options for metrics for layers.".to_string(),
                                                    swing_out: true,
                                                    ..default()
                                                },
                                                swing_out: true,
                                                ui_option_type: MenuOptionInputType::SubMenu,
                                            },

                                            // layer
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerConcavity(MetricsConfigurationOption::Concavity(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Deselected,
                                                                        CONCAVITY
                                                                    ))),
                                                                index: 0,
                                                                metadata: MenuItemMetadata {
                                                                    icon: "".to_string(),
                                                                    font: Default::default(),
                                                                    name: "Layer Concavity".to_string(),
                                                                    icon_pos: Default::default(),
                                                                    color: Default::default(),
                                                                    description: "".to_string(),
                                                                    swing_out: true,
                                                                    id: 11.0,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                                ui_option_type: MenuOptionInputType::Activated,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Deselected,
                                                                        VARIANCE
                                                                    ))),
                                                                index: 1,
                                                                metadata: MenuItemMetadata {
                                                                    icon: "".to_string(),
                                                                    font: Default::default(),
                                                                    name: "Layer Variance".to_string(),
                                                                    icon_pos: Default::default(),
                                                                    color: Default::default(),
                                                                    description: "".to_string(),
                                                                    id: 12.0,
                                                                    swing_out: true,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                                ui_option_type: MenuOptionInputType::Activated,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Layer Metrics".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 7.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            swing_out: true,
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::LayerMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Deselected,
                                                                METRICS
                                                        )),
                                                    },
                                                    parent: MenuItemMetadata {
                                                        icon: "".to_string(),
                                                        id: 4.0,
                                                        name: "".to_string(),
                                                        icon_pos: Position::Left,
                                                        color: Default::default(),
                                                        ..default()
                                                    },
                                                    config_option: ConfigurationOptionEnum::LayerMetrics(MetricsConfigurationOption::Metrics(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Deselected,
                                                        METRICS
                                                    )),
                                                },
                                                index: 0,
                                                metadata: MenuItemMetadata {
                                                    icon: "".to_string(),
                                                    name: "Layer Metrics".to_string(),
                                                    icon_pos: Position::Left,
                                                    id: 3.0,
                                                    color: Default::default(),
                                                    description: "Options for metrics for layers.".to_string(),
                                                    swing_out: true,
                                                    ..default()
                                                },
                                                swing_out: true,
                                                ui_option_type: MenuOptionInputType::SubMenu,
                                            },

                                            // node
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeConcavity(MetricsConfigurationOption::Concavity(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Deselected,
                                                                        CONCAVITY
                                                                    ))),
                                                                index: 0,
                                                                metadata: MenuItemMetadata {
                                                                    icon: "".to_string(),
                                                                    font: Default::default(),
                                                                    name: "Node Concavity".to_string(),
                                                                    icon_pos: Default::default(),
                                                                    color: Default::default(),
                                                                    description: "".to_string(),
                                                                    id: 13.0,
                                                                    swing_out: true,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                                ui_option_type: MenuOptionInputType::Activated,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Deselected,
                                                                        VARIANCE
                                                                    ))),
                                                                index: 1,
                                                                metadata: MenuItemMetadata {
                                                                    icon: "".to_string(),
                                                                    font: Default::default(),
                                                                    name: "Node Variance".to_string(),
                                                                    icon_pos: Default::default(),
                                                                    color: Default::default(),
                                                                    description: "".to_string(),
                                                                    id: 14.0,
                                                                    swing_out: true,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                                ui_option_type: MenuOptionInputType::Activated,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Node Metrics".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 8.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            swing_out: true,
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::NodeMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Deselected,
                                                            METRICS
                                                        )),
                                                    },
                                                    parent: MenuItemMetadata {
                                                        icon: "".to_string(),
                                                        name: "Node Metrics".to_string(),
                                                        icon_pos: Position::Left,
                                                        id: 5.0,
                                                        color: Default::default(),
                                                        ..default()
                                                    },
                                                    config_option: ConfigurationOptionEnum::NodeMetrics(MetricsConfigurationOption::Metrics(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Deselected,
                                                        METRICS
                                                    )),
                                                },
                                                index: 0,
                                                metadata: MenuItemMetadata {
                                                    icon: "".to_string(),
                                                    name: "Layer Metrics".to_string(),
                                                    icon_pos: Position::Left,
                                                    id: 4.0,
                                                    color: Default::default(),
                                                    description: "Options for metrics for layers.".to_string(),
                                                    swing_out: true,
                                                    ..default()
                                                },
                                                swing_out: true,
                                                ui_option_type: MenuOptionInputType::SubMenu,
                                            },

                                        ],

                                        metadata: MenuItemMetadata {
                                            icon: "".to_string(),
                                            name: "Network Metrics".to_string(),
                                            icon_pos: Position::Left,
                                            color: Default::default(),
                                            description: "Menu options for metrics.".to_string(),
                                            id: 2.0,
                                            swing_out: true,
                                            ..default()
                                        },

                                        option: ConfigurationOptionEnum::Metrics(
                                            MetricsConfigurationOption::Metrics(
                                                PhantomData::<Metric<Network>>::default(),
                                                DataType::Deselected,
                                                METRICS
                                            )
                                        ),
                                    },
                                    parent: MenuItemMetadata {
                                        icon: "".to_string(),
                                        font: Default::default(),
                                        name: "".to_string(),
                                        icon_pos: Default::default(),
                                        size: None,
                                        color: Default::default(),
                                        description: "".to_string(),
                                        id: 101.0,
                                        ..default()
                                    },
                                    config_option: ConfigurationOptionEnum::Menu(
                                        MetricsConfigurationOption::GraphMenu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Deselected,
                                                MENU,
                                            MenuType::Menu
                                        )
                                    ),
                                },
                                index: 0,
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    font: Default::default(),
                                    name: "Metrics".to_string(),
                                    initial_value: "".to_string(),
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                    swing_out: false,
                                },
                                swing_out: true,
                                ui_option_type: MenuOptionInputType::SubMenu,
                            },
                        ],
                        metadata: MenuItemMetadata {
                            icon: "".to_string(),
                            font: Default::default(),
                            name: "Metrics".to_string(),
                            icon_pos: Default::default(),
                            size: None,
                            color: Default::default(),
                            description: "".to_string(),
                            id: 0.0,
                            swing_out: true,
                            ..default()
                        },
                        option: ConfigurationOptionEnum::Menu(
                            MetricsConfigurationOption::GraphMenu(
                                PhantomData::<Menu>::default(),
                                DataType::Deselected,
                                MENU,
                                MenuType::Menu
                            )
                        ),
                    },

                    MenuInputType::CollapsableMenuInputType {
                        options: vec![

                            MenuOption {
                                data_type: MenuOptionType::Primitive(
                                    ConfigurationOptionEnum::Menu(
                                        MetricsConfigurationOption::GraphMenu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Deselected,
                                            MENU,
                                            MenuType::Graph
                                        ))
                                ),
                                index: 0,
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    font: Default::default(),
                                    name: "Display Graph".to_string(),
                                    initial_value: "".to_string(),
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                    swing_out: false,
                                },
                                swing_out: false,
                                ui_option_type: MenuOptionInputType::Activated,
                            },
                            MenuOption {
                                data_type: MenuOptionType::Primitive(
                                    ConfigurationOptionEnum::Menu(
                                        MetricsConfigurationOption::NetworkMenu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Deselected,
                                            MENU,
                                            MenuType::Network
                                        ))
                                ),
                                index: 0,
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    font: Default::default(),
                                    name: "Display Network".to_string(),
                                    initial_value: "".to_string(),
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                    swing_out: true,
                                },
                                swing_out: false,
                                ui_option_type: MenuOptionInputType::Activated,
                            },

                        ],
                        metadata: MenuItemMetadata {
                            icon: "".to_string(),
                            font: Default::default(),
                            name: "Display".to_string(),
                            initial_value: "".to_string(),
                            icon_pos: Default::default(),
                            size: None,
                            color: Default::default(),
                            description: "".to_string(),
                            id: 0.0,
                            swing_out: false,
                        },
                        option: ConfigurationOptionEnum::Menu(
                            MetricsConfigurationOption::GraphMenu(
                                PhantomData::<Menu>::default(),
                                DataType::Deselected,
                                MENU,
                                MenuType::Menu
                            )
                        ),
                    },

                    // MenuInputType::Slider {
                    //     metadata: Default::default(),
                    //     slider_data: SliderData {
                    //         start: 0,
                    //         end: 100
                    //     },
                    //     option: Default::default(),
                    // },

                    // MenuInputType::Radial {
                    //     options: vec![
                    //         MenuOption {
                    //             data_type: MenuOptionType::Primitive(ConfigurationOptionEnum::LayerVariance(MetricsConfigurationOption::Variance(
                    //                 PhantomData::<Layer>::default(),
                    //                 DataType::Selected,
                    //                     "name 1"
                    //             ))),
                    //             index: 0,
                    //             metadata: MenuItemMetadata {
                    //                 name: "test".to_string(),
                    //                 ..default()
                    //             },
                    //             swing_out: false,
                    //             ui_option_type: MenuOptionInputType::Activated,
                    //         },
                    //         MenuOption {
                    //             data_type: MenuOptionType::Primitive(ConfigurationOptionEnum::LayerVariance(MetricsConfigurationOption::Variance(
                    //                 PhantomData::<Layer>::default(),
                    //                 DataType::Selected,
                    //                 "name 2"
                    //             ))),
                    //             index: 0,
                    //             metadata: MenuItemMetadata {
                    //                 name: "test".to_string(),
                    //                 ..default()
                    //             },
                    //             swing_out: false,
                    //             ui_option_type: MenuOptionInputType::Activated,
                    //         }
                    //     ],
                    //     metadata: MenuItemMetadata {
                    //         name: "test_two".to_string(),
                    //         ..default()
                    //     },
                    //     option: Default::default(),
                    // }
                    //

                ],
            }
        }
    }
}

