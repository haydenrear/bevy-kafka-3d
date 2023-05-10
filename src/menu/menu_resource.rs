use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Component, Resource};
use bevy::ui::{Size, Val};
use bevy::utils::default;
use crate::graph::Graph;
use crate::graph::graph_plugin::GraphPlugin;
use crate::menu::{MetricsConfigurationOption, DataType, MenuData, MenuOption, MenuInputType, MenuItemMetadata, MenuOptionType, Position, ConfigurationOptionEnum, Menu, MenuType};
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

                    MenuInputType::CollapsableMenu {
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
                                                                            DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(
                                                                        MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Network Metric Options".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 6.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::NetworkMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected,
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
                                                            DataType::Selected,
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
                                                    ..default()
                                                },
                                                swing_out: true,
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
                                                                        DataType::Selected,
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
                                                                    id: 11.0,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Layer Metrics".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 7.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::LayerMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Selected,
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
                                                        DataType::Selected,
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
                                                    ..default()
                                                },
                                                swing_out: true,
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
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Node Metrics".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 8.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::NodeMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Selected,
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
                                                        DataType::Selected,
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
                                                    ..default()
                                                },
                                                swing_out: true,
                                            },

                                        ],

                                        metadata: MenuItemMetadata {
                                            icon: "".to_string(),
                                            name: "Network Metrics".to_string(),
                                            icon_pos: Position::Left,
                                            color: Default::default(),
                                            description: "Menu options for metrics.".to_string(),
                                            id: 2.0,
                                            ..default()
                                        },

                                        option: ConfigurationOptionEnum::Metrics(
                                            MetricsConfigurationOption::Metrics(
                                                PhantomData::<Metric<Network>>::default(),
                                                DataType::Selected,
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
                                    },
                                    config_option: ConfigurationOptionEnum::Menu(
                                        MetricsConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected,
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
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                },
                                swing_out: true,
                            },
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
                                                                            DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Network Metric Options".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 6.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::NetworkMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected,
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
                                                            DataType::Selected,
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
                                                    ..default()
                                                },
                                                swing_out: true,
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
                                                                        DataType::Selected,
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
                                                                    id: 11.0,
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Layer Metrics".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 7.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::LayerMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Selected,
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
                                                        DataType::Selected,
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
                                                    ..default()
                                                },
                                                swing_out: true,
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
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(MetricsConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected,
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
                                                                    ..default()
                                                                },
                                                                swing_out: true,
                                                            },
                                                        ],
                                                        metadata: MenuItemMetadata {
                                                            name: "Node Metrics".to_string(),
                                                            icon_pos: Position::Left,
                                                            id: 8.0,
                                                            color: Default::default(),
                                                            description: "Show metrics for whole network ".to_string(),
                                                            ..default()
                                                        },
                                                        option: ConfigurationOptionEnum::NodeMetrics(MetricsConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Selected,
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
                                                        DataType::Selected,
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
                                                    ..default()
                                                },
                                                swing_out: true,
                                            },

                                        ],

                                        metadata: MenuItemMetadata {
                                            icon: "".to_string(),
                                            name: "Network Metrics".to_string(),
                                            icon_pos: Position::Left,
                                            color: Default::default(),
                                            description: "Menu options for metrics.".to_string(),
                                            id: 2.0,
                                            ..default()
                                        },

                                        option: ConfigurationOptionEnum::Metrics(
                                            MetricsConfigurationOption::Metrics(
                                                PhantomData::<Metric<Network>>::default(),
                                                DataType::Selected,
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
                                    },
                                    config_option: ConfigurationOptionEnum::Menu(
                                        MetricsConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected,
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
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                },
                                swing_out: true,
                            }
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
                        },
                        option: ConfigurationOptionEnum::Menu(
                            MetricsConfigurationOption::Menu(
                                PhantomData::<Menu>::default(),
                                DataType::Selected,
                                MENU,
                                MenuType::Menu
                            )
                        ),
                    },

                    MenuInputType::CollapsableMenu {
                        options: vec![

                            MenuOption {
                                data_type: MenuOptionType::Primitive(
                                    ConfigurationOptionEnum::GraphingMenu(
                                        MetricsConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected,
                                            MENU,
                                            MenuType::Graph
                                        ))
                                ),
                                index: 0,
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    font: Default::default(),
                                    name: "Display Graph".to_string(),
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                },
                                swing_out: false,
                            },
                            MenuOption {
                                data_type: MenuOptionType::Primitive(
                                    ConfigurationOptionEnum::GraphingMenu(
                                        MetricsConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected,
                                            MENU,
                                            MenuType::Network
                                        ))
                                ),
                                index: 0,
                                metadata: MenuItemMetadata {
                                    icon: "".to_string(),
                                    font: Default::default(),
                                    name: "Display Network".to_string(),
                                    icon_pos: Default::default(),
                                    size: None,
                                    color: Default::default(),
                                    description: "".to_string(),
                                    id: 101.0,
                                },
                                swing_out: false,
                            },

                        ],
                        metadata: MenuItemMetadata {
                            icon: "".to_string(),
                            font: Default::default(),
                            name: "Display".to_string(),
                            icon_pos: Default::default(),
                            size: None,
                            color: Default::default(),
                            description: "".to_string(),
                            id: 0.0,
                        },
                        option: ConfigurationOptionEnum::Menu(
                            MetricsConfigurationOption::Menu(
                                PhantomData::<Menu>::default(),
                                DataType::Selected,
                                MENU,
                                MenuType::Menu
                            )
                        ),
                    },
                ],
            }
        }
    }
}

