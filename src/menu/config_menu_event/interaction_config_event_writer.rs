use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use crate::event::event_actions::RetrieveState;
use crate::event::event_descriptor::EventDescriptor;
use crate::event::event_propagation::PropagateComponentEvent;
use crate::event::event_state::Context;
use crate::menu::{MetricsConfigurationOption, ConfigurationOptionComponent, ConfigurationOptionEnum, DataType, MenuType};
use crate::menu::config_menu_event::config_event::{ConfigurationOptionChange, ConfigurationOptionEventArgs};
use crate::menu::menu_resource::MENU;
use crate::network::{Network, Node};

#[derive(Default, Resource, Debug)]
pub struct ConfigOptionActionStateRetriever;

#[derive(Default, Resource,Debug)]
pub struct ConfigOptionContext {
    pub(crate) graph_entity: Option<Entity>,
    pub(crate) network_entity: Option<Entity>
}

impl Context for ConfigOptionContext{}

impl <T: Component + Send + Sync + Default + Clone + Debug + 'static> RetrieveState<
    ConfigurationOptionEventArgs<T>,
    DataType,
    MetricsConfigurationOption<T>,
    ConfigOptionContext,
    (Entity, &MetricsConfigurationOption<T>),
    (Entity, &Parent, &MetricsConfigurationOption<T>),
    (Entity, &Children, &MetricsConfigurationOption<T>),
    (With<MetricsConfigurationOption<T>>),
    (With<Parent>, With<MetricsConfigurationOption<T>>),
    (With<Children>, With<MetricsConfigurationOption<T>>),
>
for ConfigOptionActionStateRetriever
{
    fn create_event(
        commands: &mut Commands,
        entity: Entity,
        mut context: &mut ResMut<ConfigOptionContext>,
        self_query: &Query<
            (Entity, &MetricsConfigurationOption<T>),
            (With<MetricsConfigurationOption<T>>)
        >,
        with_parent_query: &Query<
            (Entity, &Parent, &MetricsConfigurationOption<T>),
            (With<Parent>, With<MetricsConfigurationOption<T>>)
        >,
        with_child_query: &Query<
            (Entity, &Children, &MetricsConfigurationOption<T>),
            (With<Children>, With<MetricsConfigurationOption<T>>)
        >
    ) -> (
        Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
        Vec<PropagateComponentEvent>
    )
    {
        let mut event_descriptors = vec![];
        let mut propagate_events = vec![];

        self_query.iter()
            .for_each(|(entity, config)| {
                info!("Found {:?}", config);
                match config {
                    MetricsConfigurationOption::Menu(_, data_type, _, MenuType::Graph) => {
                        if let DataType::Selected = data_type {
                            create_add_events(
                                &mut event_descriptors,
                                &mut propagate_events,
                                entity,
                                config,
                                DataType::Deselected,
                                MenuType::Graph,
                                Visibility::Hidden,
                                context.network_entity
                            );
                        } else if let DataType::Deselected = data_type {
                            create_add_events(&mut event_descriptors,
                                              &mut propagate_events,
                                              entity,
                                              config,
                                              DataType::Selected,
                                              MenuType::Graph,
                                              Visibility::Visible,
                                              context.network_entity
                            );
                        }
                    }
                    MetricsConfigurationOption::Menu(_, data_type, _, MenuType::Network) => {
                        if let DataType::Selected = data_type {
                            create_add_events(&mut event_descriptors,
                                              &mut propagate_events,
                                              entity,
                                              config,
                                              DataType::Deselected,
                                              MenuType::Network,
                                              Visibility::Hidden,
                                              context.graph_entity
                            );
                        } else if let DataType::Deselected = data_type {
                            create_add_events(&mut event_descriptors,
                                              &mut propagate_events,
                                              entity,
                                              config,
                                              DataType::Selected,
                                              MenuType::Network,
                                              Visibility::Visible,
                                              context.graph_entity
                            );
                        }
                    }
                    _ => {}
                }
            });
        (event_descriptors, propagate_events)
    }
}

fn create_add_events<T>(
    mut event_descriptors: &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
    mut propagate_events: &mut Vec<PropagateComponentEvent>,
    entity: Entity,
    config: &MetricsConfigurationOption<T>,
    data_type: DataType,
    menu_type: MenuType,
    visible: Visibility,
    other_entity: Option<Entity>
)
    where
        T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    let event_descriptor = create_graph_menu_event(entity, data_type, menu_type, config.clone());
    let event = create_event_tuple(
        event_descriptor,
        &|entity| Some(PropagateComponentEvent::ChangeVisible(entity, visible)),
        other_entity,
    );
    add_to_events(&mut event_descriptors, &mut propagate_events, event);
}

fn add_to_events<T>(mut event_descriptors:
                 &mut Vec<EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>>,
                 mut propagate_events: &mut Vec<PropagateComponentEvent>,
                 event: Vec<(EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>, Vec<PropagateComponentEvent>)>
)
    where
        T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    event.into_iter().for_each(|(event, prop)| {
        event_descriptors.push(event);
        prop.into_iter().for_each(|prop| propagate_events.push(prop));
    });
}

fn create_event_tuple<T>(
    event_descriptor: EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>,
    option: &dyn Fn(Entity) -> Option<PropagateComponentEvent>,
    entity: Option<Entity>
) -> Vec<(EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>, Vec<PropagateComponentEvent>)>
where
    T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    let mut event_tuple = (event_descriptor, vec![]);
    if entity.is_some() {
        event_tuple.1 = option(entity.unwrap()).map(|p| vec![p]).or(Some(vec![])).unwrap();
    }
    vec![event_tuple]
}

fn create_graph_menu_event<T>(
    entity: Entity,
    data_type: DataType,
    menu_type: MenuType,
    config_option: MetricsConfigurationOption<T>
) -> EventDescriptor<DataType, ConfigurationOptionEventArgs<T>, MetricsConfigurationOption<T>>
where
    T: Component + Send + Sync + Default + Clone + Debug + 'static
{
    let mut config_option_change = HashMap::new();
    config_option_change.insert(entity, MetricsConfigurationOption::Menu(
        PhantomData::<T>::default(), data_type.clone(),
        MENU, menu_type
    ));
    EventDescriptor {
        component: Default::default(),
        event_data: data_type,
        event_args: ConfigurationOptionEventArgs::Event(ConfigurationOptionChange {
            config_option: config_option_change,
        }, entity),
    }
}