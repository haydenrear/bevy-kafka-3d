use std::marker::PhantomData;
use bevy::ecs::component::TableStorage;
use bevy::prelude::{Component, Resource};
use bevy::ui::{Size, Val};
use bevy::utils::default;
use crate::menu::{ConfigurationOption, DataType, MenuData, MenuOption, MenuInputType, MenuItemMetadata, MenuOptionType, Position, ConfigurationOptionEnum, Menu};
use crate::metrics::Metric;
use crate::network::{Layer, Network, Node};

#[derive(Resource)]
pub struct MenuResource {
    pub(crate) menu_data: MenuData,
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
                                                                        ConfigurationOption::Concavity(
                                                                            PhantomData::<Network>::default(),
                                                                            DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(
                                                                        ConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NetworkMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                                        ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                            },

                                            // layer
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Selected
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
                                            },

                                            // node
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Selected
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
                                            ConfigurationOption::Metrics(
                                                PhantomData::<Metric>::default(),
                                                DataType::Selected,
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
                                        ConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected
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
                                                                        ConfigurationOption::Concavity(
                                                                            PhantomData::<Network>::default(),
                                                                            DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NetworkMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                                        ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                            },

                                            // layer
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Selected
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
                                            },

                                            // node
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Selected
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
                                            ConfigurationOption::Metrics(
                                                PhantomData::<Metric>::default(),
                                                DataType::Selected,
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
                                        ConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected
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
                            ConfigurationOption::Menu(
                                PhantomData::<Menu>::default(),
                                DataType::Selected
                            )
                        ),
                    },
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
                                                                        ConfigurationOption::Concavity(
                                                                            PhantomData::<Network>::default(),
                                                                            DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NetworkMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                                        ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                            },

                                            // layer
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Selected
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
                                            },

                                            // node
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Selected
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
                                            ConfigurationOption::Metrics(
                                                PhantomData::<Metric>::default(),
                                                DataType::Selected,
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
                                        ConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected
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
                                                                        ConfigurationOption::Concavity(
                                                                            PhantomData::<Network>::default(),
                                                                            DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NetworkVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Network>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NetworkMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                                        ConfigurationOption::Metrics(
                                                            PhantomData::<Network>::default(),
                                                            DataType::Selected
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
                                            },

                                            // layer
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::LayerVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Layer>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Layer>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::LayerMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Layer>::default(),
                                                        DataType::Selected
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
                                            },

                                            // node
                                            MenuOption {
                                                data_type: MenuOptionType::SubMenu {
                                                    sub_menu: MenuInputType::Dropdown {
                                                        options: vec![
                                                            // concavity
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeConcavity(ConfigurationOption::Concavity(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                            },
                                                            // variance
                                                            MenuOption {
                                                                data_type: MenuOptionType::Primitive(
                                                                    ConfigurationOptionEnum::NodeVariance(ConfigurationOption::Variance(
                                                                        PhantomData::<Node>::default(),
                                                                        DataType::Selected
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
                                                        option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                            PhantomData::<Node>::default(),
                                                            DataType::Selected
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
                                                    config_option: ConfigurationOptionEnum::NodeMetrics(ConfigurationOption::Metrics(
                                                        PhantomData::<Node>::default(),
                                                        DataType::Selected
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
                                            ConfigurationOption::Metrics(
                                                PhantomData::<Metric>::default(),
                                                DataType::Selected,
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
                                        ConfigurationOption::Menu(
                                            PhantomData::<Menu>::default(),
                                            DataType::Selected
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
                            ConfigurationOption::Menu(
                                PhantomData::<Menu>::default(),
                                DataType::Selected
                            )
                        ),
                    },
                ],
            }
        }
    }
}

